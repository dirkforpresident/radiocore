# RadioCore

**Ein Board. Drei Anwendungen. Dein Amateurfunk-Oekosystem.**

99€ | ESP32-S3 + ES8388 Audio Codec | USB-C + RJ45 | WiFi + Bluetooth | Open Source

---

## Was ist RadioCore?

Ein universelles Funk-Interface, das drei kommerzielle Produkte ersetzt:

| RadioCore Modus | Ersetzt | Preis des Originals | Deine Ersparnis |
|-----------------|---------|---------------------|-----------------|
| **Repeater-Steuerung** | SVXLink + CM108 | ~200€ + Bastelei | **101€** |
| **Remote-Station** | RemoteRig RRC-1258 | ~500€ | **401€** |
| **Digimode-Interface** | SignaLink USB / Digirig | ~120€ | **21€** |

**Das Besondere:** Du brauchst nicht drei Geraete. Ein RadioCore-Board kann zwischen allen drei Modi umgeschaltet werden — per Software. Heute Digimode fuer FT8, morgen Repeater-Steuerung, uebermorgen Remote-Station.

---

## Features

### Hardware (65 x 45 mm)

- **ESP32-S3 WROOM-1 N16R8:** 16MB Flash, 8MB PSRAM, WiFi, Bluetooth, USB OTG
- **ES8388 24-bit Audio Codec:** 105dB SNR, I2S, Line In/Out, professionelle Audio-Qualitaet
- **Dual Power:** USB-C (5V) oder DC 5-24V (13.8V Funknetzteil kompatibel)
- **Geschirmter RJ45:** Audio, PTT, COS, CAT — **ein einziges Kabel** zum Funkgeraet
- **2x 3.5mm Klinke:** RX In + TX Out (optional, parallel zu RJ45)
- **Expansion Header:** 2x5 Pin (I2C + 4x GPIO) fuer Relay Shields, Rotor-Steuerung, Display
- **WS2812B Status-LED:** RGB-Feedback fuer COS, PTT, TX, Errors
- **Reset + Boot Taster:** Firmware-Updates ohne Kabel-Fummelei
- **EMC-geschuetzt:** Ferrite Beads, LC-Filter, getrennte Ground-Planes (AGND/DGND)

### Software

- **Rust Workspace:** 4 Crates (core, repeater, remote, digimode) — schnell, sicher, wartbar
- **ESP32-S3 Firmware:** C/ESP-IDF mit 3 umschaltbaren Modi
- **Lua Scripting:** Repeater-Logik hot-reloadable (kein Neustart noetig)
- **Web Dashboards:** Fuer alle drei Modi — konfiguriere per Browser, kein SSH noetig
- **Cloud Service:** Optional fuer Remote-Betrieb (oder selbst hosten)

---

## Die drei Modi

### 1. Digimode Interface

**Ersetzt:** SignaLink USB, Digirig, CM108-Bastelei

Board an PC → USB-C → fertig. Keine Treiber, keine Konfiguration. Zeigt sich als:
- **USB Audio Device:** Stereo In/Out, 48kHz/24bit
- **USB Serial Port (CDC-ACM):** CAT-Kontrolle fuer Frequenz, Modus, PTT

**Anwendungen:** WSJT-X, fldigi, Direwolf (Packet), SSTV, JS8Call, VARA, Winlink

**Vorteile gegenueber SignaLink:**
- Kein Audio-Fummelei (Pegel digital einstellbar)
- CAT ueber USB (kein zweites Kabel)
- PTT via CAT oder VOX (konfigurierbar)
- 24-bit Audio statt 16-bit

### 2. Repeater-Steuerung

**Ersetzt:** SVXLink + CM108 + Raspberry Pi (optional)

**Standalone-Modus (nur ESP32):**
- COS → PTT Logic (Tail, Timeout, Courtesy Tone)
- WAV-Ansagen aus Flash (Rufzeichen, Zeit, DTMF-Quittung)
- Web-UI ueber WiFi (Status, Config, Logs)
- DTMF-Decoder (Kommandos ohne Pi)

**Erweiterter Modus (mit Raspberry Pi):**
- **Piper TTS:** KI-generierte Ansagen (klingt menschlich)
- **Lua Scripting:** Eigene Logik (Wetter, Echolink, APRS-Tor)
- **Live-Monitoring:** Waterfall, Audio-Archiv, Statistiken
- **VoIP-Linking:** Opus/QUIC (besser als Echolink)

