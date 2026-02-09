# Getting Started

## Voraussetzungen

### Software (Rust Host)
- Rust 1.75+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- Eine Soundkarte (eingebaute, USB, oder RadioCore Board)
- Optional: Raspberry Pi 4/5

### Firmware (ESP32-S3)
- ESP-IDF v5.x
- USB-C Kabel
- RadioCore Board oder ESP32-S3 DevKit + ES8388 Modul

## Quick Start (ohne Board)

```bash
git clone https://github.com/dirkforpresident/radiocore
cd radiocore

# Repeater-Modus
cargo run -p radiocore-repeater

# Digimode-Modus
cargo run -p radiocore-digimode

# Remote-Modus
cargo run -p radiocore-remote
```

Browser: http://localhost:8080

## Mit RadioCore Board

1. Board per USB-C anschliessen
2. Board erscheint als Audio-Device + Serial Port
3. In `config/local.toml`:

```toml
[audio]
device = "RadioCore"
serial_port = "/dev/ttyACM0"
```

4. `cargo run -p radiocore-repeater` — fertig!

## Transceiver anschliessen (RJ45)

```
Pin 1: RX Audio (TRX Speaker → Board Line In)
Pin 2: TX Audio (Board Line Out → TRX Mic)
Pin 3: PTT (Open Collector)
Pin 4: COS/SQL
Pin 5: CAT TX (UART)
Pin 6: CAT RX (UART)
Pin 7: +5V
Pin 8: GND
```
