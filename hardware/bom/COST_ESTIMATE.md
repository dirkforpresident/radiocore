# RadioCore v1.0 — Kostenabschaetzung JLCPCB

## BOM Kosten (pro Board, bei 5 Stueck)

| Bauteil | LCSC | Stueckpreis | Anzahl | Summe |
|---|---|---|---|---|
| ESP32-S3-WROOM-1-N16R8 | C2913202 | $3.71 | 1 | $3.71 |
| ES8388 Audio Codec | C365736 | $0.52 | 1 | $0.52 |
| AMS1117-3.3V | C6186 | $0.11 | 1 | $0.11 |
| 2N7002 MOSFET | C8545 | $0.01 | 1 | $0.01 |
| USBLC6-2SC6 ESD | C7519 | $0.15 | 1 | $0.15 |
| WS2812B LED | C2761795 | $0.04 | 1 | $0.04 |
| USB-C Connector | C165948 | $0.10 | 1 | $0.10 |
| RJ45 Connector | ~C386756 | $0.30 | 1 | $0.30 |
| Taster (2x) | C318884 | $0.03 | 2 | $0.06 |
| Kondensatoren (diverse) | — | ~$0.01 | 15 | $0.15 |
| Widerstaende (diverse) | — | ~$0.01 | 8 | $0.08 |
| **BOM Gesamt** | | | | **~$5.23** |

## PCB + Assembly Kosten (5 Stueck)

| Position | Preis |
|---|---|
| PCB 60x40mm 2-Layer (5 Stk) | ~$2.00 |
| SMT Assembly Setup | ~$8.00 |
| SMT Assembly (pro Board) | ~$0.50 |
| Extended Parts Gebuehr | ~$3.00 |
| **PCB + Assembly Gesamt** | **~$15.50** |

## Gesamtkosten

| | 5 Stueck | Pro Board |
|---|---|---|
| BOM (5x) | $26.15 | $5.23 |
| PCB + Assembly | $15.50 | $3.10 |
| **Zwischensumme** | **$41.65** | **$8.33** |
| Versand (DHL Express) | ~$20.00 | $4.00 |
| **Total** | **~$62** | **~$12.33** |

## Nicht bei JLCPCB bestueckt (THT / manuell)

| Bauteil | Grund | Preis |
|---|---|---|
| RJ45 Buchse | THT, manuell loeten | ~$0.30 |

RJ45 ist Through-Hole und muss von Hand geloetet werden.
Alternative: 2x 3.5mm Klinkenbuchsen (SMD, bei JLCPCB bestueckbar).

## Zeitplan

| Schritt | Dauer |
|---|---|
| KiCad Schematic + PCB Layout | 1-2 Tage |
| JLCPCB Review + Fertigung | 3-5 Tage |
| Versand (DHL Express) | 5-7 Tage |
| **Gesamt** | **~10-14 Tage** |