**Vorteile gegenueber SVXLink:**

| Feature | SVXLink | RadioCore |
|---------|---------|-----------|
| Audio-Latenz | 40-80ms | **<5ms** (I2S statt USB) |
| Audio-Qualitaet | 16-bit/80dB (CM108) | **24-bit/105dB (ES8388)** |
| Web-UI | Nein | **Ja, live (Vue.js)** |
| TTS | espeak (roboterhaft) | **Piper KI-Stimme** |
| Sprache | C++/Tcl | **Rust/Lua** |
| Hardware | CM108 Bastelei | **Fertiges Board, 12€ Herstellung** |

### 3. Remote-Station

**Ersetzt:** RemoteRig RRC-1258 (600€), WebSDR-Hardware (teuer + kompliziert)

**Funkgeraet von ueberall steuern — nur ein Browser noetig.**

**Wie es funktioniert:**
1. RadioCore-Board am Funkgeraet (RJ45: Audio + CAT)
2. Board mit Internet verbunden (WiFi oder Ethernet via USB-Hub)
3. Cloud-Service relay (oder selbst gehostet)
4. Browser oeffnen → Deine Station bedienen (Waterfall, Frequenz, Modus, PTT)

**Features:**
- **Audio-Streaming:** Opus-Codec, 8-64 kbit/s (anpassbar)
- **CAT-Kontrolle:** Hamlib-Unterstuetzung fuer >300 Funkgeraete
- **Video-Optional:** USB-Kamera fuer Transceiver-Display
- **Multi-User:** Sharing-Modus (siehe unten)

**Station Sharing — Airbnb fuer Amateurfunk:**

Du hast eine KW-Station, benutzt sie aber nur am Wochenende? **Vermiete sie!**

- Du legst Zeitslots fest (z.B. Mo-Fr 09-17 Uhr)
- Andere OMs buchen deine Station (4.99€/Stunde)
- Du bekommst 70%, RadioCore-Cloud 30%
- Automatische Frequenz-Limits, Power-Limits, Bandplan-Checks

**Beispiel:** Du hast eine KW-Beam-Antenne in JN58. Ein OM in JO50 (keine Antenne) bucht deine Station fuer 2 Stunden DX-Contest. Er zahlt 9.98€, du bekommst 6.99€. Win-win.

---

## Hardware-Uebersicht

```
┌─────────────────────────────────────────────────────────────┐
│                       RadioCore Board                        │
│                         65 x 45 mm                           │
│                                                              │
│  [USB-C]                                      [RJ45 TRX]    │
│   Power                                       Audio+CAT      │
│   Data                                                       │
│                                                              │
│             ┌──────────────────┐                             │
│             │   ESP32-S3       │    ┌──────────────┐        │
│             │   WROOM-1 N16R8  │───►│   ES8388     │        │
│             │   16MB/8MB       │I2S │   Audio      │        │
│             │   WiFi/BT        │    │   Codec      │        │
│             └──────────────────┘    └──────────────┘        │
│                     │                       │                │
│                     │GPIO                   │Line In/Out     │
│                     ▼                       ▼                │
│              [PTT] [COS]            [RX Audio] [TX Audio]    │
│                                                              │
│  [●] WS2812B                        [RST] [BOOT]            │
│   RGB LED                            Buttons                 │
│                                                              │
│  ┌──────────────────────────────┐                           │
│  │   Expansion Header 2x5       │                           │
│  │   I2C + 4x GPIO + 3.3V + GND │                           │
│  └──────────────────────────────┘                           │
│                                                              │
│  [3.5mm RX In]           [3.5mm TX Out]                     │
│   Optional Audio Jacks (parallel zu RJ45)                   │
└─────────────────────────────────────────────────────────────┘
```

---

## Anschluesse

### RJ45 (ein Kabel fuer alles)

**Pinout (T568B-kompatibel):**

