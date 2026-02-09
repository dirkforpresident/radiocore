# RadioCore v1.0 — Schaltplan

## Block-Diagramm

```
   USB-C (5V)                    DC Eingang (5-24V)
       │                              │
       │ VBUS                    ┌────┴─────┐
       │                         │ XL1509   │
       │                         │ Buck→5V  │
       │                         └────┬─────┘
       │                              │ 5V
       └──[D3 Schottky]──┬──[D4 Schottky]──┘
                          │
                     5V Rail ──► WS2812B VCC
                          │
                    ┌─────┴─────┐
                    │ AMS1117   │
                    │  3.3V     │
                    └─────┬─────┘
                          │ 3.3V
          ┌───────────────┼──────────────────────────┐
          │               │                          │
   ┌──────┴──────┐  ┌────┴───────┐          ┌───────┴─────┐
   │  ESP32-S3   │  │   ES8388   │          │   WS2812B   │
   │  WROOM-1    │  │   Codec    │          │   RGB LED   │
   │  N16R8      │  │            │          └─────────────┘
   │             │  │ Line In ◄──┤◄── RJ45 Pin 1 (RX Audio)
   │ GPIO4 BCK──►├──►──BCK      │
   │ GPIO5 WS───►├──►──WS       │
   │ GPIO6 DOUT─►├──►──DIN      │──► RJ45 Pin 2 (TX Audio)
   │ GPIO7 DIN──◄├──◄──DOUT     │        Line Out
   │ GPIO3 MCLK─►├──►──MCLK     │
   │             │  │            │
   │ GPIO8 SDA──►├──►──SDA      │ (4.7K Pull-Ups)
   │ GPIO9 SCL──►├──►──SCL      │
   │             │  └────────────┘
   │ GPIO10─────►├──►──[2N7002]──► RJ45 Pin 3 (PTT)
   │ GPIO11─────◄├──◄────────────── RJ45 Pin 4 (COS/SQL)
   │ GPIO12─────►├──►──WS2812B DIN
   │ GPIO17 TX──►├──►────────────── RJ45 Pin 5 (CAT TX)
   │ GPIO18 RX──◄├──◄────────────── RJ45 Pin 6 (CAT RX)
   │             │
   │ GPIO19 D-──►├──►──USB-C D-
   │ GPIO20 D+──►├──►──USB-C D+
   │             │
   │ EN─────────◄├──◄──[SW1 Reset]──GND
   │ GPIO0──────◄├──◄──[SW2 Boot]───GND
   │             │
   │ GPIO13─────►├──►──[EXP Header Pin 5]  (Open Collector 1)
   │ GPIO14─────►├──►──[EXP Header Pin 6]  (Open Collector 2)
   │ GPIO15─────►├──►──[EXP Header Pin 7]  (Open Collector 3)
   │ GPIO16─────►├──►──[EXP Header Pin 8]  (Open Collector 4)
   └─────────────┘

   DC Eingang: Schraubklemme 2-Pin (5-24V, z.B. 13.8V Funkgeraet-Netzteil)

   Anschluesse:
   [USB-C]  [DC 5-24V]  [RST] [BOOT]  [EXP 2x5]  ← Oberseite
   [RJ45 geschirmt]  ○ RX In  ○ TX Out            ← Unterseite
    (alle Signale)    3.5mm    3.5mm
                     (optional, parallel zu RJ45 Audio)

   RJ45 Belegung (T568B Paar-Zuordnung):
   Pin 1+2 (Paar 1 orange): RX Audio IN  (verdrillt = Stoerschutz)
   Pin 3+6 (Paar 2 gruen):  TX Audio OUT (verdrillt = Stoerschutz)
   Pin 4+5 (Paar 3 blau):   PTT + COS
   Pin 7+8 (Paar 4 braun):  CAT TX + CAT RX
   Schirm (Metallgehaeuse):  GND

   3.5mm Klinken (optional, parallel zu RJ45):
   RX In:  Tip=Audio, Ring=NC, Sleeve=GND
   TX Out: Tip=Audio, Ring=NC, Sleeve=GND
```

