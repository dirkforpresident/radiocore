# RadioCore

Ein Board. Drei Anwendungen. Open-Source Amateurfunk-Hardware.

**~12EUR | ESP32-S3 + ES8388 | USB Audio + Serial | MIT License**

```
┌──────────────────────────────────────────┐
│         RadioCore Board (~12€)           │
│   ESP32-S3 + ES8388 + PTT + COS + CAT   │
│                                          │
│   [USB-C]           [RJ45 Funkgeraet]   │
└──────┬───────────────────────────────────┘
       │
       ├── radiocore-repeater    Relaissteuerung (SVXLink Ersatz)
       ├── radiocore-remote      Fernbedienung (RemoteRig Ersatz)
       └── radiocore-digimode    Digimode Interface (SignaLink Ersatz)
```

## Was ist RadioCore?

Ein universelles Audio-Interface fuer Amateurfunk. **Eine Hardware** ersetzt drei kommerzielle Produkte:

| RadioCore Modus | Ersetzt | Preis des Originals |
|---|---|---|
| **Repeater** | SVXLink + CM108 | ~200EUR + Bastelei |
| **Remote** | RemoteRig RRC-1258 | ~600EUR |
| **Digimode** | SignaLink USB | ~120EUR |

### Vergleich mit SVXLink (Repeater-Modus)

| | SVXLink | RadioCore |
|---|---------|-----------|
| Audio-Latenz | 40-80ms | **<5ms** |
| Audio-Qualitaet | 16-bit/80dB (CM108) | **24-bit/105dB (ES8388)** |
| Web-UI | Nein | **Ja, live** |
| TTS | espeak (roboterhaft) | **Piper KI-Stimme** |
| Sprache | C++/Tcl | **Rust/Lua** |
| Hardware | CM108 Bastelei | **Fertiges Board, 12EUR** |

## Quick Start

```bash
git clone https://github.com/dirkforpresident/radiocore
cd radiocore

# Repeater-Modus (mit beliebiger Soundkarte)
cargo run -p radiocore-repeater

# Digimode-Modus
cargo run -p radiocore-digimode

# Remote Station-Modus
cargo run -p radiocore-remote
```

Browser: http://localhost:8080

## Hardware

### RadioCore Board

```
60 x 40 mm | 2-Layer | ~12 EUR fertig bestueckt (JLCPCB)

USB-C ──► ESP32-S3 ──► I2S ──► ES8388 ──► Line Out (TX Audio)
                                        ◄── Line In  (RX Audio)
                       GPIO ──────────► PTT (Open Collector)
                              ◄──────── COS/SQL
                       UART ──────────► CAT TX
                              ◄──────── CAT RX
```

### Stueckliste

| Bauteil | Preis |
|---------|-------|
| ESP32-S3-WROOM-1 (N16R8) | 3,50 EUR |
| ES8388 Audio Codec | 0,80 EUR |
| USB-C, LDO, MOSFET, LED, Passive | 0,70 EUR |
| PCB + Assembly (JLCPCB, 5 Stk) | 7,00 EUR |
| **Gesamt** | **~12 EUR** |

### RJ45 Anschluss zum Funkgeraet

```
Pin 1: RX Audio (TRX Speaker Out → Board Line In)
Pin 2: TX Audio (Board Line Out → TRX Mic In)
Pin 3: PTT (Open Collector, Active Low)
Pin 4: COS/SQL (Active Low, Pull-Up)
Pin 5: CAT TX (3.3V UART)
Pin 6: CAT RX (3.3V UART)
Pin 7: +5V (optional)
Pin 8: GND
```

## Standalone (ohne Pi/Computer)

Das Board kann auch **ohne Raspberry Pi** betrieben werden:

| Funktion | Standalone ESP32 | Mit Pi/PC |
|---|---|---|
| Repeater (Basis) | Ja | Ja |
| Repeater + TTS | WAV aus Flash | Piper Live-TTS |
| Digimode Interface | Ja (USB Bridge) | Ja |
| Remote Station | Ja (WiFi) | Ja (Internet) |
| VoIP Linking | Einfach | Opus/QUIC |
| Web-UI | Klein (WiFi) | Voll |

## Architektur

```
radiocore/
├── crates/
│   ├── radiocore-core/        # Shared: Audio, DSP, Serial, Web
│   ├── radiocore-repeater/    # Relaissteuerung + Lua + TTS
│   ├── radiocore-remote/      # Station-Server + Hamlib
│   └── radiocore-digimode/    # USB Interface Manager
├── firmware/                  # ESP32-S3 Firmware (ESP-IDF/C)
├── hardware/                  # KiCad PCB + 3D-Druck Gehaeuse
│   ├── pcb/
│   └── case/
│       ├── repeater/          # Wandmontage / DIN-Schiene
│       ├── remote/            # Kompakt neben TRX
│       └── digimode/          # Desktop
├── config/                    # TOML + Lua Scripts
└── docs/                      # Dokumentation
```

## Drei Gehaeuse, ein Board

```
Repeater               Remote Station          Digimode
┌─────────────┐        ┌─────────────┐        ┌─────────────┐
│ LED  [USB-C]│        │ LED  [USB-C]│        │ LED  [USB-C]│
│             │        │             │        │             │
│ [RJ45] RX  │        │ [RJ45] TRX  │        │ [RJ45] TRX  │
│ [RJ45] TX  │        │             │        │             │
│             │        │             │        │  DO1XX       │
│ Wandmontage │        │ Kompakt     │        │  Desktop     │
└─────────────┘        └─────────────┘        └─────────────┘
```

## Firmware-Modi

Die ESP32-S3 Firmware unterstuetzt drei Modi (umschaltbar per Serial/WiFi):

- **DIGIMODE** (Standard): Transparente USB Audio Bridge
- **REPEATER**: Standalone COS→PTT, Timer, WAV-Ansagen
- **REMOTE**: USB Audio + CAT UART Forwarding

## Projekt-Status

- [ ] Phase 1: Software Proof of Concept (Audio, PTT, Web-UI)
- [ ] Phase 2: ESP32-S3 Firmware (USB Audio + Serial)
- [ ] Phase 3: PCB Design + Fertigung
- [ ] Phase 4: Standalone Repeater auf ESP32
- [ ] Phase 5: VoIP Linking (Opus/QUIC)
- [ ] Phase 6: Remote Head Hardware (Display + Encoder)

## Lizenz

MIT License — siehe [LICENSE](LICENSE)

---

*73 de DO1XX — Elektronik Reich / HanseMesh*
