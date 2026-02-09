# RadioCore v1.0 — PCB-Erstellung in EasyEDA (Standard)

Schritt-fuer-Schritt-Anleitung: Vom leeren Bildschirm zur JLCPCB-Bestellung.

---

## 1. Account & Projekt

1. Gehe zu **https://easyeda.com** und klicke "Sign Up"
2. Account mit E-Mail erstellen (kostenlos, Standard-Version reicht)
3. Nach dem Login: **File → New → Project**
4. Projektname: `RadioCore v1.0`
5. Im Projekt: **New → Schematic** anlegen (wird automatisch geoeffnet)

---

## 2. Bauteile aus LCSC importieren

EasyEDA ist direkt mit LCSC verbunden. Jedes Bauteil hat eine LCSC-Nummer (z.B. C2913202).

**So geht's:**

1. Rechts im Panel: **Library** (Bibliothek) oeffnen
2. In das Suchfeld die **LCSC-Nummer** eingeben (z.B. `C2913202`)
3. Das Bauteil erscheint — auf **Place** klicken
4. Mit der Maus auf dem Schematic platzieren, Klick zum Absetzen
5. **ESC** druecken um den Platziermodus zu verlassen

> **Tipp:** Vor dem Platzieren auf das Bauteil klicken und "Datasheet" pruefen — so stellst du sicher, dass es das richtige Teil ist.

---

## 3. Schematic — Bauteile platzieren

Platziere alle Bauteile in dieser Reihenfolge. Lass genuegend Abstand zwischen den Gruppen.

### 3.1 ICs (Hauptbauteile)

| Ref | Bauteil | LCSC# | Beschreibung |
|-----|---------|-------|--------------|
| U1 | ESP32-S3-WROOM-1-N16R8 | C2913202 | Mikrocontroller-Modul (16MB Flash, 8MB PSRAM) |
| U2 | ES8388 | C365736 | Audio-Codec (I2S, ADC+DAC) |
| U3 | AMS1117-3.3 | C6186 | 3.3V LDO-Regler |
| U4 | XL1509-5.0E1 | C61063 | 5V Step-Down Buck-Converter |

**Platzierung auf dem Schematic:**
- U1 (ESP32) in die Mitte-Links setzen — hat die meisten Verbindungen
- U2 (ES8388) rechts daneben
- U3 (3.3V LDO) oben links (Stromversorgung)
- U4 (Buck) oben rechts (Stromversorgung)

### 3.2 Halbleiter

| Ref | Bauteil | LCSC# | Beschreibung |
|-----|---------|-------|--------------|
| Q1 | 2N7002 | C8545 | N-Channel MOSFET (PTT-Schaltung) |
| D1 | WS2812B | C2761795 | Adressierbare RGB-LED |
| D2 | USBLC6-2SC6 | C7519 | USB ESD-Schutz |
| D3 | SS34 | C8678 | Schottky-Diode (Buck-Converter) |
| D4 | SS34 | C8678 | Schottky-Diode (Verpolschutz) |

### 3.3 Steckverbinder & Taster

| Ref | Bauteil | LCSC# | Beschreibung |
|-----|---------|-------|--------------|
| J1 | USB-C Buchse | C165948 | USB Type-C 2.0 (Stromversorgung + Daten) |
| J3 | Schraubklemme 2P | C474881 | 2-Pin Schraubklemme (ext. 12V Eingang) |
| SW1 | Taster | C318884 | BOOT-Taste (GPIO0) |
| SW2 | Taster | C318884 | RESET-Taste (EN) |

> **Hinweis:** RJ45-Buchse (J2) ist ein Durchsteck-Bauteil (THT) und wird spaeter manuell bestueckt. Trotzdem jetzt schon im Schematic platzieren, z.B. LCSC# suchen oder als generisches 8-Pin-Connector-Symbol verwenden.

### 3.4 Passive Bauteile

Suche diese ueber die Library. Fuer generische Widerstaende/Kondensatoren kannst du auch direkt nach dem Wert suchen (z.B. "10k 0402").

**Widerstaende (alle 0402, 1%):**