## Detailschaltung

### 1. Spannungsversorgung (Dual-Input: USB oder 5-24V DC)

```
Quelle A: USB-C VBUS (5V)
  USB VBUS ── C4 (22uF) ── [D3 Schottky SS34] ──┐
                                                  │
Quelle B: DC Eingang (5-24V, z.B. 13.8V)         │
  DC+ ── C7 (100uF/50V) ──┐                      │
                           │                      │
                      ┌────┴────┐                 │
                      │ XL1509  │ Buck DC-DC      │
                      │ 5.0E1   │ (4.5-28V→5V)   │
                      │         │                 │
                      │ FB─R7─R8│ (fest 5V)       │
                      │ BST─C8  │ (100nF)         │
                      │ SW──L1──┤ (33uH Inductor) │
                      │   C9────┤ (22uF Output)   │
                      └────┬────┘                 │
                           │                      │
                      [D4 Schottky SS34] ─────────┤
                                                  │
                                             5V Rail
                                                  │
                                        ┌─────────┤
                                        │         │
                                    WS2812B   ┌───┴───┐
                                     VCC      │AMS1117│
                                              │ 3.3V  │
                                              └───┬───┘
                                                  │
                                             3.3V Rail
                                                  │
                                           ESP32 + ES8388

Automatische Umschaltung: Schottky-Dioden (D3, D4) verhindern
Rueckspeisung. Hoechste Spannung gewinnt. USB allein = OK.
DC allein = OK. Beides gleichzeitig = auch OK.
```

### 2. ESP32-S3 WROOM-1

```
3.3V ──┬── C1a (100nF) ── GND
       │
       ├── R4 (100K) ── EN Pin
       │
       ├── R5 (10K) ── GPIO0 (Boot)
       │
       └── ESP32-S3 VDD

EN Pin ── [SW1] ── GND    (Reset: druecken = Reset)
GPIO0  ── [SW2] ── GND    (Boot:  halten + Reset = Flash-Modus)
```

### 3. ES8388 Audio Codec

```
3.3V ── DVDD (Pin 1)  ── C1b (100nF) ── GND
3.3V ── AVDD (Pin 27) ── C6a (10nF)  ── GND
3.3V ── AVDD (Pin 28) ── C6b (10nF)  ── GND
        VREFP (Pin 16) ── C5a (1uF)  ── GND
        VREFN (Pin 15) ── C5b (1uF)  ── GND

I2S Verbindung:
  SCLK  (Pin 2)  ◄── GPIO4 (BCK)
  LRCK  (Pin 3)  ◄── GPIO5 (WS)
  DSDIN (Pin 4)  ◄── GPIO6 (ESP32 DOUT → ES8388 DAC Input)
  ASDOUT(Pin 7)  ──► GPIO7 (ES8388 ADC Output → ESP32 DIN)
  MCLK  (Pin 22) ◄── GPIO3 (Master Clock, 12.288MHz von ESP32 APLL)

I2C Konfiguration:
  SCL (Pin 24) ◄── GPIO9 ── R1a (4.7K) ── 3.3V
  SDA (Pin 23) ◄── GPIO8 ── R1b (4.7K) ── 3.3V
  AD0 (Pin 25) ── GND (I2C Adresse: 0x10)

Audio Ein/Ausgaenge:
  LOUT1 (Pin 19) ── 100nF ── RJ45 Pin 2 (TX Audio, Line Out)
  ROUT1 (Pin 18) ── NC (nur Mono)
  LINPUT1 (Pin 10) ◄── 100nF ◄── RJ45 Pin 1 (RX Audio, Line In)
  RINPUT1 (Pin 9) ── NC (nur Mono)
```

### 4. PTT Ausgang (Open Collector)