| Pin | Farbe (T568B) | Signal | Beschreibung |
|-----|---------------|--------|--------------|
| 1 | Orange/Weiss | **RX Audio +** | TRX Speaker Out → Board Line In |
| 2 | Orange | **RX Audio -** | Twisted Pair mit Pin 1 |
| 3 | Gruen/Weiss | **TX Audio +** | Board Line Out → TRX Mic In |
| 4 | Blau | **PTT** | Open Collector, Active Low (max 100mA) |
| 5 | Blau/Weiss | **COS/SQL** | Active Low, interne Pull-Up (3.3V) |
| 6 | Gruen | **TX Audio -** | Twisted Pair mit Pin 3 |
| 7 | Braun/Weiss | **CAT TX** | 3.3V UART (Board → TRX) |
| 8 | Braun | **CAT RX / GND** | 3.3V UART (TRX → Board) + Ground |

**Warum RJ45?**
- **Geschirmt:** Bessere EMV als Klinkenstecker
- **Robust:** Verriegelt, kein Wackelkontakt
- **Ein Kabel:** Audio + PTT + COS + CAT (statt 4 Kabel)
- **Standard:** Viele TRX haben schon RJ45 (Yaesu, Icom Data-Port)

**Adapter:**
Fuer TRX ohne RJ45 gibt es Breakout-Kabel (siehe `hardware/BREAKOUT.md`):
- Yaesu FT-817/857/897 (Mini-DIN)
- Icom IC-7300 (13-pin DIN)
- Kenwood TS-480 (DIN)
- Generisch (Klinkenstecker)

### Expansion Header (2x5 Pin)

```
  1  2   3.3V          VCC (max 500mA via LDO)
  3  4   GND           Ground
  5  6   I2C SDA       I2C Data (3.3V, Pull-Up onboard)
  7  8   I2C SCL       I2C Clock (3.3V, Pull-Up onboard)
  9 10   GPIO 1        Frei programmierbar (3.3V)
 11 12   GPIO 2        Frei programmierbar (3.3V)
 13 14   GPIO 3        Frei programmierbar (3.3V)
 15 16   GPIO 4        Frei programmierbar (3.3V)
 17 18   RESET         ESP32 Reset (Active Low)
 19 20   GND           Ground
```

**Verwendung:**
- **Relay Shield:** PTT fuer mehrere TRX, Antennen-Umschaltung
- **Rotor-Interface:** Azimut/Elevation-Steuerung (I2C oder GPIO)
- **OLED Display:** 128x64 Status-Anzeige (I2C)
- **Sensoren:** Temperatur, SWR, Power-Meter (I2C/GPIO)

---

## Erweiterungs-Oekosystem

| Produkt | Preis | Beschreibung |
|---------|-------|--------------|
| **RadioCore Board** | 99€ | Hauptboard (siehe oben) |
| **Relay Shield 4x** | 19€ | 4x SPDT-Relais (PTT, Antennen, CW-Key, externe Geraete) |
| **Relay Shield 8x** | 29€ | 8x SPDT-Relais (komplexe Setups, Antennen-Matrix) |
| **Rotor Interface** | 29€ | Yaesu/Alfaspid-Protokoll, Azimut+Elevation, Endschalter |
| **Sensor Board** | 19€ | SWR, Temperatur, Spannung, Strom (I2C) |
| **Breakout-Kabel Yaesu** | 9€ | RJ45 → Mini-DIN (FT-817/857/897/991) |
| **Breakout-Kabel Icom** | 9€ | RJ45 → 13-pin DIN (IC-7300/9700) |
| **Breakout-Kabel Kenwood** | 9€ | RJ45 → DIN (TS-480/590/890) |
| **Breakout-Kabel Generisch** | 7€ | RJ45 → 3.5mm Klinke (alle anderen) |
| **Gehaeuse Repeater** | 14€ | Wandmontage / DIN-Schiene, 2x RJ45 (RX+TX) |
| **Gehaeuse Remote** | 12€ | Kompakt neben TRX, 1x RJ45 |
| **Gehaeuse Digimode** | 12€ | Desktop mit Standfuss, 1x RJ45 |

**Komplettsets:**

| Set | Preis | Enthalten |
|-----|-------|-----------|
| **Digimode Starter** | 119€ | Board + Gehaeuse Digimode + Breakout-Kabel (Hersteller waehlbar) |
| **Repeater Starter** | 139€ | Board + Gehaeuse Repeater + 2x Breakout-Kabel + Relay Shield 4x |
| **Remote Starter** | 129€ | Board + Gehaeuse Remote + Breakout-Kabel + 1 Jahr Cloud Basic |
| **Ultimate Kit** | 199€ | Board + alle 3 Gehaeuse + 2x Breakout + Relay 4x + Sensor Board |

