//! Lua logic engine — loads and executes repeater logic scripts.
//! Supports hot-reload: scripts can be changed without restarting.

pub mod dtmf;

use anyhow::Result;
use mlua::prelude::*;
use tracing::{info, warn, error};

use crate::RepeaterConfig;

pub struct Engine {
    lua: Lua,
    script_path: String,
}

impl Engine {
    pub fn new(config: &RepeaterConfig) -> Result<Self> {
        let lua = Lua::new();
        let globals = lua.globals();

        // Logging
        globals.set("log", lua.create_function(|_, msg: String| {
            info!("[Lua] {}", msg);
            Ok(())
        })?)?;

        // PTT control
        globals.set("ptt_on", lua.create_function(|_, ()| {
            info!("[PTT] ON");
            Ok(())
        })?)?;

        globals.set("ptt_off", lua.create_function(|_, ()| {
            info!("[PTT] OFF");
            Ok(())
        })?)?;

        // Announcements
        globals.set("announce", lua.create_function(|_, text: String| {
            info!("[TTS] {}", text);
            Ok(())
        })?)?;

        globals.set("play_roger_beep", lua.create_function(|_, ()| {
            info!("[Audio] Roger beep");
            Ok(())
        })?)?;

        // Timers
        globals.set("start_timeout_timer", lua.create_function(|_, secs: u32| {
            info!("[Timer] Timeout: {}s", secs);
            Ok(())
        })?)?;

        globals.set("start_tail_timer", lua.create_function(|_, secs: u32| {
            info!("[Timer] Tail: {}s", secs);
            Ok(())
        })?)?;

        // Linking
        globals.set("link_talkgroup", lua.create_function(|_, tg: String| {
            info!("[Link] {}", tg);
            Ok(())
        })?)?;

        globals.set("unlink_all", lua.create_function(|_, ()| {
            info!("[Link] All disconnected");
            Ok(())
        })?)?;

        // Config table for Lua
        let config_table = lua.create_table()?;
        config_table.set("callsign", config.callsign.as_str())?;
        config_table.set("timeout_seconds", config.repeater.timeout_seconds)?;
        config_table.set("tail_seconds", config.repeater.tail_seconds)?;
        if let Some(tone) = config.repeater.ctcss_tone {
            config_table.set("ctcss_tone", tone)?;
        }
        globals.set("config", config_table)?;

        let script_path = config.repeater.logic_script.clone();
        let engine = Engine { lua, script_path };
        engine.reload()?;

        Ok(engine)
    }

    pub fn reload(&self) -> Result<()> {
        match std::fs::read_to_string(&self.script_path) {
            Ok(script) => {
                self.lua.load(&script).exec()?;
                info!("Logic script loaded: {}", self.script_path);
                Ok(())
            }
            Err(e) => {
                warn!("Could not load {}: {}", self.script_path, e);
                Ok(())
            }
        }
    }

    pub fn call_event(&self, event: &str, args: impl IntoLuaMulti) -> Result<()> {
        let globals = self.lua.globals();
        if let Ok(func) = globals.get::<LuaFunction>(event) {
            if let Err(e) = func.call::<()>(args) {
                error!("Lua error in {}: {}", event, e);
            }
        }
        Ok(())
    }

    pub fn on_squelch_open(&self) -> Result<()> { self.call_event("on_squelch_open", ()) }
    pub fn on_squelch_close(&self) -> Result<()> { self.call_event("on_squelch_close", ()) }
    pub fn on_dtmf(&self, code: &str) -> Result<()> { self.call_event("on_dtmf", code.to_string()) }
    pub fn on_timeout(&self) -> Result<()> { self.call_event("on_timeout", ()) }
}