```
GPIO10 ── R (10K) ── Gate ┐
                          │ 2N7002
RJ45 Pin 3 (PTT) ── Drain┘
                     Source ── GND

Funktion: GPIO10 HIGH = MOSFET leitet = PTT Pin auf GND gezogen
          GPIO10 LOW  = MOSFET sperrt  = PTT Pin offen (hochohmig)
```

### 5. COS/SQL Eingang

```
RJ45 Pin 4 (COS) ──┬── R2 (10K) ── 3.3V
                    │
                    └── GPIO11

Funktion: COS offen/HIGH = Squelch zu
          COS auf GND    = Squelch offen
```

### 6. CAT UART

```
GPIO17 (TX) ── RJ45 Pin 5 (CAT TX)
GPIO18 (RX) ◄── RJ45 Pin 6 (CAT RX)

3.3V UART Pegel! Fuer 5V TTL: Level-Shifter noetig.
```

### 7. USB Type-C

```
USB-C Pin A6/B6 (D+)  ── GPIO20
USB-C Pin A7/B7 (D-)  ── GPIO19
USB-C Pin A5 (CC1)     ── R3a (5.1K) ── GND
USB-C Pin B5 (CC2)     ── R3b (5.1K) ── GND
USB-C Pin A4/B4 (VBUS) ── [D2 ESD] ── 5V Rail
USB-C Pin A1/B1 (GND)  ── GND

5.1K an CC1/CC2 = Device Mode (kein Host)
```

### 8. Status LED (WS2812B)

```
3.3V ── VCC (WS2812B) ── C1c (100nF) ── GND
GPIO12 ── R6 (100R) ── DIN (WS2812B)
DOUT (WS2812B) ── NC (nur 1 LED)
```

### 9. EMC / HF-Schutz (wichtig fuer Amateurfunk-Umgebung!)

```
Audio-Leitungen (RJ45 Pin 1+2):
  Line In:  RJ45 ── FB1 (Ferrite 600R@100MHz) ── C_LP (1nF) ── ES8388 LINPUT1
  Line Out: ES8388 LOUT1 ── FB2 (Ferrite 600R@100MHz) ── C_LP (1nF) ── RJ45

PTT / COS (RJ45 Pin 3+4):
  PTT:  2N7002 Drain ── FB3 (Ferrite) ── RJ45 Pin 3
  COS:  RJ45 Pin 4 ── FB4 (Ferrite) ── C_LP (100pF) ── GPIO11

CAT UART (RJ45 Pin 5+6):
  TX: GPIO17 ── FB5 (Ferrite) ── RJ45 Pin 5
  RX: RJ45 Pin 6 ── FB6 (Ferrite) ── C (100pF) ── GPIO18

Stromversorgung:
  5V Rail:  ── FB7 (Ferrite 1A) ── ESP32 VDD
  3.3V:     ── FB8 (Ferrite 1A) ── ES8388 AVDD
  DC Input: Schottky + TVS (SMBJ24A) fuer Ueberspannungsschutz

USB:
  D+/D-: Bereits geschuetzt durch USBLC6-2SC6 (D2)

Ground:
  Durchgehende Ground-Plane auf Bottom-Layer
  Analog-GND und Digital-GND getrennt, Sternpunkt unter AMS1117
  Keine Ground-Plane unter ESP32-S3 Antenne!

Ferrite Beads: 0402 SMD, 600 Ohm @ 100MHz (LCSC: C1015)
```

### 10. Expansion Header (2x5 Pin)

```
Pin-Belegung (von oben, Beschriftung auf PCB):

    ┌─────┬─────┐
  1 │ 5V  │ GND │ 2
    ├─────┼─────┤
  3 │ SDA │ SCL │ 4     I2C Bus (geteilt mit ES8388)
    ├─────┼─────┤
  5 │ IO13│ IO14│ 6     GPIO / Open Collector Out
    ├─────┼─────┤
  7 │ IO15│ IO16│ 8     GPIO / Open Collector Out
    ├─────┼─────┤
  9 │3.3V │ GND │ 10
    └─────┴─────┘

GPIOs 13-16: Direkt vom ESP32-S3, 3.3V Logik.
Fuer Relais/Lasten: Externe MOSFETs (2N7002) auf Erweiterungsboard.
I2C: Bereits mit 4.7K Pull-Ups von ES8388-Beschaltung.
5V: Direkt von 5V Rail (max 500mA gesamt inkl. Board).
3.3V: Vom AMS1117 (max 200mA frei fuer Erweiterungen).
```