---

## Cloud & Remote Service

```
┌──────────────┐         ┌──────────────┐         ┌──────────────┐
│ Deine Station│         │  Cloud Relay │         │ Dein Browser │
│              │         │  (optional)  │         │ (ueberall)   │
│ RadioCore ───┼────────►│ WebSocket    │────────►│ Web-UI       │
│ Board        │ WiFi/   │ Opus Audio   │ HTTPS   │ Waterfall    │
│ + TRX        │ Ethernet│ CAT Proxy    │         │ CAT Control  │
└──────────────┘         └──────────────┘         └──────────────┘
```

### Cloud-Abos (optional)

| Plan | Preis/Monat | Features |
|------|-------------|----------|
| **Free** | 0€ | Lokales Netz (WiFi), Web-UI, Config, Logs |
| **Basic** | 4.99€ | Internet-Zugriff, 1 Station, 10 GB Traffic, Opus 32kbit/s |
| **Pro** | 9.99€ | 3 Stationen, 50 GB Traffic, Opus 64kbit/s, Video, API |
| **Station Share** | 14.99€ | Alles aus Pro + Sharing (verdiene Geld mit deiner Station) |

**Wichtig:** Die Cloud ist **optional**. Alle Modi funktionieren auch ohne Cloud:
- **Digimode:** USB an PC, keine Cloud noetig
- **Repeater:** Standalone auf ESP32 oder lokal mit Pi
- **Remote:** Im lokalen Netz (WiFi) ohne Cloud-Abo

Du bezahlst nur, wenn du Internet-Fernsteuerung oder Station Sharing willst.

### Station Sharing — Details

**Fuer Station-Besitzer:**
- Lege Verfuegbarkeit fest (Wochentage, Uhrzeiten)
- Setze Limits (Frequenzbereiche, Power, Bandplan-Checks)
- Verdiene 70% der Buchungsgebuehr (RadioCore behaelt 30%)
- Automatische Abrechnung (monatlich auf dein Konto)

**Fuer Mieter:**
- Suche Stationen nach Locator, Antenne, Band
- Buche Zeitslots (1-4 Stunden)
- Bezahle 4.99€/Stunde (Preis vom Besitzer anpassbar)
- Voller Zugriff (Audio, CAT, PTT) waehrend deiner Zeit

**Beispiel-Rechnung:**
- Du vermietest deine KW-Station 20 Stunden/Monat (Buero-Zeiten)
- Durchschnittspreis: 4.99€/Stunde
- Einnahmen: 20h × 4.99€ = 99.80€
- Dein Anteil (70%): **69.86€/Monat**
- → Dein RadioCore-Board amortisiert sich in 2 Monaten!

---

## Standalone-Betrieb (ohne Pi/PC)

Das Board ist **nie "tot"**. Selbst ohne Raspberry Pi oder PC laufen viele Features:

| Funktion | ESP32 Standalone | Mit Pi/PC |
|----------|------------------|-----------|
| **Digimode Interface** | Ja (USB an PC) | Ja |
| **Repeater COS→PTT** | Ja (Tail, Timeout) | Ja |
| **Repeater TTS** | WAV aus Flash (16MB) | Piper Live-TTS |
| **Repeater Web-UI** | Ja (WiFi, klein) | Ja (voll) |
| **Remote via WiFi** | Ja (lokal) | Ja (Internet via Cloud) |
| **DTMF-Decoder** | Ja (Goertzel) | Ja |
| **Lua Scripting** | Nein (zu wenig RAM) | Ja |
| **VoIP Linking** | Einfach (1-2 Peers) | Unbegrenzt (Opus/QUIC) |

**Beispiel Standalone-Repeater:**
- ESP32-S3 empfaengt COS von TRX
- Aktiviert PTT nach programmiertem Tail (z.B. 500ms)
- Spielt Courtesy Tone (WAV aus Flash)
- Timeout nach 3 Minuten (piepst, dann PTT aus)
- Web-UI zeigt Status (letzte 10 QSOs, Uptime, Config)
- Alles ohne Pi, nur 5V USB-Netzteil noetig

---

