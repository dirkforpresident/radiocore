-- RadioCore — Standard Repeater-Logik
--
-- Verfuegbare Funktionen (von Rust bereitgestellt):
--   log(msg)              — Nachricht ins Log
--   ptt_on()              — Sender einschalten
--   ptt_off()             — Sender ausschalten
--   announce(text)        — Text per TTS ansagen
--   play_roger_beep()     — Roger-Beep abspielen
--   start_timeout_timer(s) — Timeout-Timer starten
--   start_tail_timer(s)   — Nachlauf-Timer starten
--   link_talkgroup(name)  — Talkgroup verbinden
--   unlink_all()          — Alle Verbindungen trennen

function on_squelch_open()
    log("Squelch open")
    ptt_on()
    start_timeout_timer(config.timeout_seconds)
end

function on_squelch_close()
    log("Squelch close")
    play_roger_beep()
    start_tail_timer(config.tail_seconds)
end

function on_timeout()
    log("Timeout!")
    announce("Timeout, bitte kuerzer tasten")
    ptt_off()
end

function on_dtmf(code)
    log("DTMF: " .. code)

    if code == "*1#" then
        link_talkgroup("TG-Hamburg")
        announce("Verbunden mit Talkgroup Hamburg")
    elseif code == "*2#" then
        link_talkgroup("TG-Deutschland")
        announce("Verbunden mit Talkgroup Deutschland")
    elseif code == "*0#" then
        unlink_all()
        announce("Alle Verbindungen getrennt")
    elseif code == "*9#" then
        announce("RadioCore Version 0.1")
    end
end

function on_id_timer()
    log("CW ID: " .. config.callsign)
end