| Ref | Wert | Menge | Verwendung |
|-----|------|-------|------------|
| R1, R2 | 5.1k | 2 | USB-C CC1/CC2 Pull-Down |
| R3, R4 | 22R | 2 | USB D+/D- Serienwiderstands |
| R5 | 10k | 1 | ESP32 EN Pull-Up |
| R6 | 10k | 1 | ESP32 GPIO0 Pull-Up |
| R7 | 10k | 1 | 2N7002 Gate Pull-Down |
| R8 | 4.7k | 1 | WS2812B Datenleitung |
| R9, R10 | 22k | 2 | ES8388 I2C Pull-Up (auf 3.3V) |
| R11 | 100k | 1 | Buck-Converter Feedback |
| R12 | 22k | 1 | Buck-Converter Feedback |

**Kondensatoren:**

| Ref | Wert | Package | Menge | Verwendung |
|-----|------|---------|-------|------------|
| C1, C2 | 100nF | 0402 | 2 | ESP32 Abblockkondensatoren |
| C3 | 10uF | 0805 | 1 | ESP32 Bulk-Kondensator |
| C4, C5 | 100nF | 0402 | 2 | ES8388 DVDD/AVDD Abblockung |
| C6 | 10uF | 0805 | 1 | ES8388 Bulk |
| C7 | 22uF | 0805 | 1 | AMS1117 Ausgang |
| C8 | 22uF | 0805 | 1 | AMS1117 Eingang |
| C9 | 100uF | 1206 | 1 | Buck Ausgang (Low-ESR) |
| C10 | 100nF | 0402 | 1 | Buck VCC Abblockung |
| C11 | 100nF | 0402 | 1 | USB ESD-Schutz Abblockung |
| C12 | 100nF | 0402 | 1 | WS2812B Abblockung |

**Spule (Induktor):**

| Ref | Wert | Verwendung |
|-----|------|------------|
| L1 | 68uH | Buck-Converter XL1509 Speicherdrossel |

**Ferrite Beads (0402):**

| Ref | Wert | LCSC# | Verwendung |
|-----|------|-------|------------|
| FB1 | 600R@100MHz | C1015 | Analog-VCC Filterung (ES8388 AVDD) |
| FB2 | 600R@100MHz | C1015 | Audio-Ausgang Filterung |

> **Tipp zum schnellen Platzieren:** Wenn du mehrere gleiche Bauteile brauchst (z.B. 100nF Kondensatoren), platziere das erste, dann waehle es aus und druecke **Strg+C / Strg+V** zum Duplizieren. EasyEDA vergibt automatisch neue Referenzbezeichnungen (C1 → C2 → C3...).

---

## 4. Verbindungen ziehen (Wiring)

Verwende das **Wire-Tool** (Taste **W**) um Verbindungen zu ziehen. Klicke auf einen Pin, ziehe die Leitung zum Ziel-Pin, Klick zum Absetzen.

### 4.1 Stromversorgung

```
J1 (USB-C VBUS) ──→ D2 (ESD) ──→ U3 (AMS1117 VIN)
                                  ──→ U4 (XL1509 VIN) falls kein ext. 12V

J3 (Schraubklemme +) ──→ D4 (SS34, Verpolschutz) ──→ U4 (XL1509 VIN)
J3 (Schraubklemme -) ──→ GND

U4 (XL1509 OUT) ──→ L1 ──→ 5V Rail ──→ D1 (WS2812B VDD)
                                      ──→ U3 (AMS1117 VIN)

U3 (AMS1117 VOUT) ──→ 3.3V Rail ──→ U1 (ESP32 3V3)
                                  ──→ U2 (ES8388 DVDD)
                                  ──→ R5/R6 Pull-Ups

3.3V ──→ FB1 ──→ U2 (ES8388 AVDD)   [Analog-VCC gefiltert]
```

> **Tipp: Power Flags & Netzlabels verwenden!** Statt jede GND-Verbindung als Draht zu ziehen: Setze **Net Labels** (Taste **N**). Erstelle Labels fuer `GND`, `3V3`, `5V`, `VBUS`. Alle Pins mit dem gleichen Label sind automatisch verbunden.

