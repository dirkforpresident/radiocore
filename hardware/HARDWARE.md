# RadioCore v1.0 — Hardware-Dokumentation

**Eine Platine, drei Anwendungen: Repeater, Remote Station, Digimode Interface**

Diese Dokumentation richtet sich an Funkamateure, die RadioCore in EasyEDA nachbauen und bei JLCPCB fertigen lassen moechten.

---

## Inhaltsverzeichnis

1. [Ueberblick](#ueberblick)
2. [Board-Layout](#board-layout)
3. [Hauptkomponenten](#hauptkomponenten)
4. [Spannungsversorgung](#spannungsversorgung)
5. [Audio-Signalweg](#audio-signalweg)
6. [Steuer-Signale](#steuer-signale)
7. [Anschluesse](#anschluesse)
8. [EMC / HF-Schutz](#emc--hf-schutz)
9. [Stueckliste (BOM)](#stueckliste-bom)
10. [PCB Layout Richtlinien](#pcb-layout-richtlinien)
11. [Breakout-Adapter](#breakout-adapter-funkgeraet-seite)
12. [Erweiterungsboards](#erweiterungsboards-zubehoer)
13. [Firmware-Modi](#firmware-modi)
14. [Herstellung bei JLCPCB](#herstellung-bei-jlcpcb)

---

## Ueberblick

RadioCore ist eine universelle Audio-Interface-Platine fuer Amateurfunk-Anwendungen. Mit einem ESP32-S3 Mikrocontroller und einem ES8388 Audio-Codec verbindet sie Funkgeraete mit Computern oder arbeitet als eigenstaendige Repeater-Steuerung.

### Drei Anwendungsfaelle

| Modus | Beschreibung | Anwendung |
|-------|--------------|-----------|
| **DIGIMODE** | Transparente USB Audio Bridge | FT8, SSTV, Packet Radio, APRS |
| **REPEATER** | Standalone Relaissteuerung | FM-Repeater, Echolink-Node, Ansage-Texte |
| **REMOTE** | Remote Station Interface | Fernbedienung ueber Internet, CAT-Forwarding |

### Technische Daten

- **Abmessungen**: 65 x 45 mm, 2-Layer PCB
- **Mikrocontroller**: ESP32-S3-WROOM-1-N16R8 (16MB Flash, 8MB PSRAM, WiFi, Bluetooth)
- **Audio-Codec**: ES8388 (24-bit, 48kHz, 105dB SNR)
- **Anschluesse**: USB-C (Audio + Serial), RJ45 (alle Signale), 2x 3.5mm Klinke (Audio optional)
- **Spannungsversorgung**: USB-C 5V oder DC 5-24V (typisch 13.8V Funkgeraet-Netzteil)
- **Preis**: ca. 12.72 EUR pro Board (bei 5 Stueck von JLCPCB)
- **Verkaufspreis**: 99 EUR als Komplett-Kit

---

## Board-Layout

ASCII-Darstellung der Bauteil-Platzierung (Ansicht von oben):

```
                     65mm
    +---------------------------------------------+
    |  [USB-C]     [XL1509]  [L1]  [C9]          |
    |  [D2 ESD]     Buck     33uH  100uF         |
    |                                             |
    |  [SW1]  [SW2]                   [J5 EXP]   | 45mm
    |  BOOT   RESET                   2x5 Pin    |
    |                                             |
    |  +----------------+   +----------+          |
    |  |                |   |          |          |
    |  |   ESP32-S3     |   |  ES8388  |  [FB1]  |
    |  |   WROOM-1      |   |  Codec   |  [FB2]  |
    |  |                |   |          |  Audio  |
    |  |  [Antenna -->] |   +----------+   EMC   |
    |  +----------------+                         |
    |                                             |
    |  [C1] [C2] [C3]    [D1 WS2812B LED]        |
    |                                             |
    |  [AMS1117]                      [Q1 PTT]   |
    |   3.3V LDO        [D3] [D4]     MOSFET     |
    |                   Schottky                  |
    |  o RX In  o TX Out                          |
    |  3.5mm    3.5mm    [J2 RJ45 geschirmt]     |
    |                     (alle Signale)          |
    |  [J3 DC 5-24V]                              |
    |   Schraubklemme                             |
    +---------------------------------------------+

Legende:
  [USB-C]    = USB Type-C Buchse (Oberseite, kurze Kante)
  [RJ45]     = Geschirmte RJ45 Buchse (Unterseite, kurze Kante, THT)
  [J5 EXP]   = Expansion Header 2x5 Pin (Lange Seite, 2.54mm Raster)
  o RX/TX    = 3.5mm TRS Klinkenbuchsen (optional, parallel zu RJ45)
  Antenna    = ESP32-S3 integrierte PCB-Antenne (Freiraum beachten!)
```

### Anordnung der Anschluesse

**Oberseite (Frontpanel-Seite):**
- Links: USB-C Buchse
- Mitte: Reset + Boot Taster
- Rechts: Expansion Header 2x5 Pin

**Unterseite (Funkgeraet-Seite):**
- Mitte: RJ45 Buchse (geschirmt, THT)
- Links: 2x 3.5mm Klinkenbuchsen (RX In, TX Out)
- Links unten: DC-Eingang Schraubklemme

**Seite:**
- WS2812B Status-LED (RGB, programmierbar)

---

## Hauptkomponenten

### ESP32-S3-WROOM-1-N16R8 (U1)

**Warum dieser spezifische Chip?**

| Feature | RadioCore Nutzen |
|---------|------------------|
| **16MB Flash** | OTA-Updates + WAV-Ansagen speichern (Repeater-Modus) |
| **8MB PSRAM** | Audio-Ringbuffer fuer fluessiges Streaming ohne Aussetzer |
| **USB OTG** | Native USB Audio Class 2 + CDC Serial (Composite Device) |
| **Dual Core** | Core 0: Audio-Verarbeitung, Core 1: WiFi/Netzwerk/Steuerung |
| **WiFi 2.4GHz** | Remote-Station Anbindung, Webinterface, OTA-Updates |
| **Bluetooth** | Zukuenftig: Headset-Anbindung, Mobile-App |
| **I2S Master** | Synchrone 24-bit Audio-Verarbeitung mit ES8388 |
| **APLL** | Praezise Master Clock Generierung (12.288MHz fuer I2S) |

**LCSC**: C2913202
**Preis**: ca. 3.71 USD

**Technische Details:**
- Taktfrequenz: 240 MHz (Dual Core Xtensa LX7)
- GPIO: 3.3V Logic, max 40mA pro Pin
- USB: Full Speed (12 Mbps), Device Mode
- WiFi: 802.11 b/g/n, bis +20dBm TX Power
- Stromaufnahme: ca. 80mA (WiFi aktiv), 15mA (Deep Sleep)

### ES8388 Audio Codec (U2)

**Warum dieser Codec?**

| Feature | RadioCore Nutzen |
|---------|------------------|
| **24-bit Aufloesung** | Hohe Dynamik fuer schwache Signale (QRP, EME) |
| **105dB SNR** | Rauscharmer Audio-Pfad, wichtig fuer SSB/CW |
| **I2S Interface** | Synchrone digitale Verbindung zum ESP32, keine Jitter-Probleme |
| **I2C Konfiguration** | Gain, EQ, Filter per Software einstellbar |
| **Line In/Out** | Direkte Verbindung zu Funkgeraet, kein Mic-Preamp noetig |
| **Guenstig** | Nur 0.52 USD, trotzdem Studio-Qualitaet |
| **Bewaehrt** | Weit verbreitet in ESP32-Audioboards (Lyrat, AI-Thinker) |

**LCSC**: C365736
**Preis**: ca. 0.52 USD

**Technische Details:**
- ADC: 24-bit Delta-Sigma, 8kHz - 96kHz
- DAC: 24-bit Delta-Sigma, 8kHz - 96kHz
- THD+N: -80dB @ 1kHz
- Versorgung: 3.3V (AVDD + DVDD getrennt)
- I2C Adresse: 0x10 (AD0 = GND)

### XL1509-5.0E1 Buck Converter (U4)

**Warum dieser Step-Down-Wandler?**

Der XL1509 wandelt die typische Funkgeraet-Versorgung von 13.8V auf stabile 5V fuer das Board.

| Feature | RadioCore Nutzen |
|---------|------------------|
| **Eingangsspannung** | 4.5V - 28V (deckt 12V + 13.8V + 24V Systeme ab) |
| **Ausgangsstrom** | 2A (ausreichend fuer ESP32 + Peripherie) |
| **Fest eingestellt** | 5.0V Variante, keine externen Spannungsteiler noetig |
| **Effizienz** | ca. 85% bei 13.8V → 5V |
| **Guenstig** | 0.11 USD |

**LCSC**: C61063
**Preis**: ca. 0.11 USD

**Schaltung:**
- Induktor L1: 33uH, 2A (LCSC: C339747)
- Output Cap C9: 100uF Low-ESR (LCSC: C176672)
- Freilaufdiode D3: SS34 Schottky (LCSC: C8678)
- Feedback fest verdrahtet fuer 5.0V Ausgang

### AMS1117-3.3 LDO (U3)

**Warum dieser Linearregler?**

Der AMS1117 erzeugt die saubere 3.3V Versorgung fuer ESP32 und ES8388.

| Feature | RadioCore Nutzen |
|---------|------------------|
| **Eingangsspannung** | 5V (vom Buck-Converter oder USB) |
| **Ausgangsstrom** | 1A (ESP32 braucht max 500mA, ES8388 ca. 50mA) |
| **Dropout** | 1.1V (5V Input reicht problemlos fuer 3.3V Out) |
| **Rauschen** | Niedriger als Schaltregler, wichtig fuer Audio |
| **Standard-Bauteil** | Ueberall verfuegbar, kein Obsolescence-Risiko |

**LCSC**: C6186
**Preis**: ca. 0.11 USD

**Schaltung:**
- Input Cap C8: 22uF Tantal (LCSC: C7171)
- Output Cap C7: 22uF Tantal (LCSC: C7171)
- Kuehlung: Kupferflaeche auf Bottom Layer (thermal relief)

---

## Spannungsversorgung

RadioCore unterstuetzt **Dual-Input**: USB-C (5V) oder externe DC-Versorgung (5-24V). Beide Quellen koennen gleichzeitig angeschlossen sein.

### Block-Diagramm

```
Quelle A: USB-C VBUS (5V)
    |
    +--[D3 Schottky SS34]--+
                            |
Quelle B: DC Eingang        |         +----> WS2812B VCC (5V)
(5-24V, z.B. 13.8V)         |         |
    |                       |         |
    +--[XL1509 Buck]-----+  |         |
       4.5-28V -> 5V     |  |         |
                         |  |         |
       [D4 Schottky SS34]+--+--> 5V Rail
                                     |
                              +------+------+
                              |             |
                          WS2812B       AMS1117
                           LED        5V -> 3.3V
                                          |
                                      3.3V Rail
                                          |
                            +-------------+-------------+
                            |                           |
                        ESP32-S3                    ES8388
                        + Logik                   Analog (gefiltert)
```

### Automatische Umschaltung

**Schottky-Dioden D3 und D4** verhindern Rueckspeisung zwischen den Quellen:
- **Hoechste Spannung gewinnt**: Wenn DC 5.0V liefert und USB 5.0V, teilen sie die Last
- **USB allein**: D3 leitet, D4 sperrt → USB versorgt das Board
- **DC allein**: D4 leitet, D3 sperrt → DC versorgt das Board
- **Beide gleichzeitig**: Beide Dioden leiten, Last wird geteilt

**Typische Szenarien:**

| Situation | USB-C | DC-Eingang | Aktive Quelle | Nutzen |
|-----------|-------|------------|---------------|--------|
| Am PC, WSJT-X | 5V | - | USB | Einfach, keine externe Versorgung noetig |
| Am Funkgeraet | - | 13.8V | DC via Buck | USB-Port am PC nicht belastet |
| Entwicklung | 5V | 13.8V | Beide | Board laeuft auch wenn DC-Quelle abfaellt |
| Portable (Akku) | - | 12V | DC via Buck | Batteriebetrieb, kein PC noetig (Repeater) |

### Stromaufnahme

| Betriebsmodus | Typisch | Maximum |
|---------------|---------|---------|
| DIGIMODE (USB aktiv, kein WiFi) | 150 mA @ 5V | 250 mA |
| REPEATER (WiFi aktiv, Standby) | 200 mA @ 5V | 350 mA |
| REMOTE (WiFi + Audio Streaming) | 250 mA @ 5V | 450 mA |
| Deep Sleep (ESP32 aus) | 15 mA @ 5V | 20 mA |

Bei 13.8V DC-Eingang zieht der Buck-Converter:
- 150 mA @ 5V = 0.75W → ca. 60 mA @ 13.8V (unter Beruecksichtigung 85% Effizienz)

---

## Audio-Signalweg

RadioCore verarbeitet **Mono-Audio** (nur linker Kanal) mit **48kHz Samplerate** und **24-bit Aufloesung**.

### RX-Pfad (Funkgeraet → Computer)

```
Funkgeraet Speaker Out
    |
    | (analog, 0.1 - 2 Vpp)
    |
RJ45 Pin 1+2 (verdrilltes Paar, orange)
    |
    +--[Ferrite Bead FB1]--+  (600 Ohm @ 100MHz, EMC-Filter)
                           |
                    [1nF Cap]--GND  (Tiefpass gegen HF-Einstrahlung)
                           |
                   ES8388 LINPUT1 (Line In)
                           |
                      [ADC 24-bit]
                           |
                    I2S Bus (digital)
                           |
                  ESP32-S3 GPIO7 (I2S_DIN)
                           |
                  [Audio Processing]
                           |
                   USB Audio Out (UAC2)
                           |
                          PC
```

**Filterung:**
- **Ferrite Bead**: Blockiert HF-Stoerungen vom Funkgeraet (z.B. TX-Leistung auf Gehaeuse)
- **1nF Kondensator**: Tiefpass mit ca. 160kHz Grenzfrequenz (Audio bis 20kHz passiert, HF wird abgeblockt)

### TX-Pfad (Computer → Funkgeraet)

```
PC (WSJT-X, SDR-Software)
    |
USB Audio In (UAC2)
    |
ESP32-S3 [Audio Processing]
    |
GPIO6 (I2S_DOUT)
    |
I2S Bus (digital)
    |
ES8388 DSDIN
    |
[DAC 24-bit]
    |
ES8388 LOUT1 (Line Out)
    |
[1nF Cap]--GND  (Tiefpass)
    |
[Ferrite Bead FB2]--+
                     |
RJ45 Pin 3+6 (verdrilltes Paar, gruen)
    |
    | (analog, 0.1 - 2 Vpp)
    |
Funkgeraet Mic In / ACC In
```

### I2S Signale (ESP32 ↔ ES8388)

| Signal | ESP32 Pin | ES8388 Pin | Beschreibung |
|--------|-----------|------------|--------------|
| MCLK | GPIO3 | Pin 22 | Master Clock 12.288MHz (256 × 48kHz) |
| BCK | GPIO4 | Pin 2 | Bit Clock (48kHz × 64 = 3.072MHz) |
| WS | GPIO5 | Pin 3 | Word Select / LRCK (48kHz) |
| DOUT | GPIO6 | Pin 4 | Data Out: ESP32 → ES8388 DAC |
| DIN | GPIO7 | Pin 7 | Data In: ES8388 ADC → ESP32 |

**Warum 48kHz?**
- Standard fuer USB Audio Class 2
- Ausreichend fuer Sprache (bis 20kHz Audio-Bandbreite)
- Kompatibel mit allen Digimode-Programmen (WSJT-X, fldigi, direwolf)

**Warum MCLK = 12.288MHz?**
- I2S braucht Master Clock fuer PLL im ES8388
- Typischerweise 256 × Sample Rate = 256 × 48kHz = 12.288MHz
- ESP32-S3 erzeugt diese Frequenz praezise mit der APLL (Audio PLL)
- **Kein externer Quarz noetig!**

### Audio-Pegel

| Signal | Typisch | Maximum | Einstellung |
|--------|---------|---------|-------------|
| RX Audio (Line In) | 0.5 Vpp | 2.0 Vpp | ES8388 ADC Gain: 0 - +24dB |
| TX Audio (Line Out) | 0.5 Vpp | 2.0 Vpp | ES8388 DAC Gain: -96 - 0dB |

Einstellbar per I2C (Software-Befehl ueber USB Serial):
```
SET RX_GAIN 12     # ADC Gain +12dB
SET TX_GAIN -6     # DAC Gain -6dB
```

---

## Steuer-Signale

RadioCore steuert das Funkgeraet ueber **PTT** (Push-to-Talk) und liest **COS** (Carrier Operated Squelch) sowie **CAT** (Computer Aided Transceiver) Kommandos.

### PTT Ausgang (Open Collector)

**Schaltung:**

```
ESP32 GPIO10 --[10K Widerstand]-- Gate
                                   |
                              2N7002 MOSFET
                                   |
RJ45 Pin 4 (PTT) -------------- Drain
                                   |
                                Source -- GND
```

**Funktion:**
- **GPIO10 HIGH** (3.3V): MOSFET leitet → PTT-Pin wird auf GND gezogen → Funkgeraet sendet
- **GPIO10 LOW** (0V): MOSFET sperrt → PTT-Pin offen (hochohmig) → Funkgeraet empfaengt

**Warum Open Collector?**
- Kompatibel mit allen Funkgeraeten (0V = TX, offen = RX)
- Keine Spannung vom Board auf PTT-Leitung (sicher!)
- Standard in kommerziellen Interfaces (Rigblaster, SignaLink)

**Timing:**
- Software-konfigurierbar: PTT-Verzoegerung 0 - 500ms
- Wichtig fuer langsame TX/RX-Umschaltung (Roehrenfunkgeraete)

### COS / Squelch Eingang

**Schaltung:**

```
RJ45 Pin 5 (COS) --[Ferrite Bead]--[100pF Cap]--+--[10K Pull-Up]--3.3V
                                                 |
                                            ESP32 GPIO11
```

**Funktion:**
- **COS offen / HIGH**: Squelch geschlossen (kein Signal)
- **COS auf GND gezogen**: Squelch offen (Traeger erkannt)

**Einsatz:**
- **REPEATER-Modus**: COS triggert Durchschaltung zum Sender
- **VOX Alternative**: Zuverlaessiger als Audio-Level-Detection

### CAT UART (Computer Aided Transceiver)

**Schaltung:**

```
ESP32 GPIO17 (TX) --[Ferrite Bead]-- RJ45 Pin 7 (CAT TX)
ESP32 GPIO18 (RX) --[Ferrite Bead]--[100pF]-- RJ45 Pin 8 (CAT RX)
```

**Funktion:**
- Serielle Schnittstelle zum Funkgeraet (z.B. Icom CI-V, Yaesu CAT)
- Standard-Baudraten: 4800, 9600, 19200, 38400 Baud
- **3.3V TTL-Pegel!** (fuer 5V Funkgeraete: Level-Shifter im Breakout-Adapter noetig)

**Einsatz:**
- **REMOTE-Modus**: CAT-Kommandos werden ueber USB Serial durchgeschleift
- Frequenz-Anzeige, Band-Wechsel, Split-Betrieb
- Kompatibel mit Ham Radio Deluxe, fldigi, WSJT-X

**Beispiel (Icom IC-7300):**
```
PC -> USB Serial -> ESP32 UART -> RJ45 -> IC-7300 CI-V Port
```

---

## Anschluesse

### RJ45 Buchse (geschirmt, THT)

RadioCore nutzt eine **geschirmte RJ45-Buchse** (THT = Through-Hole Technology, manuell loeten) als **Haupt-Interface** zum Funkgeraet.

**Warum RJ45?**
- **Standard-Kabel**: Cat5e/Cat6 STP (Shielded Twisted Pair) fertig kaufen, 1-10m
- **8 Pins**: Ausreichend fuer Audio, PTT, COS, CAT, GND
- **Geschirmt**: Metallgehaeuse + Schirm im Kabel = HF-Schutz
- **Verdrillte Paare**: Audio auf Twisted Pairs = Common Mode Rejection (wichtig bei Sendern!)
- **Robust**: RJ45 haelt mechanisch mehr aus als 3.5mm Klinken

**Pin-Belegung (T568B Twisted Pair Mapping):**

| RJ45 Pin | Farbe (T568B) | Paar | Signal | Richtung |
|----------|---------------|------|--------|----------|
| 1 | Weiss/Orange | 1 | RX Audio + | Funkgeraet → RadioCore |
| 2 | Orange | 1 | RX Audio - | Funkgeraet → RadioCore |
| 3 | Weiss/Gruen | 2 | TX Audio + | RadioCore → Funkgeraet |
| 4 | Blau | 3 | PTT | RadioCore → Funkgeraet (Open Collector) |
| 5 | Weiss/Blau | 3 | COS / Squelch | Funkgeraet → RadioCore |
| 6 | Gruen | 2 | TX Audio - | RadioCore → Funkgeraet |
| 7 | Weiss/Braun | 4 | CAT TX | RadioCore → Funkgeraet (UART) |
| 8 | Braun | 4 | CAT RX | Funkgeraet → RadioCore (UART) |
| Schirm | Metallgehaeuse | - | GND | Gemeinsame Masse |

**Warum Audio auf verdrillten Paaren?**

Wenn das Funkgeraet sendet (z.B. 100W auf 2m), entstehen starke HF-Felder. Ein verdrilltes Kabel-Paar nimmt diese Stoerung **gleichtaktmaessig** auf (beide Adern gleich viel). Der ES8388 verwendet **differentiell-symmetrische** Eingaenge (LINPUT1+ / LINPUT1-), die Gleichtaktstoerungen unterdruecken.

**Beispiel:**
- Unsymmetrisches Kabel (Audio + GND): HF-Stoerung ueberlagert sich dem Audio → Brummen, Klicks
- Twisted Pair (Audio+ / Audio-): HF-Stoerung hebt sich auf → sauberes Audio

**LCSC:** C2683360 (geschirmte RJ45 THT Buchse)
**Preis:** ca. 0.13 USD

### 3.5mm Klinkenbuchsen (optional, parallel zu RJ45)

**RX In:** Tip = Audio, Sleeve = GND
**TX Out:** Tip = Audio, Sleeve = GND

**Warum optional?**
- Manche Nutzer bevorzugen Klinken-Kabel (einfacher zu loeten als RJ45-Breakout)
- **Parallel geschaltet** zur RJ45: Entweder Klinken ODER RJ45 nutzen (nicht beide gleichzeitig!)

**LCSC:** C17701688 (3.5mm TRS SMD Buchse)
**Preis:** ca. 0.02 USD

### USB-C Buchse

**Pinout:**

| USB-C Pin | Signal | Verbindung |
|-----------|--------|------------|
| A4/B4 | VBUS | 5V Rail (via D2 ESD) |
| A7/B7 | D+ | ESP32 GPIO20 |
| A6/B6 | D- | ESP32 GPIO19 |
| A5 | CC1 | 5.1K → GND (Device Mode) |
| B5 | CC2 | 5.1K → GND (Device Mode) |
| A1/B1 | GND | GND |

**CC1/CC2 Widerstaende (5.1K):**
- USB-C benoetigt diese, um dem Host mitzuteilen: "Ich bin ein Device, kein Host"
- Ohne diese Widerstaende: Keine Stromversorgung!

**ESD-Schutz:**
- **USBLC6-2SC6** (D2) schuetzt D+ und D- vor elektrostatischen Entladungen
- Wichtig bei haeufigem Ein-/Ausstecken

**ESP32-S3 Native USB:**
- ESP32-S3 hat **integrierte USB-PHY** (Physical Layer)
- Kein externer USB-Chip (FTDI, CH340) noetig!
- Erscheint als:
  - **USB Audio Class 2** Soundkarte (48kHz, 24-bit)
  - **USB CDC Serial** Port (virtueller COM-Port fuer Steuerung)
  - Beide gleichzeitig (Composite Device)

**LCSC:** C165948 (USB Type-C 2.0 Buchse)
**Preis:** ca. 0.10 USD

### DC-Eingang (Schraubklemme)

**Spezifikation:**
- **Eingangsspannung:** 5 - 24V DC
- **Polung:** + und - beschriftet auf Platine
- **Typische Anwendung:** 13.8V Funkgeraet-Netzteil
- **Schutz:** SS34 Schottky-Diode (Verpolschutz), SMBJ24A TVS-Diode (Ueberspannungsschutz)

**Warum 13.8V?**
- Standard-Spannung in Amateurfunk-Stationen (abgeleitet von 12V Blei-Akkus unter Ladung)
- Netzteil kann auch andere Geraete versorgen (Funkgeraet, TNC, Rotor)

**LCSC:** C474881 (2-Pin Schraubklemme, 5mm Raster)
**Preis:** ca. 0.08 USD

### Expansion Header (2x5 Pin, 2.54mm)

**Pin-Layout:**

```
       Beschriftung auf PCB
    +-----+-----+
  1 | 5V  | GND | 2
    +-----+-----+
  3 | SDA | SCL | 4     <-- I2C Bus (geteilt mit ES8388)
    +-----+-----+
  5 | IO13| IO14| 6     <-- GPIO / Open Collector
    +-----+-----+
  7 | IO15| IO16| 8     <-- GPIO / Open Collector
    +-----+-----+
  9 |3.3V | GND | 10
    +-----+-----+
```

**Signale:**

| Pin | Signal | Beschreibung | Max. Strom |
|-----|--------|--------------|------------|
| 1 | 5V | Vom Buck-Converter oder USB | 500mA gesamt |
| 2, 10 | GND | Gemeinsame Masse | - |
| 3 | SDA | I2C Data (mit 4.7K Pull-Up) | - |
| 4 | SCL | I2C Clock (mit 4.7K Pull-Up) | - |
| 5-8 | GPIO13-16 | 3.3V Logic, Output | 40mA pro Pin |
| 9 | 3.3V | Vom AMS1117 | 200mA frei |

**I2C Bus:**
- Geteilt mit ES8388 (I2C-Adresse 0x10)
- Pull-Ups (4.7K) bereits auf dem Board vorhanden
- Typische Erweiterungen: MCP23017 (GPIO-Expander), ADS1115 (ADC), OLED-Display

**GPIOs 13-16:**
- Direkt vom ESP32-S3, 3.3V Logik
- Fuer Relais/Lasten: Externe MOSFETs (2N7002) auf Erweiterungsboard verwenden!
- Software-konfigurierbar: Input, Output, PWM, Interrupt

**Anwendungsfaelle:**
- Relay Shield (4x oder 8x Relais fuer Antennenumschaltung, PA On/Off)
- Rotor-Interface (I2C + PWM fuer Schrittmotor)
- Band-Decoder (BCD-Ausgabe fuer Antennentuner)
- Sensor-Board (SWR, Temperatur, Spannung, Strom)
- OLED-Display (Status-Anzeige ohne PC)

---

## EMC / HF-Schutz

**Warum ist das wichtig?**

RadioCore sitzt direkt neben oder sogar IM Funkgeraet. Typische HF-Leistungen:
- **Kurzwelle (HF):** 100W PEP SSB, CW, FT8
- **2m/70cm (VHF/UHF):** 50W FM, Packet Radio
- **Repeater:** Dauerstrichbetrieb, hohe Duty Cycle

Ohne Schutz:
- Audio-Verzerrungen (HF ueberlagert sich dem Signal)
- ESP32-Abstuerze (HF koppelt in Stromversorgung ein)
- Fehlerhafte I2C-Kommunikation (ES8388 verliert Konfiguration)

### Schutzmassnahmen auf RadioCore

#### 1. Ferrite Beads (auf ALLEN RJ45-Leitungen)

| Signal | Ferrite Bead | Funktion |
|--------|--------------|----------|
| RX Audio | FB1 (600Ω @ 100MHz) | Blockiert HF auf Audio-Eingang |
| TX Audio | FB2 (600Ω @ 100MHz) | Blockiert HF auf Audio-Ausgang |
| PTT | FB3 (600Ω @ 100MHz) | Schuetzt GPIO10 |
| COS | FB4 (600Ω @ 100MHz) | Schuetzt GPIO11 |
| CAT TX | FB5 (600Ω @ 100MHz) | Schuetzt GPIO17 |
| CAT RX | FB6 (600Ω @ 100MHz) | Schuetzt GPIO18 |
| 5V Rail | FB7 (600Ω @ 100MHz) | Filtert HF auf Stromversorgung |
| 3.3V Analog | FB8 (600Ω @ 100MHz) | Saubere Analog-Versorgung fuer ES8388 |

**LCSC:** C1015 (Murata 0402 Ferrite Bead, 600Ω @ 100MHz)
**Preis:** ca. 0.01 USD

**Wie funktioniert ein Ferrite Bead?**
- Bei niedrigen Frequenzen (Audio, DC): nahezu 0 Ohm → Signal passiert ungestoert
- Bei hohen Frequenzen (HF, >1MHz): hoher Widerstand → HF wird blockiert/daempfung

#### 2. LC-Tiefpass-Filter

**Audio-Leitungen:**
- Ferrite Bead (Induktivitaet) + 1nF Kondensator gegen GND
- Grenzfrequenz: ca. 160kHz (Audio bis 20kHz passiert, HF ab 1MHz wird gedaempft)

**Digital-Leitungen (PTT, COS, CAT):**
- Ferrite Bead + 100pF Kondensator gegen GND
- Grenzfrequenz: ca. 1MHz (digitale Signale bis 115200 Baud passieren)

#### 3. Separate Analog/Digital Ground Planes

**Problem:**
- Digitale Schaltkreise (ESP32, USB, WS2812B) erzeugen Schalt-Noise auf GND
- Wenn dieser Noise die Analog-GND des ES8388 erreicht → Audio-Rauschen!

**Loesung:**
- **Bottom Layer:** Durchgehende GND-Plane
- **Analog-Bereich** (ES8388, Audio-Buchsen): Eigene GND-Zone
- **Verbindung:** Nur an einem Punkt (Sternpunkt unter AMS1117)
- **Digital-Bereich** (ESP32, USB): Eigene GND-Zone
- Beide Zonen sind elektrisch verbunden, aber Noise-Strom fliesst nicht durch Analog-GND

#### 4. Kein Kupfer unter ESP32-S3 Antenne

**Problem:**
- ESP32-S3 WROOM-1 hat integrierte PCB-Antenne (letzte 5mm des Moduls)
- Kupfer unter oder neben der Antenne → Fehlanpassung, schlechte WiFi-Reichweite

**Loesung:**
- **Top + Bottom Layer:** Freiraum 10mm x 5mm unter/neben Antenne
- Antenne ragt idealerweise ueber den PCB-Rand hinaus
- Keine Bauteile in diesem Bereich platzieren!

#### 5. Ground Stitching Vias

**Funktion:**
- Vias (Durchkontaktierungen) verbinden Top- und Bottom-GND-Plane
- Bilden einen "HF-Zaun" um empfindliche Bereiche (ES8388)
- Verhindern, dass HF unter Bauteilen hindurch kriecht

**Platzierung:**
- Rund um ES8388: Vias im 2mm-Raster
- Zwischen Digital- und Analog-Bereich: Doppelreihe Vias
- Unter Ferrite Beads: Via direkt daneben (kurzer Pfad zu GND)

#### 6. Geschirmte RJ45-Buchse

**Funktion:**
- Metallgehaeuse der RJ45-Buchse ist elektrisch mit GND verbunden
- Kabelschirm (bei STP Cat5-Kabel) wird automatisch geerdet
- Blockiert HF-Einstrahlung ueber das Kabel

**Wichtig:**
- **STP-Kabel verwenden!** (Shielded Twisted Pair, nicht UTP)
- Schirm muss an beiden Enden geerdet sein (RadioCore + Funkgeraet)

#### 7. TVS-Diode auf DC-Eingang

**Bauteil:** SMBJ24A (24V Transient Voltage Suppressor)

**Funktion:**
- Schuetzt vor Ueberspannungsspitzen (z.B. Blitzeinschlag in der Naehe, Schaltspitzen von Linearnetzteilen)
- Bei Spannung > 24V: Diode wird leitend → leitet Ueberspannung gegen GND ab
- ESP32 und ES8388 bleiben unbeschaedigt

**LCSC:** C82417
**Preis:** ca. 0.12 USD

---

## Stueckliste (BOM)

Komplette Bauteilliste fuer RadioCore v1.0 (1 Board). Bei JLCPCB-Bestellung fuer 5 Stueck: Mengen × 5.

### ICs / Aktive Bauteile

| Ref | Bauteil | Package | LCSC | Menge | Preis (USD) | Gesamt |
|-----|---------|---------|------|-------|-------------|--------|
| U1 | ESP32-S3-WROOM-1-N16R8 | SMD Modul | C2913202 | 1 | 3.71 | 3.71 |
| U2 | ES8388 | QFN-28 | C365736 | 1 | 0.52 | 0.52 |
| U3 | AMS1117-3.3V | SOT-223 | C6186 | 1 | 0.11 | 0.11 |
| U4 | XL1509-5.0E1 | SOP-8 | C61063 | 1 | 0.11 | 0.11 |
| Q1 | 2N7002 (N-MOSFET) | SOT-23 | C8545 | 1 | 0.01 | 0.01 |
| D1 | WS2812B RGB LED | SMD 5050 | C2761795 | 1 | 0.04 | 0.04 |
| D2 | USBLC6-2SC6 (ESD) | SOT-23-6 | C7519 | 1 | 0.15 | 0.15 |
| D3, D4 | SS34 Schottky | DO-214AC | C8678 | 2 | 0.04 | 0.08 |

### Passive Bauteile (Widerstaende 0402, 1%)

| Ref | Wert | Menge | LCSC | Preis | Gesamt |
|-----|------|-------|------|-------|--------|
| R1, R2 | 5.1kΩ | 2 | C25905 | 0.001 | 0.002 |
| R3, R4 | 22Ω | 2 | C25092 | 0.001 | 0.002 |
| R5, R6 | 10kΩ | 2 | C25744 | 0.001 | 0.002 |
| R7 | 10kΩ | 1 | C25744 | 0.001 | 0.001 |
| R8 | 4.7kΩ | 1 | C25900 | 0.001 | 0.001 |
| R9, R10 | 4.7kΩ (I2C Pull-Up) | 2 | C25900 | 0.001 | 0.002 |

### Passive Bauteile (Kondensatoren)

| Ref | Wert | Package | LCSC | Menge | Preis | Gesamt |
|-----|------|---------|------|-------|-------|--------|
| C1, C2 | 100nF | 0402 | C1525 | 2 | 0.001 | 0.002 |
| C3 | 10µF | 0805 | C15850 | 1 | 0.01 | 0.01 |
| C4, C5 | 100nF | 0402 | C1525 | 2 | 0.001 | 0.002 |
| C6 | 10µF | 0805 | C15850 | 1 | 0.01 | 0.01 |
| C7, C8 | 22µF | 0805 | C45783 | 2 | 0.03 | 0.06 |
| C9 | 100µF | 1206 | C176672 | 1 | 0.08 | 0.08 |
| C10 | 100nF | 0402 | C1525 | 1 | 0.001 | 0.001 |
| C11 | 100nF | 0402 | C1525 | 1 | 0.001 | 0.001 |
| C12 | 100nF | 0402 | C1525 | 1 | 0.001 | 0.001 |
| C_LP1, C_LP2 | 1nF (Audio LP) | 0402 | C1523 | 2 | 0.001 | 0.002 |
| C_LP3-5 | 100pF (Digital LP) | 0402 | C1546 | 3 | 0.001 | 0.003 |

### Induktivitaeten / Ferrite Beads

| Ref | Wert | Package | LCSC | Menge | Preis | Gesamt |
|-----|------|---------|------|-------|-------|--------|
| L1 | 33µH (Buck) | SMD 1210 | C339747 | 1 | 0.06 | 0.06 |
| FB1-8 | 600Ω @ 100MHz | 0402 | C1015 | 8 | 0.01 | 0.08 |

### Steckverbinder (SMD, bei JLCPCB bestueckt)

| Ref | Bauteil | LCSC | Menge | Preis | Gesamt |
|-----|---------|------|-------|-------|--------|
| J1 | USB Type-C Buchse | C165948 | 1 | 0.10 | 0.10 |
| J4, J5 | 3.5mm TRS Klinkenbuchse | C17701688 | 2 | 0.02 | 0.04 |
| SW1, SW2 | Taster 6x6mm | C318884 | 2 | 0.03 | 0.06 |

### Steckverbinder (THT, manuell loeten)

| Ref | Bauteil | LCSC | Menge | Preis | Gesamt |
|-----|---------|------|-------|-------|--------|
| J2 | RJ45 Buchse geschirmt THT | C2683360 | 1 | 0.13 | 0.13 |
| J3 | Schraubklemme 2P 5mm | C474881 | 1 | 0.08 | 0.08 |
| J6 | Pin Header 2x5 2.54mm | C492404 | 1 | 0.05 | 0.05 |

### **Gesamt-BOM: ca. 5.62 USD pro Board** (SMD-Bauteile bei JLCPCB)

---

## PCB Layout Richtlinien

Diese Regeln sind fuer EasyEDA gedacht, gelten aber auch fuer KiCad, Altium, etc.

### Board-Parameter

| Parameter | Wert |
|-----------|------|
| Abmessungen | 65 x 45 mm |
| Lagen | 2 (Top + Bottom) |
| Kupferdicke | 1oz (35µm) |
| Min. Leiterbahnbreite | 0.2mm (Signale), 0.3mm (Strom) |
| Min. Abstand | 0.2mm |
| Via-Durchmesser | 0.6mm Pad, 0.3mm Bohrung |
| Loetstopp-Maske | Gruen (Standard bei JLCPCB) |
| Oberflaechenfinish | HASL bleifrei (oder ENIG fuer bessere Loetbarkeit) |
| Montageloecher | 4x M3 (3.2mm Bohrung), Ecken 3mm vom Rand |

### Bauteil-Platzierung (Top Layer)

**Stromversorgung (oben links):**
- U3 (AMS1117) mit C7, C8 direkt daneben
- U4 (XL1509) mit L1, D3, C9, C10 → kompakte Gruppe (kurze Stromschleifen!)
- D4 (Schottky) neben J3 (DC-Eingang)

**Mikrocontroller (Mitte):**
- U1 (ESP32-S3) zentriert
- Antenne zeigt zum Rand (oder ragt darueber hinaus)
- **KEIN Kupfer unter Antenne!** (weder Top noch Bottom, 10mm Freiraum)
- C1, C2, C3 direkt neben U1 (kurze Wege zu VDD-Pins)
- SW1 (BOOT), SW2 (RESET) erreichbar

**Audio-Codec (rechts neben ESP32):**
- U2 (ES8388) so nah wie moeglich an U1 (kurze I2S-Leitungen!)
- C4, C5, C6 direkt an U2 (DVDD, AVDD Pins)
- FB1, FB2 zwischen U2 und J2 (RJ45) platzieren

**Anschluesse:**
- J1 (USB-C): Oberseite, kurze Kante, mittig
- J2 (RJ45): Unterseite, kurze Kante, mittig (THT = Bottom Side)
- J3 (DC): Unterseite, Ecke
- J4, J5 (3.5mm Klinken): Unterseite, neben RJ45
- J6 (Expansion Header): Lange Seite, zugaenglich fuer Shield-Boards

**LED + Sonstiges:**
- D1 (WS2812B): Sichtbar, nahe Kante
- Q1 (PTT MOSFET): Zwischen U1 und J2

### Leiterbahn-Routing

**1. Stromversorgung zuerst (breite Bahnen):**
- 5V Rail: 0.5mm Breite (max 500mA)
- 3.3V Rail: 0.3mm Breite (max 300mA)
- GND: Bottom Layer Kupferflaeche (Ground Plane)

**2. USB-Datenleitungen (differentiell):**
- D+ und D- als Paar routen
- Gleiche Laenge (max 5mm Unterschied)
- 90 Ohm Impedanz (in EasyEDA: 0.2mm Leiterbahn, 0.3mm Abstand, auf 1.6mm FR4)
- So kurz wie moeglich (< 20mm von J1 zu U1)
- Keine Vias wenn moeglich (sonst gleich viele auf beiden Leitungen)

**3. I2S-Bus (synchron, kurz halten):**
- MCLK, BCK, WS, DOUT, DIN von U1 zu U2
- Alle 5 Leitungen gleichmaessig routen (gleiche Laenge +/- 2mm)
- 0.2mm Breite
- Top Layer, direkte Wege
- Keine langen parallelen Abschnitte neben Audio-Leitungen

**4. Audio-Leitungen (analog, kritisch):**
- RX Audio: J2 → FB1 → C_LP → U2 LINPUT1
- TX Audio: U2 LOUT1 → FB2 → C_LP → J2
- RX und TX **niemals parallel fuehren!** (Uebersprechen vermeiden)
- 0.2mm Breite
- Kurze Wege
- Ferrite Beads + LP-Kondensatoren direkt am RJ45-Pin

**5. I2C-Bus (unkritisch):**
- SDA, SCL von U1 zu U2 + J6 (Expansion Header)
- 0.2mm Breite
- Darf kreuzen, Vias sind OK

**6. Digital-Signale (PTT, COS, CAT):**
- 0.2mm Breite
- Ferrite Beads direkt am RJ45-Pin
- LP-Kondensatoren direkt nach FB gegen GND

### Ground Plane (Bottom Layer)

**Setup:**
1. Gesamte Bottom-Flaeche mit Kupfer fuellen
2. Netz: **GND** zuweisen
3. Thermal Relief bei Pads (Kreuzform, sonst schwer loetbar)
4. Kupferflaeche neu berechnen nach jeder Aenderung

**Analog/Digital Trennung:**
- Analog-Zone (ES8388, Audio-Buchsen): Eigene GND-Flaeche
- Digital-Zone (ESP32, USB, LED): Eigene GND-Flaeche
- Verbindung: Schmaler Steg (1mm breit) unter U3 (AMS1117) → Sternpunkt
- In EasyEDA: Mit "Cutout" (Ausschnitt) in Copper Area arbeiten

**Ground Stitching Vias:**
- Rund um U2 (ES8388): Vias im 2mm-Raster (HF-Zaun)
- Zwischen Analog/Digital: Doppelreihe Vias
- Unter jedem Abblock-Kondensator: Via direkt am GND-Pad
- Gesamt: ca. 30-50 Vias auf dem Board

### Antenne (ESP32-S3 WROOM-1)

**Kritisch!** Falsche Platzierung → schlechte WiFi-Reichweite

**Regeln:**
1. Antenne zeigt zum PCB-Rand (oder darueber hinaus)
2. **Kein Kupfer** (weder Top noch Bottom) unter der Antenne (letzte 5-10mm des Moduls)
3. **Kein Kupfer** neben der Antenne (5mm Freiraum seitlich)
4. Keine Bauteile in diesem Bereich
5. In EasyEDA: "Keepout Area" zeichnen

### Decoupling Capacitors (Abblockkondensatoren)

**Platzierung:**
- **Direkt** am zugehoerigen IC-Pin (< 5mm Abstand)
- Via zu GND so nah wie moeglich am Kondensator
- Nicht irgendwo auf dem Board verteilen!

**Beispiel:**
- C1 (100nF) direkt an U1 Pin 2 (VDD)
- Via direkt neben C1 GND-Pad → Bottom GND Plane

### Ferrite Beads

- Direkt am RJ45-Connector platzieren (zwischen Pin und Leiterbahn)
- Kurze Verbindung zur nachfolgenden Schaltung

---

## Breakout-Adapter (Funkgeraet-Seite)

RadioCore nutzt RJ45 als Haupt-Interface. Am Funkgeraet-Ende des Kabels wird ein **Breakout-Adapter** benoetigt, der das RJ45-Kabel auf die spezifischen Anschluesse des Funkgeraetes aufteilt.

### Konzept

```
RadioCore Board         Standard Cat5 STP Kabel         Breakout-Adapter
┌──────────────┐         (fertig kaufen, 1-10m)        ┌──────────────┐
│              │                                        │              │
│   [RJ45] ════╪════════════════════════════════════════╪═ [RJ45]      │
│              │         geschirmt                      │    |         │
└──────────────┘                                        │    +-- o RX  │ 3.5mm
                                                        │    +-- o TX  │ 3.5mm
                                                        │    +-- o CAT │ 3.5mm
                                                        │    +-- [□□]  │ Klemme
                                                        │     PTT COS  │
                                                        └──────────────┘
                                                         30 x 20 mm PCB
```

### Option 1: Breakout-PCB (empfohlen)

Kleine Platine (30 x 20 mm) mit:
- 1x RJ45 Buchse (geschirmt, THT)
- 3x 3.5mm Klinkenbuchse (RX, TX, CAT)
- 1x Schraubklemme 2x2 Pin (PTT, COS, GND, GND)

**BOM Breakout-PCB:**

| Bauteil | LCSC | Menge | Preis |
|---------|------|-------|-------|
| RJ45 Buchse THT geschirmt | C2683360 | 1 | 0.13 USD |
| 3.5mm Klinkenbuchse TRS | C145819 | 3 | 0.15 USD |
| Schraubklemme 2x2P | C474881 | 1 | 0.08 USD |
| PCB 30x20mm | - | 1 | 0.50 USD |
| **Gesamt** | | | **ca. 1.00 USD** |

Keine aktiven Bauteile → jeder kann es loeten!

**Schaltung:**
- RJ45 Pin 1 → 3.5mm Buchse 1 Tip (RX Audio)
- RJ45 Pin 2 → 3.5mm Buchse 1 Sleeve (GND)
- RJ45 Pin 3 → 3.5mm Buchse 2 Tip (TX Audio)
- RJ45 Pin 6 → 3.5mm Buchse 2 Sleeve (GND)
- RJ45 Pin 7 → 3.5mm Buchse 3 Tip (CAT TX)
- RJ45 Pin 8 → 3.5mm Buchse 3 Sleeve (CAT RX)
- RJ45 Pin 4 → Schraubklemme Pin 1 (PTT)
- RJ45 Pin 5 → Schraubklemme Pin 2 (COS)
- RJ45 Schirm → Schraubklemme Pin 3+4 (GND)

### Option 2: DIY (ohne PCB)

Cat5-Kabel am Ende abmanteln, Paare aufloesen, direkt Stecker anloeten:

```
RJ45 Pinout (T568B Farbcode):

Pin 1 (Weiss/Orange) ──┐
Pin 2 (Orange)         ├── RX Audio → 3.5mm Tip + Sleeve
                       │
Pin 3 (Weiss/Gruen)  ──┤
Pin 6 (Gruen)          ├── TX Audio → 3.5mm Tip + Sleeve
                       │
Pin 7 (Weiss/Braun)  ──┤
Pin 8 (Braun)          ├── CAT UART → 3.5mm Tip + Sleeve (oder direkt an Funkgeraet)
                       │
Pin 4 (Blau)         ──── PTT → Anschlusskabel / Klemme
Pin 5 (Weiss/Blau)   ──── COS → Anschlusskabel / Klemme

Schirm ───────────────── GND
```

### Funkgeraet-spezifische Adapter

#### Yaesu FT-891 / FT-991A (Mini-DIN 6 DATA-Buchse)

```
Breakout RX Audio → Mini-DIN Pin 2 (1200bps / PKT)
Breakout TX Audio → Mini-DIN Pin 1 (PKD)
Breakout PTT     → Mini-DIN Pin 3 (PTT)
Breakout GND     → Mini-DIN Pin 5 (GND)
Breakout CAT     → Separates 3.5mm Kabel (CAT Buchse am Funkgeraet)
```

#### Icom IC-7300 (ACC2 / USB)

```
Breakout RX Audio → 3.5mm Klinke (ACC2 AF Out oder Phone Out)
Breakout TX Audio → 3.5mm Klinke (ACC2 Mod In)
Breakout PTT     → ACC2 Pin 3 (SEND) oder RCA Buchse (SEND)
CAT: IC-7300 hat USB-Serial direkt (nicht ueber RadioCore!)
```

#### Baofeng / Handfunke (K1 Connector = 3.5mm + 2.5mm)

```
Breakout RX Audio → 3.5mm Klinke Tip (Speaker)
Breakout TX Audio → 2.5mm Klinke Tip (Mic)
Breakout PTT     → 2.5mm Klinke Ring (PTT)
Breakout GND     → Gemeinsam Sleeve
```

#### Universal (offene Kabelenden)

Einfach Cat5-Kabel aufschneiden, Adern nach Farbcode verloeten:
- Orange-Paar → RX Audio
- Gruen-Paar → TX Audio
- Blau-Paar → PTT + COS
- Braun-Paar → CAT TX + RX
- Schirm → GND

---

## Erweiterungsboards (Zubehoer)

RadioCore hat einen 2x5 Pin Expansion Header. Darauf koennen Shield-Boards gesteckt werden.

### 1. Relay Shield 4x (19 EUR)

**Features:**
- 4x Relais-Ausgaenge (230V / 10A Wechselkontakt)
- Ansteuerung ueber GPIO13-16 (direkt vom ESP32)
- 2N7002 MOSFET + Freilaufdiode pro Relais
- 5V Relais-Spulen (vom RadioCore 5V Rail versorgt)

**Anwendung:**
- PA Ein/Aus (sequenziell schalten vor TX)
- Antennenumschaltung (2x Relais = 3 Antennen via Wechselkontakte)
- Bandfilter-Umschaltung
- Zubehoer schalten (Tuner, Rotor-Netzteil)

**Schaltung pro Kanal:**
```
ESP32 GPIO13 --[10K]-- Gate 2N7002
                        |
                    Drain -- Relais-Spule -- 5V
                        |
                     Source -- GND
                        |
                    [1N4148 Diode] (Freilaufschutz)
```

**LCSC BOM:**
- Relais: C35449 (5V, 10A, SPDT) × 4 = 1.20 USD
- 2N7002: C8545 × 4 = 0.04 USD
- Dioden 1N4148: C81598 × 4 = 0.04 USD
- Schraubklemmen: 4x 3-Pin = 0.40 USD
- PCB 65x45mm = 0.50 USD
- **Gesamt: ca. 2.50 USD Selbstkosten → 19 EUR VK**

### 2. Relay Shield 8x (29 EUR)

**Features:**
- 8x Relais-Ausgaenge
- I2C GPIO-Expander MCP23017 (16 GPIO, davon 8 fuer Relais)
- Adresse per Jumper einstellbar (bis zu 8 Boards = 64 Relais!)

**Anwendung:**
- Antennen-Matrix (8x8 = 64 Kombinationen)
- Band-Decoder mit BCD-Ausgabe
- Komplexe Station-Automation

**Schaltung:**
```
ESP32 I2C Bus (SDA/SCL)
    |
MCP23017 (I2C Adresse 0x20)
    |
8x GPIO Ausgaenge --> 8x 2N7002 --> 8x Relais
```

**LCSC BOM:**
- MCP23017: C47546 = 0.80 USD
- Relais × 8 = 2.40 USD
- 2N7002 × 8 = 0.08 USD
- Dioden × 8 = 0.08 USD
- Schraubklemmen × 8 = 0.80 USD
- PCB = 0.50 USD
- **Gesamt: ca. 5.00 USD → 29 EUR VK**

### 3. Rotor Interface (29 EUR)

**Features:**
- DC-Motor-Steuerung (Yaesu G-5500 kompatibel)
- I2C H-Bruecke (L298N oder DRV8833)
- Position-Feedback ueber ADC (Potentiometer)
- Software: Hamlib rotctld Protokoll

**Anwendung:**
- Antennenrotor fuer Satellit, EME, Contesting
- Az/El Steuerung (2-Achsen)
- Remote-Steuerung ueber Netzwerk

**Schaltung:**
```
ESP32 I2C --> MCP23017 --> L298N H-Bridge --> Motor (CW/CCW)
ESP32 ADC <-- Spannungsteiler <-- Rotor Potentiometer (0-5V)
```

### 4. Sensor Board (19 EUR)

**Features:**
- 4x 16-bit ADC Eingaenge (ADS1115 ueber I2C)
- Spannungsteiler fuer 0-30V Messung
- Stromsensor (INA219 oder Shunt + OpAmp)
- Temperatursensor (DS18B20 oder DHT22)

**Anwendung:**
- SWR-Messung (Vorwaerts/Rueckwaerts-Spannung)
- PA-Temperatur ueberwachen
- Versorgungsspannung loggen
- PA-Strom messen (Schutz vor Ueberlast)

**LCSC BOM:**
- ADS1115: C37593 = 1.20 USD
- INA219: C424048 = 0.80 USD
- DS18B20: C376006 = 0.50 USD
- Widerstaende, Kondensatoren = 0.20 USD
- PCB = 0.50 USD
- **Gesamt: ca. 3.50 USD → 19 EUR VK**

---

## Firmware-Modi

RadioCore unterstuetzt drei Betriebsmodi, umschaltbar ueber USB Serial-Befehl:

```
MODE DIGIMODE   # Standard-Modus
MODE REPEATER   # Relais-Steuerung
MODE REMOTE     # Remote Station
```

### DIGIMODE (Standard)

**Beschreibung:**
Transparente USB Audio Bridge. RadioCore verhält sich wie eine externe Soundkarte.

**Funktion:**
- ESP32 → USB Composite Device:
  - **USB Audio Class 2** (UAC2): 48kHz, 24-bit, Stereo (nur Left aktiv)
  - **USB CDC Serial** (virtueller COM-Port): 115200 Baud
- Audio-Durchleitung: RX → ADC → I2S → USB | USB → I2S → DAC → TX
- PTT-Steuerung: Per USB Serial-Befehl `PTT ON` / `PTT OFF`

**Anwendung:**
- **FT8 / FT4** (WSJT-X): Soundkarte = RadioCore, PTT via CAT oder Serial
- **SSTV** (MMSSTV, QSSTV): Audio In/Out
- **Packet Radio** (Direwolf): TNC-Modus, APRS, APRS-IS Gateway
- **RTTY, PSK31** (fldigi): Soundkarte-Modus
- **SDR-Software** (SDR++, HDSDR): IQ-Audio (zukuenftig)

**Vorteil:**
- Keine Treiber noetig (UAC2 = Standard in Windows/Linux/macOS)
- Funktioniert mit jeder Digimode-Software
- Niedriges Latency (<10ms Audio Round-Trip)

### REPEATER (Standalone)

**Beschreibung:**
Eigenstaendige Repeater-Steuerung ohne PC. COS triggert PTT, Timer, Ansagen.

**Funktion:**
- COS (GPIO11) ueberwacht Squelch vom Empfaenger
- Bei COS aktiv:
  1. Audio RX → TX durchschalten (I2S Loopback)
  2. PTT (GPIO10) aktivieren nach konfigurierbarer Verzoegerung (z.B. 200ms)
  3. Timer starten (max. 3 Minuten, dann Timeout)
- Nach COS inaktiv:
  1. Nachlaufzeit (z.B. 5 Sekunden), dann PTT aus
  2. Optional: Kennung senden (WAV-Datei aus Flash, alle 10 Minuten)
- WiFi Webinterface:
  - Config (Timer, Verzoegerungen, Kennung)
  - Status (Letzte QSOs, Uptime, Audio-Pegel)
  - Fernsteuerung (PTT manuell, Test-Ansage)

**Anwendung:**
- **FM-Repeater** (2m/70cm)
- **Echolink-Node** (ueber WiFi/Ethernet Bridge)
- **Wetterstation** (Ansage alle 10 Minuten)
- **APRS-Digipeater** (mit APRS-Decoding im ESP32)

**Vorteil:**
- Kein PC noetig
- Geringer Stromverbrauch (< 2W)
- WAV-Ansagen direkt im Flash (bis 10MB = ca. 3 Minuten Audio)

### REMOTE (Remote Station)

**Beschreibung:**
Fernsteuerung eines Funkgeraetes ueber Internet. Audio + CAT werden per WebSocket/QUIC gestreamed.

**Funktion:**
- ESP32 verbindet sich mit Cloud-Server (z.B. remoteham.io)
- Audio RX → Opus Codec (48kHz → 16kbit/s) → WebSocket → Internet
- Audio TX ← Opus Decode ← WebSocket ← Internet
- CAT UART: Bidirektionales Forwarding ueber WebSocket
- Optionale Verschluesselung (TLS/DTLS)

**Anwendung:**
- **Fernbedienung** des Heim-Shacks vom Urlaub aus
- **Club-Station** (mehrere OPs remote bedienen eine Station)
- **Satelliten-Betrieb** (Rotor + CAT + Audio remote)

**Vorteil:**
- Geringe Bandbreite (Opus 16kbit/s = 12 MB/h Audio)
- Niedrige Latency (QUIC Protokoll, <100ms bei guter Verbindung)
- Arbeitet hinter NAT (kein Port-Forwarding noetig)

---

## Herstellung bei JLCPCB

Schritt-fuer-Schritt: Von EasyEDA zur fertigen Platine.

### Schritt 1: EasyEDA Projekt erstellen

1. Account auf **easyeda.com** anlegen (kostenlos)
2. **File → New → Project**: `RadioCore v1.0`
3. **New → Schematic** anlegen
4. Bauteile aus LCSC-Library importieren (siehe [Stueckliste](#stueckliste-bom))
5. Schaltplan zeichnen (siehe [EASYEDA_GUIDE.md](/tmp/radiocore/hardware/EASYEDA_GUIDE.md))
6. **Design → Electrical Rules Check (ERC)** durchfuehren → fehlerfrei!

### Schritt 2: PCB Layout

1. **Design → Convert Schematic to PCB**
2. Board-Umriss zeichnen: 65 x 45 mm
3. Bauteile platzieren (siehe [PCB Layout Richtlinien](#pcb-layout-richtlinien))
4. Leiterbahnen routen
5. Ground Plane auf Bottom Layer
6. **Design → Design Rules Check (DRC)** durchfuehren → fehlerfrei!

### Schritt 3: Gerber + BOM + Pick&Place generieren

1. Im PCB-Editor: **Fabrication → JLCPCB Order**
2. EasyEDA generiert automatisch:
   - **Gerber-Dateien** (PCB-Layout fuer Fertigung)
   - **BOM (Bill of Materials)** (CSV mit LCSC-Nummern)
   - **CPL (Component Placement List)** (Pick&Place Koordinaten)
3. Du wirst zu **jlcpcb.com** weitergeleitet (Login mit JLCPCB-Account)

### Schritt 4: JLCPCB PCB-Optionen

Einstellungen fuer RadioCore:

| Option | Einstellung | Grund |
|--------|-------------|-------|
| Base Material | FR-4 | Standard |
| Layers | 2 | Top + Bottom |
| Dimensions | 65 x 45 mm | Automatisch erkannt |
| PCB Qty | 5 | Minimum bei JLCPCB, guenstigster Preis |
| PCB Thickness | 1.6mm | Standard, mechanisch stabil |
| PCB Color | Green | Guenstigste Option (0 USD Aufpreis) |
| Surface Finish | HASL (lead-free) | Standard, gut loetbar |
| Outer Copper Weight | 1oz (35µm) | Ausreichend fuer 500mA Stroeme |
| Gold Fingers | No | Nicht noetig |
| Castellated Holes | No | Nicht noetig |
| Remove Order Number | No | JLCPCB druckt kleine Nummer auf Board (kostenlos) |

**Preis:** ca. 2.00 USD fuer 5 Stueck

### Schritt 5: SMT Assembly aktivieren

1. **SMT Assembly** Checkbox aktivieren
2. **Assemble Top Side** (RadioCore hat nur Top-Bestueckung)
3. **Tooling holes**: "Added by JLCPCB" (automatisch)
4. **Confirm** klicken

### Schritt 6: BOM + CPL hochladen

1. **BOM-Datei hochladen** (von EasyEDA generiert, .csv Format)
2. JLCPCB matched automatisch LCSC-Nummern
3. **CPL-Datei hochladen** (Pick&Place Koordinaten, .csv Format)
4. Vorschau erscheint: Alle Bauteile auf dem Board angezeigt
5. **Rotation pruefen!** Falls Bauteile um 90° verdreht angezeigt werden:
   - In der BOM-Tabelle: "Rotation" Spalte anpassen (0°, 90°, 180°, 270°)
   - U1 (ESP32-S3 WROOM-1): Typischerweise 0° oder 270°
   - U2 (ES8388 QFN-28): 0°
   - Passiv-Bauteile (0402): Meistens 0°

### Schritt 7: Bauteil-Verfuegbarkeit pruefen

JLCPCB zeigt an, welche Bauteile auf Lager sind:

| Kategorie | Aufpreis | Verfuegbarkeit |
|-----------|----------|----------------|
| **Basic Parts** | 0 USD | Immer auf Lager, schnelle Bestueckung |
| **Extended Parts** | 3 USD pro einzigartigem Bauteil | Meistens auf Lager, 1-2 Tage Verzoegerung |
| **Out of Stock** | - | Nicht verfuegbar, alternatives Bauteil waehlen! |

**RadioCore Extended Parts:**
- ESP32-S3-WROOM-1-N16R8 (C2913202): Extended
- ES8388 (C365736): Extended
- WS2812B (C2761795): Extended

**Gesamt Extended Parts Gebuehr:** ca. 3 × 3 USD = 9 USD

**Falls ein Bauteil "Out of Stock":**
1. Alternatives Bauteil mit gleicher Funktion suchen (z.B. AMS1117 von anderem Hersteller)
2. LCSC-Nummer in BOM aendern
3. Neu hochladen

### Schritt 8: Manuell nachzuloetende Bauteile

JLCPCB bestueckt **nur SMD-Bauteile**. Folgende Bauteile muessen von Hand geloetet werden:

| Bauteil | Grund |
|---------|-------|
| J2 (RJ45 geschirmt) | THT (Through-Hole), JLCPCB macht kein THT Assembly |
| J3 (Schraubklemme) | THT |
| J6 (2x5 Pin Header) | THT |

**Was tun?**
1. Diese Bauteile separat bei LCSC mitbestellen (im gleichen Warenkorb!)
2. Nach Erhalt der Platinen: Von Hand nachloeten
3. Werkzeug: Loetkolben 40W, Loetzinn 0.8mm bleifrei, Flussmittel

### Schritt 9: Kosten-Uebersicht

| Position | Preis (USD) |
|----------|-------------|
| PCB-Fertigung (5 Stk) | 2.00 |
| SMT Assembly Setup Fee | 8.00 |
| SMT Assembly (pro Board, 5 Stk) | 2.50 |
| Extended Parts Gebuehr (3 Stueck) | 9.00 |
| Bauteile (BOM, 5 Boards) | 27.50 |
| **Zwischensumme** | **49.00** |
| Versand (DHL Express, 5-7 Tage) | 20.00 |
| **Gesamt** | **ca. 69 USD** |

**Pro Board:** 69 USD / 5 = **13.80 USD = ca. 12.70 EUR**

### Schritt 10: Bestellung abschliessen

1. Alle Optionen nochmal pruefen
2. Vorschau-Bild der Bestueckung anschauen (Rotation OK?)
3. **Add to Cart**
4. Ggf. THT-Bauteile separat im LCSC-Shop in den Warenkorb legen
5. **Checkout**
6. Versandart waehlen:
   - **Standard (7-15 Tage):** ca. 5-10 USD
   - **DHL Express (5-7 Tage):** ca. 20 USD
7. Bezahlen (PayPal, Kreditkarte)
8. E-Mail-Bestaetigung abwarten
9. Nach 1-2 Tagen: E-Mail mit Produktions-Fortschritt
10. Nach 3-5 Tagen: Versand-E-Mail mit Tracking-Nummer

### Schritt 11: Qualitaets-Kontrolle nach Erhalt

1. Platinen visuell pruefen:
   - Alle SMD-Bauteile bestueckt?
   - Lotstellen sauber (keine Bruecken)?
   - ESP32-S3 Modul korrekt ausgerichtet?
2. Durchgangspruefer:
   - 5V Rail → GND: **offen** (kein Kurzschluss!)
   - 3.3V Rail → GND: **offen**
   - USB VBUS → GND: **offen**
3. RJ45, Schraubklemme, Pin-Header nachloeten
4. Erste Inbetriebnahme:
   - USB-C anschliessen
   - WS2812B LED leuchtet?
   - PC erkennt USB-Geraet? (`lsusb` unter Linux)
   - Firmware flashen (siehe firmware/README.md)

### Zeitplan

| Schritt | Dauer |
|---------|-------|
| EasyEDA Design | 1-2 Tage (Erstnutzer), 2-4h (erfahrener User) |
| JLCPCB Review + Fertigung | 3-5 Tage |
| Versand (DHL Express) | 5-7 Tage |
| Nachloeten (THT-Bauteile) | 30 Minuten |
| **Gesamt** | **ca. 10-14 Tage** |

---

## Checkliste vor der Bestellung

- [ ] ERC (Electrical Rules Check) im Schematic: **fehlerfrei**
- [ ] DRC (Design Rules Check) im PCB: **fehlerfrei**
- [ ] Alle Netzlabels korrekt (keine schwebenden Leitungen)
- [ ] Abblock-Kondensatoren direkt an IC-Pins platziert
- [ ] ESP32-S3 Antenne: **Kein Kupfer unter/neben Antenne** (Top + Bottom frei!)
- [ ] Ground Plane auf Bottom Layer vorhanden
- [ ] Analog/Digital GND-Trennung mit Sternpunkt unter AMS1117
- [ ] USB-C am Rand, zugaenglich
- [ ] RJ45 auf Unterseite (THT)
- [ ] Montageloecher (4x M3, 3.2mm Bohrung) in Ecken
- [ ] BOM: Alle LCSC-Nummern korrekt und auf Lager
- [ ] Pick & Place: Bauteil-Rotation in JLCPCB-Vorschau geprueft
- [ ] Silkscreen lesbar (Referenzbezeichnungen nicht unter Bauteilen)
- [ ] Board-Groesse = 65 x 45 mm
- [ ] Layer = 2 (Top + Bottom)
- [ ] PCB Thickness = 1.6mm
- [ ] Surface Finish = HASL bleifrei (oder ENIG)

---

## Support & Weiterentwicklung

**GitHub Repository:**
https://github.com/DO1XX/radiocore (falls public)

**Kontakt:**
DO1XX (Dirk) — E-Mail: dirk@1xx.is

**Lizenz:**
Hardware: CERN Open Hardware License v2 (CERN-OHL-S)
Firmware: MIT License

**73 de DO1XX!**
