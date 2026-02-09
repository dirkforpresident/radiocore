# RadioCore v1.0 — Kostenabschaetzung JLCPCB

## BOM Kosten (pro Board, bei 5 Stueck)

| Bauteil | LCSC | Stueckpreis | Anzahl | Summe |
|---|---|---|---|---|
| ESP32-S3-WROOM-1-N16R8 | C2913202 | $3.71 | 1 | $3.71 |
| ES8388 Audio Codec | C365736 | $0.52 | 1 | $0.52 |
| AMS1117-3.3V LDO | C6186 | $0.11 | 1 | $0.11 |
| XL1509-5.0E1 Buck DC-DC | C61063 | $0.11 | 1 | $0.11 |
| 2N7002 MOSFET (PTT) | C8545 | $0.01 | 1 | $0.01 |
| SS34 Schottky Diode | C8678 | $0.04 | 2 | $0.08 |
| USBLC6-2SC6 ESD | C7519 | $0.15 | 1 | $0.15 |
| WS2812B LED | C2761795 | $0.04 | 1 | $0.04 |
| USB-C Connector | C165948 | $0.10 | 1 | $0.10 |
| Schraubklemme 2P (DC) | C474881 | $0.08 | 1 | $0.08 |
| 33uH Inductor (Buck) | C339747 | $0.06 | 1 | $0.06 |
| 100uF/50V Elko (DC In) | C176672 | $0.08 | 1 | $0.08 |
| Taster (2x) | C318884 | $0.03 | 2 | $0.06 |
| 3.5mm TRS Klinke (RX/TX) | C17701688 | $0.02 | 2 | $0.04 |
| Ferrite Bead 600R (EMC) | C76890 | $0.01 | 8 | $0.08 |
| 1nF Kondensator (Audio LP) | C14442 | $0.001 | 2 | $0.01 |
| 100pF Kondensator (Digital LP) | C1546 | $0.001 | 3 | $0.01 |
| Kondensatoren (diverse) | — | ~$0.01 | 16 | $0.16 |
| Widerstaende (diverse) | — | ~$0.01 | 8 | $0.08 |
| **BOM Gesamt (SMD)** | | | | **~$5.49** |

## Nicht bei JLCPCB bestueckt (THT / manuell)

| Bauteil | LCSC | Grund | Preis |
|---|---|---|---|
| RJ45 Buchse geschirmt THT | C2683360 | Through-Hole, manuell loeten | ~$0.13 |

RJ45 ist Through-Hole und muss von Hand geloetet werden (1 Bauteil, einfach).
Die 3.5mm Klinken sind SMD und werden bei JLCPCB bestueckt.

## PCB + Assembly Kosten (5 Stueck)

| Position | Preis |
|---|---|
| PCB 65x45mm 2-Layer (5 Stk) | ~$2.00 |
| SMT Assembly Setup | ~$8.00 |
| SMT Assembly (pro Board) | ~$0.50 |
| Extended Parts Gebuehr | ~$3.00 |
| **PCB + Assembly Gesamt** | **~$15.50** |

## Gesamtkosten

| | 5 Stueck | Pro Board |
|---|---|---|
| BOM SMD (5x) | $27.45 | $5.49 |
| BOM THT (5x RJ45) | $0.65 | $0.13 |
| PCB + Assembly | $15.50 | $3.10 |
| **Zwischensumme** | **$43.60** | **$8.72** |
| Versand (DHL Express) | ~$20.00 | $4.00 |
| **Total** | **~$64** | **~$12.72** |

## Stromversorgung

| Quelle | Spannung | Weg |
|---|---|---|
| USB-C | 5V | D3 Schottky → 5V Rail → AMS1117 → 3.3V |
| DC Eingang | 5-24V | XL1509 Buck → 5V → D4 Schottky → 5V Rail → AMS1117 → 3.3V |
| Beides | Auto | Schottky-Dioden: hoechste Quelle gewinnt, keine Rueckspeisung |

Typische Szenarien:
- **Am Computer**: USB-C liefert 5V — fertig
- **Am Funkgeraet-Netzteil**: 13.8V an Schraubklemme → Buck auf 5V
- **Beides gleichzeitig**: Kein Problem, Dioden regeln das

## Zeitplan

| Schritt | Dauer |
|---|---|
| EasyEDA Schematic + PCB Layout | 1-2 Tage |
| JLCPCB Review + Fertigung | 3-5 Tage |
| Versand (DHL Express) | 5-7 Tage |
| **Gesamt** | **~10-14 Tage** |