### 4.2 USB-C Datenleitungen

```
J1 D+ ──→ R3 (22R) ──→ U1 (ESP32 GPIO20/D+)
J1 D- ──→ R4 (22R) ──→ U1 (ESP32 GPIO19/D-)
J1 CC1 ──→ R1 (5.1k) ──→ GND
J1 CC2 ──→ R2 (5.1k) ──→ GND

D2 (USBLC6): IO1/IO2 an D+/D-, VCC an VBUS, GND an GND
```

### 4.3 I2S Bus (ESP32 → ES8388)

```
U1 GPIO4  (BCLK)  ──→ U2 SCLK
U1 GPIO5  (WS)    ──→ U2 LRCK
U1 GPIO6  (DOUT)  ──→ U2 DSDIN    [ESP32 sendet → ES8388 DAC]
U1 GPIO7  (DIN)   ──→ U2 ASDOUT   [ES8388 ADC → ESP32 empfaengt]
U1 GPIO15 (MCLK)  ──→ U2 MCLK
```

### 4.4 I2C Bus (ESP32 → ES8388 Steuerung)

```
U1 GPIO8  (SDA) ──→ U2 SDA ──→ R9 (22k) ──→ 3.3V
U1 GPIO9  (SCL) ──→ U2 SCL ──→ R10 (22k) ──→ 3.3V
```

### 4.5 PTT & Audio-Ausgang

```
U1 GPIO10 (PTT) ──→ Q1 Gate
Q1 Source ──→ GND
Q1 Drain ──→ J2 (RJ45, PTT-Pin)
R7 (10k) zwischen Q1 Gate und GND (Pull-Down)

U2 (ES8388 LOUT1/ROUT1) ──→ FB2 ──→ J2 (RJ45, Audio-Out Pins)
U2 (ES8388 LINPUT1/RINPUT1) ──→ J2 (RJ45, Audio-In Pins)
```

### 4.6 Sonstige Verbindungen

```
U1 GPIO0  ──→ SW1 ──→ GND          [BOOT-Taste, mit R6 Pull-Up]
U1 EN     ──→ SW2 ──→ GND          [RESET-Taste, mit R5 Pull-Up]
U1 GPIO48 ──→ R8 (4.7k) ──→ D1 (WS2812B DIN)

D3 (SS34): Kathode an XL1509 SW-Pin, Anode an GND (Freilaufdiode)
```

### 4.7 Abblockung nicht vergessen

Jeden Abblock-Kondensator (100nF) so nah wie moeglich an den zugehoerigen VCC-Pin platzieren:

- C1, C2: an ESP32 3V3-Pins
- C4, C5: an ES8388 DVDD und AVDD
- C10: an XL1509 VCC
- C11: an USBLC6 VCC
- C12: an WS2812B VDD

### 4.8 ERC (Electrical Rules Check)

Wenn alles verbunden ist:

1. Menue: **Design → Electrical Rules Check (ERC)**
2. Alle Fehler durchgehen und beheben
3. Warnungen bei "Unconnected Pins" pruefen — manche sind gewollt (z.B. NC-Pins)
4. Erst wenn ERC sauber ist → weiter zum PCB

---

## 5. PCB Layout

### 5.1 PCB erstellen

1. Im Schematic: **Design → Convert Schematic to PCB**
2. EasyEDA erstellt automatisch ein neues PCB-Dokument
3. Alle Bauteile liegen erstmal als Haufen neben dem Board

### 5.2 Board-Umriss definieren

1. Wechsle zum **Board Outline** Layer
2. Zeichne ein Rechteck: **65mm x 45mm**
3. Optional: Ecken abrunden (Radius 2mm) fuer professionelleres Aussehen
4. 4x Montageloecher (M3, 3.2mm) in die Ecken setzen, je 3mm vom Rand

### 5.3 Design Rules einstellen

**Design → Design Rules:**

