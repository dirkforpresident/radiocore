# RadioCore Spezifikation

## Kurzfassung

- **Hardware**: ESP32-S3 + ES8388, USB Audio + Serial, ~12 EUR
- **Firmware**: ESP-IDF/C, USB Composite (UAC2 + CDC), I2S Audio, 3 Modi
- **Software**: Rust Workspace, cpal Audio, axum Web-Server
- **Anwendungen**: Repeater (+ Lua), Remote Station (+ Hamlib), Digimode Interface
- **Vernetzung**: Opus/QUIC VoIP Linking (Phase 3)

## Pinbelegung ESP32-S3

| Pin | Signal | Beschreibung |
|-----|--------|-------------|
| GPIO 4 | I2S_BCK | Bit Clock |
| GPIO 5 | I2S_WS | Word Select |
| GPIO 6 | I2S_DOUT | Data Out (DAC) |
| GPIO 7 | I2S_DIN | Data In (ADC) |
| GPIO 8 | I2C_SDA | ES8388 Config |
| GPIO 9 | I2C_SCL | ES8388 Clock |
| GPIO 10 | PTT_OUT | Open Collector |
| GPIO 11 | COS_IN | Squelch Input |
| GPIO 12 | LED_DATA | WS2812B |
| GPIO 17 | CAT_TX | UART TX |
| GPIO 18 | CAT_RX | UART RX |
| GPIO 19/20 | USB D-/D+ | USB 2.0 |

## Steuerprotokoll (USB Serial)

```
PTT ON / PTT OFF
STATUS
LEVEL
SET RX_GAIN <0-24>
SET TX_GAIN <-96 to 0>
SET SAMPLE_RATE 48000
MODE REPEATER / DIGIMODE / REMOTE
INFO
```

## Firmware-Modi

### DIGIMODE (Standard)
Transparente USB Audio Bridge. PC sieht Soundkarte + COM-Port.

### REPEATER
Standalone Relaissteuerung. COS→PTT, Timer, WAV-Ansagen aus Flash.

### REMOTE
USB Audio + CAT UART Forwarding fuer Remote-Betrieb.