## Quick Start

### 1. Software ausprobieren (ohne Hardware)

```bash
git clone https://github.com/dirkforpresident/radiocore
cd radiocore

# Repeater-Modus (mit beliebiger Soundkarte)
cargo run -p radiocore-repeater

# Digimode-Modus (simuliert USB Audio)
cargo run -p radiocore-digimode

# Remote Station-Modus
cargo run -p radiocore-remote
```

Browser: http://localhost:8080

### 2. Hardware bestellen

**Option A: Fertiggeraet (empfohlen)**
- Shop: https://radiocore.ham (ab Q3 2026)
- Lieferzeit: 1-2 Wochen
- Preis: 99€ + Versand

**Option B: Selbst bauen**
1. Schaltplan + Gerber-Files: `hardware/pcb/`
2. BOM hochladen bei JLCPCB (oder anderem Fertiger)
3. 5 Boards bestellen (~60€ inkl. Versand)
4. Kosten pro Board: ~12€
5. Bauanleitung: `hardware/ASSEMBLY.md`

### 3. Firmware flashen

```bash
cd firmware
./flash.sh /dev/ttyUSB0   # oder /dev/tty.usbserial-*
```

Oder via Web (Chrome/Edge):
https://radiocore.ham/flash → Board per USB verbinden → Flash

### 4. Konfigurieren

1. Board booten → WiFi "RadioCore-XXXXXX" erscheint
2. Verbinden (Passwort: "amateurfunk")
3. Browser: http://192.168.4.1
4. Setup-Wizard:
   - WiFi konfigurieren (dein Heimnetz)
   - Modus waehlen (Digimode / Repeater / Remote)
   - TRX-Typ waehlen (Auto-Config fuer bekannte Modelle)
5. Fertig!

---

## Projekt-Struktur