| Parameter | Wert |
|-----------|------|
| Layers | 2 (Top + Bottom) |
| Kupferdicke | 1oz (35um) |
| Min. Leiterbahnbreite | 0.2mm (Signale), 0.3mm (Strom) |
| Min. Abstand | 0.2mm |
| Via-Durchmesser | 0.6mm, Bohrung 0.3mm |

### 5.4 Bauteil-Platzierung

Ziehe die Bauteile einzeln auf das Board. **Reihenfolge und Position:**

```
┌─────────────────────────────────────────────────────┐
│  [J1 USB-C]          [U4 Buck]  [L1] [D3]          │
│                      [C9] [C10]                     │
│  [D2 ESD]  [R1-R4]                                  │
│                                                     │
│  [SW1] [SW2]                                        │
│                                                     │
│  ┌──────────────┐    ┌──────────┐   [FB1] [FB2]    │
│  │              │    │          │                    │
│  │  U1 ESP32    │    │ U2 ES8388│   Audio-Bereich   │
│  │              │    │          │                    │
│  │  [Antenne →] │    └──────────┘                   │
│  └──────────────┘    [C4-C6]                        │
│                                                     │
│  [C1-C3]       [D1 LED]                             │
│                                                     │
│  [U3 LDO]                          [Q1] [R7]       │
│  [C7] [C8]          [J2 RJ45]      [D4]            │
│                      (THT, unten)                   │
│  [J3 Klemme]                                        │
└─────────────────────────────────────────────────────┘
```

**Wichtige Regeln:**

- **ESP32 Antenne:** Die Antenne ragt ueber den PCB-Rand hinaus oder es muss ein Freiraum von mind. 10mm um die Antenne sein. **Kein Kupfer (weder Top noch Bottom) unter oder neben der Antenne!**
- **ES8388:** So nah wie moeglich am ESP32, kurze I2S-Leitungen
- **Buck-Converter (U4, L1, D3, C9):** Zusammen in eine Ecke, kurze Schleifen
- **USB-C:** Am Rand platzieren, zugaenglich
- **Abblock-Kondensatoren:** Direkt neben den zugehoerigen IC-Pins (nicht irgendwo anders!)

### 5.5 Leiterbahnen routen

1. Waehle das **Route Track** Tool (Taste **W** im PCB-Editor)
2. Klicke auf einen Pin → ziehe zum Ziel-Pin → Klick zum Absetzen

**Routing-Reihenfolge:**

1. **Stromversorgung zuerst:** VBUS, 5V, 3.3V, GND — breite Leiterbahnen (0.3-0.5mm)
2. **USB-Datenleitungen:** D+ und D- als Paar routen, gleich lang, 0.2mm
3. **I2S-Bus:** BCLK, LRCK, DSDIN, ASDOUT, MCLK — kurz halten, 0.2mm
4. **I2C:** SDA, SCL — unkritisch, 0.2mm
5. **Rest:** PTT, LED-Daten, Taster

> **Tipp:** Nutze **Vias** um zwischen Top und Bottom zu wechseln wenn sich Leitungen kreuzen wuerden.

### 5.6 Ground Plane

1. Waehle den **Bottom Layer**
2. **Place → Copper Area** (Kupferflaeche)
3. Zeichne ein Rechteck ueber das gesamte Board
4. Netz: **GND** auswaehlen
5. Klicke **Rebuild Copper Area**

Die Ground Plane verbindet automatisch alle GND-Pads ueber Vias.

**Analog/Digital GND-Trennung:**

- Ziehe eine gedachte Linie zwischen dem Digital-Bereich (ESP32, USB, LED) und dem Analog-Bereich (ES8388, Audio)
- Die GND-Plane verbindet sich nur an **einem Punkt** (in der Naehe des ES8388)
- Dazu: Einen schmalen Schlitz in der GND-Plane lassen, der nur unter dem ES8388 verbunden ist
- In EasyEDA: **Copper Area** fuer den Analog-Bereich separat zeichnen, mit einer schmalen Bruecke zum Digital-GND

### 5.7 DRC (Design Rules Check)

1. **Design → Design Rules Check (DRC)**
2. Alle Fehler beheben (zu enge Abstands, nicht verbundene Netze)
3. Erst wenn DRC fehlerfrei → weiter zur Bestellung