Typische Erweiterungen:
- Relay Shield 4x/8x (per GPIO oder I2C MCP23017)
- Rotor-Interface (I2C + PWM)
- Band-Decoder (I2C MCP23017, BCD-Ausgabe)
- OLED Display (I2C SSD1306/SH1106)
- Sensor-Board (I2C ADS1115 fuer SWR/Temperatur/Spannung)
- Remote Head (I2C Display + Encoder + Taster)

## PCB Layout Hinweise

- **Groesse**: 65 x 45 mm, 2-Layer
- **USB-C**: An der kurzen Seite oben
- **RJ45**: An der kurzen Seite unten
- **Expansion Header**: 2x5 Pin-Header (2.54mm Raster) an der langen Seite, zugaenglich fuer Erweiterungsboards
- **ESP32-S3 Modul**: Mittig, Antenne zum Rand (keine Kupferflaeche unter Antenne!)
- **ES8388**: Neben dem ESP32, kurze I2S-Leitungen
- **Analog-Bereich** (ES8388 Audio): Getrennte Ground-Plane, sternfoermig verbunden
- **USB-Leitungen**: Differentiell routen, 90 Ohm Impedanz, so kurz wie moeglich
- **Decoupling Caps**: Direkt an den IC-Pins platzieren
- **Ground Plane**: Durchgehende Kupferflaeche auf Bottom-Layer, KEINE Schlitze unter Signalleitungen
- **Analog/Digital Trennung**: ES8388 Analog-Bereich eigene Ground-Zone, Sternpunkt-Verbindung
- **Antenne frei**: Keine Kupferflaeche unter ESP32-S3 Antenne (letzte 5mm des Moduls)
- **Ferrite Beads**: Alle RJ45-Leitungen gefiltert, direkt am Stecker platzieren
- **Audio-Routing**: RX/TX Audio getrennt, nicht parallel fuehren, kurze Wege
- **Buck-Converter**: XL1509 + Induktor moeglichst weit weg vom ES8388, eigene Ground-Insel
- **Vias**: Ground-Vias um empfindliche Bereiche (ES8388) als HF-Zaun

## Pinbelegung Uebersicht

| ESP32-S3 GPIO | Funktion | Verbindung |
|---|---|---|
| GPIO 3 | I2S MCLK | ES8388 MCLK |
| GPIO 4 | I2S BCK | ES8388 SCLK |
| GPIO 5 | I2S WS | ES8388 LRCK |
| GPIO 6 | I2S DOUT | ES8388 DSDIN (DAC) |
| GPIO 7 | I2S DIN | ES8388 ASDOUT (ADC) |
| GPIO 8 | I2C SDA | ES8388 SDA |
| GPIO 9 | I2C SCL | ES8388 SCL |
| GPIO 10 | PTT OUT | 2N7002 Gate → RJ45 |
| GPIO 11 | COS IN | RJ45 → Pull-Up |
| GPIO 12 | LED DATA | WS2812B DIN |
| GPIO 13 | EXP OUT 1 | Expansion Header Pin 5 |
| GPIO 14 | EXP OUT 2 | Expansion Header Pin 6 |
| GPIO 15 | EXP OUT 3 | Expansion Header Pin 7 |
| GPIO 16 | EXP OUT 4 | Expansion Header Pin 8 |
| GPIO 17 | CAT TX | RJ45 UART TX |
| GPIO 18 | CAT RX | RJ45 UART RX |
| GPIO 19 | USB D- | USB-C |
| GPIO 20 | USB D+ | USB-C |