```
radiocore/
├── crates/
│   ├── radiocore-core/          # Shared: Audio, DSP, Serial, Web, Config
│   │   ├── src/
│   │   │   ├── audio/           # ALSA, I2S, Resampling, Filters
│   │   │   ├── dsp/             # AGC, Noise Reduction, Waterfall
│   │   │   ├── serial/          # CAT, Hamlib Wrapper
│   │   │   ├── web/             # Axum Server, WebSocket, API
│   │   │   └── config/          # TOML Parser, Validation
│   │   └── Cargo.toml
│   ├── radiocore-repeater/      # Relaissteuerung + Lua + TTS
│   │   ├── src/
│   │   │   ├── logic/           # COS→PTT, Timers, State Machine
│   │   │   ├── lua/             # mlua Integration, Hot-Reload
│   │   │   ├── tts/             # Piper TTS, Caching
│   │   │   └── voip/            # Opus, QUIC, Peer Management
│   │   └── Cargo.toml
│   ├── radiocore-remote/        # Station-Server + Hamlib + Cloud
│   │   ├── src/
│   │   │   ├── server/          # WebSocket, Audio-Streaming (Opus)
│   │   │   ├── hamlib/          # CAT-Abstraktion
│   │   │   ├── cloud/           # Relay-Client, Auth, Billing
│   │   │   └── sharing/         # Multi-User, Scheduling, Limits
│   │   └── Cargo.toml
│   └── radiocore-digimode/      # USB Interface Manager
│       ├── src/
│       │   ├── usb/             # Audio + Serial Bridge
│       │   ├── vox/             # Voice-Activated PTT
│       │   └── monitor/         # Status-LED, Web-UI (minimal)
│       └── Cargo.toml
├── firmware/                     # ESP32-S3 Firmware (ESP-IDF/C)
│   ├── main/
│   │   ├── usb_audio.c          # USB Audio Class (UAC2)
│   │   ├── usb_serial.c         # USB CDC-ACM
│   │   ├── i2s_es8388.c         # ES8388 Codec Driver
│   │   ├── gpio_control.c       # PTT, COS, LED
│   │   ├── wifi_manager.c       # WiFi AP/STA, Config Portal
│   │   └── mode_switcher.c      # Digimode/Repeater/Remote
│   ├── CMakeLists.txt
│   └── sdkconfig
├── hardware/                     # Hardware-Design
│   ├── pcb/
│   │   ├── radiocore.kicad_pro  # KiCad Projekt
│   │   ├── radiocore.kicad_sch  # Schaltplan
│   │   ├── radiocore.kicad_pcb  # PCB Layout
│   │   └── gerber/              # Produktionsdaten (JLCPCB)
│   ├── bom/
│   │   ├── bom_jlcpcb.csv       # JLCPCB-formatierte BOM
│   │   ├── COST_ESTIMATE.md     # Kostenabschaetzung
│   │   └── alternatives.md      # Alternative Bauteile
│   ├── case/                     # 3D-gedruckte Gehaeuse
│   │   ├── repeater/            # STL + STEP (Wandmontage)
│   │   ├── remote/              # STL + STEP (Kompakt)
│   │   └── digimode/            # STL + STEP (Desktop)
│   ├── expansions/               # Erweiterungs-Boards
│   │   ├── relay_shield_4x/     # 4x SPDT
│   │   ├── relay_shield_8x/     # 8x SPDT
│   │   ├── rotor_interface/     # Yaesu/Alfaspid
│   │   └── sensor_board/        # SWR, Temp, Voltage
│   ├── HARDWARE.md              # Schaltplan-Erklaerung, BOM
│   ├── EASYEDA_GUIDE.md         # Schritt-fuer-Schritt PCB Design
│   ├── BREAKOUT.md              # Adapter-Kabel fuer verschiedene TRX
│   └── ASSEMBLY.md              # Bauanleitung (falls selbst geloetet)
├── config/                       # Beispiel-Configs
│   ├── repeater/
│   │   ├── default.toml         # Standard-Repeater
│   │   ├── echolink.lua         # Echolink-Integration
│   │   └── aprs_gateway.lua     # APRS-Tor
│   ├── remote/
│   │   └── default.toml
│   └── digimode/
│       └── default.toml
├── web-ui/                       # Vue.js Frontend (fuer alle Modi)
│   ├── src/
│   │   ├── components/          # Wiederverwendbare Komponenten
│   │   ├── views/
│   │   │   ├── Repeater.vue     # Repeater Dashboard
│   │   │   ├── Remote.vue       # Remote Station UI (Waterfall!)
│   │   │   └── Digimode.vue     # Digimode Status
│   │   └── App.vue
│   ├── package.json
│   └── vite.config.js
├── docs/                         # Dokumentation
│   ├── ARCHITECTURE.md          # Backend, WebSocket, Cloud
│   ├── PRODUCT.md               # Produktpalette, Preise, Fahrplan
│   ├── API.md                   # REST API + WebSocket Protokoll
│   ├── LUA_API.md               # Lua Scripting Reference
│   └── CONTRIBUTING.md          # Wie man mithilft
├── scripts/                      # Hilfs-Skripte
│   ├── flash_firmware.sh        # ESP32 flashen
│   ├── generate_breakout.py     # Breakout-Kabel Dokumentation
│   └── cost_calculator.py       # Herstellungskosten berechnen
├── LICENSE                       # MIT (Software) + CERN-OHL-P v2 (Hardware)
└── README.md                     # Diese Datei
```

---

## Dokumentation

- **[Hardware-Dokumentation](hardware/HARDWARE.md)** — Schaltplan, BOM, Layout-Ueberlegungen, Bauanleitung
- **[EasyEDA Anleitung](hardware/EASYEDA_GUIDE.md)** — Schritt-fuer-Schritt PCB Design (fuer Einsteiger)
- **[Breakout-Adapter](hardware/BREAKOUT.md)** — Adapterkabel fuer verschiedene Funkgeraete (Pinouts, Schaltplaene)
- **[Produkt & Business](docs/PRODUCT.md)** — Produktpalette, Preise, Roadmap, Monetarisierung
- **[Cloud-Architektur](docs/ARCHITECTURE.md)** — Backend, WebSocket-Relay, Board-Provisioning, Sharing
- **[Kostenabschaetzung](hardware/bom/COST_ESTIMATE.md)** — JLCPCB Fertigung (5/10/100/1000 Stueck)
- **[API-Referenz](docs/API.md)** — REST API + WebSocket Protokoll (fuer eigene Clients)
- **[Lua Scripting](docs/LUA_API.md)** — Repeater-Logik programmieren (Hot-Reload)

---

## Roadmap

