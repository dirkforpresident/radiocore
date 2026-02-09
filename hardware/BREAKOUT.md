# RadioCore Breakout-Adapter

Kleine Platine am Funkgeraet-Ende des RJ45-Kabels.
Verteilt das eine Kabel auf separate Buchsen.

## Konzept

```
RadioCore Board              Cat5 STP Kabel              Breakout PCB
┌──────────────┐         (Standard, fertig kaufen)      ┌──────────────┐
│              │                                        │              │
│   [RJ45] ════╪════════════════════════════════════════╪═ [RJ45]      │
│              │         1-10m, geschirmt                │    │         │
└──────────────┘                                        │    ├── ○ RX  │ 3.5mm
                                                        │    ├── ○ TX  │ 3.5mm
                                                        │    ├── ○ CAT │ 3.5mm
                                                        │    └── [□□]  │ Schraub-
                                                        │     PTT COS  │ klemme
                                                        └──────────────┘
                                                         30 x 20 mm
```

## Oder DIY (ohne Breakout-PCB)

```
                  ┌── orange/weiss ── RX Audio + (Tip)
Cat5 Kabel ──────┤── orange ──────── RX Audio GND (Sleeve)
(Ende abmanteln, ├── gruen/weiss ─── TX Audio + (Tip)
 Paare aufloesen) ├── gruen ────────── TX Audio GND (Sleeve)
                  ├── blau/weiss ──── PTT
                  ├── blau ─────────── COS
                  ├── braun/weiss ─── CAT TX
                  └── braun ────────── CAT RX
                  Schirm ───────────── GND
```

Einfach Klinkenstecker an die Audio-Paare loeten — fertig.

## Breakout PCB — Stueckliste

| Bauteil | Menge | LCSC | Preis |
|---|---|---|---|
| RJ45 Buchse geschirmt THT | 1 | C386756 | $0.30 |
| 3.5mm Klinkenbuchse TRS | 3 | C145819 | $0.15 |
| Schraubklemme 2x2P | 1 | C474881 | $0.08 |
| PCB 30x20mm | 1 | — | $0.50 |
| **Gesamt** | | | **~$1.50** |

Keine aktiven Bauteile, nur Buchsen — kann jeder loeten.

## Adapter-Kabel pro Funkgeraet

### Yaesu FT-891 / FT-991 (Mini-DIN 6)

```
RJ45 Pin 1 (RX Audio) ──► Mini-DIN Pin 2 (1200bps)
RJ45 Pin 3 (TX Audio) ──► Mini-DIN Pin 1 (PKD)
RJ45 Pin 4 (PTT)      ──► Mini-DIN Pin 3 (PTT)
RJ45 Pin 8 (GND)      ──► Mini-DIN Pin 2 (GND)
RJ45 Pin 7 (CAT TX)   ──► 3.5mm TRS Tip (CAT)
RJ45 Pin 8 (CAT RX)   ──► 3.5mm TRS Ring (CAT)
```

### Icom IC-7300 (3.5mm + USB)

```
RJ45 Pin 1 (RX Audio) ──► 3.5mm Klinke (ACC2 / Phone Out)
RJ45 Pin 3 (TX Audio) ──► 3.5mm Klinke (ACC2 / Mic)
CAT: IC-7300 hat USB direkt (nicht ueber RadioCore)
```

### Baofeng / Handfunke (K1 Connector)

```
RJ45 Pin 1 (RX Audio) ──► 3.5mm Tip (Speaker)
RJ45 Pin 3 (TX Audio) ──► 2.5mm Tip (Mic)
RJ45 Pin 4 (PTT)      ──► 2.5mm Ring (PTT)
RJ45 Pin 8 (GND)      ──► gemeinsam GND
```

### Universal (offene Enden)

```
Einfach Cat5-Kabel aufschneiden und direkt anloeten.
Farben siehe DIY-Sektion oben.
```