---

## 6. Bestellung bei JLCPCB

### 6.1 Direkt aus EasyEDA exportieren

1. Im PCB-Editor: **Fabrication → JLCPCB**
2. EasyEDA generiert automatisch:
   - **Gerber-Dateien** (PCB-Fertigung)
   - **BOM** (Stueckliste mit LCSC-Nummern)
   - **Pick & Place** (Bestueckungsdaten)
3. Du wirst zu **jlcpcb.com** weitergeleitet

### 6.2 PCB-Optionen bei JLCPCB

| Option | Einstellung |
|--------|-------------|
| Layers | 2 |
| PCB Qty | 5 (Minimum) |
| Dimensions | 65 x 45 mm |
| PCB Thickness | 1.6mm |
| PCB Color | Gruen (guenstigste Option) |
| Surface Finish | HASL (lead-free) |
| Copper Weight | 1oz |
| Castellated Holes | Nein |

### 6.3 SMT Assembly bestellen

1. **"SMT Assembly"** Checkbox aktivieren
2. Seite: **Top Side** (oder Both Sides falls noetig)
3. Bestueckungsmenge: gleich wie PCB Qty
4. **BOM hochladen:** Die automatisch generierte BOM-Datei
5. **Pick & Place hochladen:** Die automatisch generierte CPL-Datei
6. JLCPCB zeigt dir eine Vorschau der Bauteil-Platzierung — **kontrollieren!**
7. Bei falscher Rotation: In der Tabelle den Winkel anpassen (0/90/180/270)

> **Wichtig:** Nicht alle Bauteile sind bei JLCPCB auf Lager. Preis-Kategorien:
> - **Basic Parts:** Guenstig, immer auf Lager (die meisten passiven Bauteile)
> - **Extended Parts:** Aufpreis ~3$ pro einzigartigem Bauteil (ESP32, ES8388, etc.)
> - **Global Sourcing:** Falls ein Teil nicht auf Lager ist, kann JLCPCB es beschaffen (dauert laenger)

### 6.4 Manuell nachloeten

Folgende Bauteile werden **nicht** von JLCPCB bestueckt und muessen von Hand nachgeloetet werden:

| Bauteil | Grund |
|---------|-------|
| J2 (RJ45) | THT-Bauteil (Durchsteckmontage) |
| J3 (Schraubklemme) | THT-Bauteil |

**Benoetigtes Werkzeug:**
- Loetkolben (mind. 40W)
- Loetzinn 0.8mm (bleifrei)
- Flussmittel
- Entloetlitze (fuer Korrekturen)

### 6.5 Bestellung abschliessen

1. Alles nochmal pruefen (BOM, Platzierung, Rotation)
2. Versandart waehlen (Standard: 7-15 Tage, Express: 3-5 Tage)
3. Bezahlen (PayPal, Kreditkarte)
4. Tracking-Nummer kommt per E-Mail

**Gesamtkosten (ca.):**
- PCB-Fertigung (5 Stueck): ~5-8$
- SMT Assembly: ~15-25$ (abhaengig von Extended Parts)
- Bauteile: ~10-20$
- Versand: ~5-15$
- **Gesamt: ca. 35-70$ fuer 5 bestueckte Boards**

---

## Checkliste vor der Bestellung

- [ ] ERC im Schematic fehlerfrei
- [ ] DRC im PCB fehlerfrei
- [ ] Alle Netzlabels korrekt (keine schwebenden Leitungen)
- [ ] Abblock-Kondensatoren direkt an IC-Pins
- [ ] ESP32-Antenne frei von Kupfer
- [ ] Ground Plane auf Bottom Layer vorhanden
- [ ] USB-C am Rand, zugaenglich
- [ ] Montageloecher vorhanden (falls Gehaeuse geplant)
- [ ] BOM: Alle LCSC-Nummern korrekt
- [ ] Pick & Place: Bauteil-Rotation in JLCPCB-Vorschau pruefen
- [ ] Silkscreen lesbar (Referenzbezeichnungen nicht unter Bauteilen)