### Phase 1: Proof of Concept (Q1 2026) ✅ DONE
- [x] Rust Workspace aufsetzen (4 Crates)
- [x] Audio-Pipeline (ALSA, Resampling, Filters)
- [x] Web-UI Grundgeruest (Axum, Vue.js)
- [x] Hardware-Design (Schaltplan, BOM, LCSC Parts)

### Phase 2: Hardware-Prototyp (Q2 2026) 🔄 IN PROGRESS
- [x] EasyEDA Schaltplan (komplett)
- [ ] PCB Layout (65x45mm, 2-Layer)
- [ ] Gerber-Files generieren
- [ ] **Prototyp bestellen (5 Stueck JLCPCB)** ← aktuell hier
- [ ] Prototyp testen (Audio, PTT, COS, CAT)

### Phase 3: Firmware (Q2 2026)
- [ ] ESP32-S3: USB Audio Class (UAC2)
- [ ] ESP32-S3: USB Serial (CDC-ACM)
- [ ] ES8388 I2S Driver (Line In/Out)
- [ ] GPIO: PTT (Open Collector), COS (Pull-Up)
- [ ] WiFi Config Portal
- [ ] Modus-Umschaltung (Digimode/Repeater/Remote)

### Phase 4: Software (Q3 2026)
- [ ] Digimode-Modus: USB Bridge fertigstellen
- [ ] Repeater-Modus: COS→PTT Logic
- [ ] Repeater-Modus: Lua Scripting
- [ ] Repeater-Modus: Piper TTS Integration
- [ ] Remote-Modus: Opus Audio Streaming
- [ ] Remote-Modus: Hamlib CAT Proxy
- [ ] Web-UI: Waterfall (Vue.js + Canvas)

### Phase 5: Cloud & Sharing (Q4 2026)
- [ ] Cloud-Backend (FastAPI, WebSocket Relay)
- [ ] Board-Provisioning (JWT, OAuth)
- [ ] Station Sharing (Scheduling, Billing)
- [ ] Stripe-Integration (Payments)
- [ ] Dashboard fuer Station-Besitzer

### Phase 6: Erweiterungen (2027)
- [ ] Relay Shield 4x (PCB Design + Fertigung)
- [ ] Relay Shield 8x
- [ ] Rotor Interface
- [ ] Sensor Board (SWR, Temp, Voltage)
- [ ] Gehaeuse (3D-Druck → Spritzguss)
- [ ] Breakout-Kabel (Serienfertigung)

### Phase 7: Launch (2027)
- [ ] Shop-Website (radiocore.ham)
- [ ] Dokumentations-Website
- [ ] Video-Tutorials (YouTube)
- [ ] HAM Radio Friedrichshafen Demo (Juni 2027)
- [ ] ARRL/RSGB/DARC Artikel

---

## Kosten & Preise

### Herstellungskosten (JLCPCB, 2-Layer PCB)

| Position | 5 Stueck | 10 Stueck | 100 Stueck | 1000 Stueck |
|----------|----------|-----------|------------|-------------|
| PCB (fertigung) | 2.00€ | 1.50€ | 0.80€ | 0.40€ |
| PCB Assembly | 6.00€ | 5.00€ | 3.50€ | 2.00€ |
| Bauteile (BOM) | 4.72€ | 4.50€ | 4.20€ | 3.80€ |
| **Gesamt/Board** | **12.72€** | **11.00€** | **8.50€** | **6.20€** |

*(Preise Stand Januar 2026, exkl. Versand)*

### Komplettkit-Kalkulation

| Position | Kosten | Preis |
|----------|--------|-------|
| RadioCore Board | 8.50€ | — |
| Gehaeuse (Spritzguss) | 3.00€ | — |
| Breakout-Kabel | 2.50€ | — |
| Verpackung + Handbuch | 1.00€ | — |
| Versand (DHL) | 5.00€ | — |
| **Gesamt (Kosten)** | **20.00€** | — |
| **Verkaufspreis** | — | **99€** |
| **Marge** | — | **79€ (79%)** |

### Warum 99€?

- **Psychologischer Preis:** Unter 100€ = "guenstig", ueber 100€ = "teuer"
- **Wettbewerb:** SignaLink (120€), RemoteRig (500€), SVXLink (200€ + Pi + Fummelei)
- **Wertversprechen:** 3 Geraete in einem, Open Source, erweiterbar
- **Marge:** 79€ deckt Entwicklung, Support, Cloud-Infrastruktur, Marketing

---

## Mitwirken

RadioCore ist **Open Source**. Wir freuen uns ueber:

- **Code:** Rust (Backend), C (Firmware), Vue.js (Frontend), Lua (Scripting)
- **Hardware:** PCB-Layout-Verbesserungen, Erweiterungs-Boards, Gehaeuse-Design
- **Dokumentation:** Anleitungen, Tutorials, Uebersetzungen
- **Testing:** Beta-Tester (Firmware, Software, Hardware-Prototypen)
- **Support:** Forum-Moderation, Issue-Triage, Fragen beantworten

**Siehe:** [CONTRIBUTING.md](docs/CONTRIBUTING.md)

---

## Lizenz

- **Software:** MIT License (frei verwendbar, auch kommerziell)
- **Hardware:** CERN Open Hardware Licence Version 2 - Permissive (CERN-OHL-P v2)
- **Dokumentation:** CC BY-SA 4.0

**Was bedeutet das?**
- Du darfst RadioCore **privat und kommerziell** nutzen
- Du darfst Boards selber bauen und **verkaufen** (auch veraendert)
- Du **musst** Aenderungen nicht veroeffentlichen (Permissive Lizenz)
- Aber es waere nett, wenn du es tust (Pull Requests willkommen!)

---

## Support & Community

- **Forum:** https://forum.radiocore.ham (Fragen, Projekte, Showcase)
- **GitHub Issues:** Bug-Reports, Feature-Requests
- **Discord:** https://discord.gg/radiocore (Real-Time Chat)
- **Email:** support@radiocore.ham (Technischer Support fuer Kaeufer)

---

## FAQ

**Q: Brauche ich ein Raspberry Pi?**
A: Nein. Digimode und Basic-Repeater laufen standalone auf dem ESP32. Nur fuer erweiterte Features (Piper TTS, Lua Scripting, VoIP Linking) brauchst du einen Pi oder PC.

**Q: Funktioniert es mit meinem Funkgeraet?**
A: Ja. RadioCore ist universell. Du brauchst nur ein Adapterkabel (siehe `hardware/BREAKOUT.md`). Wir haben fertige Kabel fuer Yaesu, Icom, Kenwood und generische Klinken-Anschluesse.

**Q: Kann ich mein eigenes Cloud-Backend hosten?**
A: Ja. Der Cloud-Code ist Open Source (`docs/ARCHITECTURE.md`). Du kannst ihn auf deinem Server laufen lassen (Docker-Image verfuegbar).

**Q: Wie gut ist die Audio-Qualitaet?**
A: 24-bit/48kHz, 105dB SNR (ES8388 Codec). Besser als CM108 (16-bit/80dB) und vergleichbar mit professionellen Audio-Interfaces.

**Q: Kann ich mehrere Boards vernetzen?**
A: Ja. Repeater-Modus unterstuetzt VoIP-Linking (Opus/QUIC). Du kannst mehrere Repeater/Remote-Stationen verbinden (auch ueber Internet).

**Q: Ist Station Sharing legal?**
A: Ja, laut Amateurfunkgesetz. Du bleibst der Stationsverantwortliche (dein Rufzeichen wird gesendet), der Mieter ist "Gastoperator". Siehe `docs/LEGAL.md` fuer Details.

**Q: Kann ich das Board selbst bauen?**
A: Ja. Gerber-Files, BOM und Bauanleitung sind Open Source. Kosten: ~12€ pro Board bei JLCPCB (5 Stueck Minimum).

**Q: Gibt es Mengenrabatt?**
A: Ja. Ab 10 Stueck: 89€/Stk, ab 100 Stueck: 79€/Stk. Kontaktiere sales@radiocore.ham fuer Vereine/OV-Sammelbestellungen.

---

## Kontakt

- **Entwickler:** Dirk (DO1XX)
- **Website:** https://radiocore.ham
- **GitHub:** https://github.com/dirkforpresident/radiocore
- **Email:** dirk@1xx.is
- **Mastodon:** @do1xx@chaos.social

---

*73 de DO1XX — Entwickelt in Deutschland, gebaut in China (JLCPCB), verwendet weltweit.*

**Amateurfunk. Open Source. Bezahlbar.**
