# RadioCore - Produkt-Dokumentation

## Produkt-Uebersicht

**RadioCore: Ein Board, drei Anwendungen**

RadioCore ist ein vielseitiges Interface-Board fuer Amateurfunk und CB-Funk, das drei Hauptanwendungen in einem Geraet vereint:

1. **Repeater-Steuerung** - Automatisches Relais mit Logik, Timer, Roger-Beep
2. **Remote Station** - Fernsteuerung der Funkstation ueber Internet
3. **Digimode Interface** - Digitale Betriebsarten (FT8, RTTY, PSK31, etc.)

### Zielgruppe

- Deutsche Funkamateure (~60.000 lizenziert)
- CB-Funker (~20.000 aktiv in DACH)
- Freenet/PMR-Nutzer
- Internationale Funkamateure (weltweit einsetzbar)
- Relaisfunkstellen und Clubs

### Alleinstellungsmerkmale

- **Vielseitigkeit**: Ein Board fuer alle drei Anwendungen
- **Preis-Leistung**: 99€ statt 500€+ fuer vergleichbare Loesungen
- **Einfache Installation**: Plug-and-Play mit vorgefertigten Kabeln
- **Cloud-Option**: Optional Internet-Fernsteuerung, funktioniert aber auch offline
- **Erweiterbar**: Expansion-Header fuer Relay-Shields, Rotor-Steuerung, Sensoren

## Produktpalette

### Hauptprodukt

| Produkt | Preis | Beschreibung |
|---------|-------|--------------|
| **RadioCore Board Kit** | **99€** | Board + Breakout-PCB + Adapter-Kabel + Cat5-Kabel + QR-Code |

**Lieferumfang:**
- RadioCore Board (ESP32-S3, Audio-Codec, PTT-Relais)
- Breakout-PCB (RJ45 zu Schraubklemmen)
- Adapter-Kabel (waehlbar: Yaesu/Icom/Baofeng/Universal)
- Cat5-Kabel (1m, verbindet Board mit Breakout)
- QR-Code-Aufkleber fuer Cloud-Pairing

**Fertigungskosten:** ~20€ (Board + Breakout + Kabel + Verpackung)
**Marge:** ~79€ (80%)

### Breakout-Kabel (Zubehoer)

| Produkt | Preis | Beschreibung |
|---------|-------|--------------|
| Breakout-Kabel Yaesu | 12€ | Mini-DIN 6 fuer FT-891, FT-991, FT-DX10 |
| Breakout-Kabel Icom | 12€ | 3.5mm + USB fuer IC-7300, IC-9700 |
| Breakout-Kabel Baofeng | 10€ | K1-Stecker fuer UV-5R, UV-82 |
| Breakout-Kabel Universal | 8€ | Offene Enden zum Selbstloeten |

### Expansion-Boards

| Produkt | Preis | Beschreibung |
|---------|-------|--------------|
| **Relay Shield 4x** | 19€ | 4 Relais-Ausgaenge, direkt auf Expansion-Header |
| **Relay Shield 8x** | 29€ | 8 Relais-Ausgaenge via I2C MCP23017 |
| **Rotor Interface** | 29€ | Yaesu/Kenpro Rotor-Steuerung (Azimut + Elevation) |
| **Sensor Board** | 19€ | SWR-Messung, Temperatur, Spannungsueberwachung |

**Anwendungsfaelle Relay Shields:**
- Antennenumschaltung (mehrere Antennen per Relais)
- Transverter-Steuerung
- Sequencer fuer Endstufen
- Zusatzgeraete schalten (z.B. Tuner, Filter)

## Cloud-Service / Abo-Modell

RadioCore funktioniert **ohne Abo** im LAN fuer Repeater und Digimode. Die Cloud-Anbindung ist **optional** fuer Remote-Betrieb ueber Internet.

| Tier | Preis/Monat | Features |
|------|-------------|----------|
| **Free** | 0€ | LAN-Betrieb, Digimode, Firmware-Updates, lokales Web-UI |
| **Remote Basic** | 4.99€ | Cloud-Remote fuer 1 Board, Audio + CAT, WebSocket-Relay |
| **Remote Pro** | 9.99€ | Bis zu 3 Boards, Multi-Client, Aufzeichnung, Logging |
| **Station Share** | 14.99€ | Station teilen mit Anderen, Buchungssystem, Einnahmen |

### Station Sharing - Das Airbnb fuer Funkstationen

**Konzept:**
- Stationseigentuemer teilt seine KW/VHF-Station ueber die Plattform
- Gastoperatoren buchen Zeitfenster per Browser
- Eigentuemer verdient Geld, Plattform nimmt 20% Provision
- Gast braucht **keine Hardware** - nur einen Browser
- Aehnlich wie RemoteHams.com, aber mit eigenem Hardware-Oekosystem

**Beispiel-Rechnung:**
- Station in JO62 mit Yagi + 100W vermietet fuer 5€/Stunde
- 20 Stunden/Monat gebucht = 100€ Bruttoeinnahmen
- 20% Provision = 20€ an Plattform
- 80€ Nettoeinnahmen fuer Stationseigentuemer

**Win-Win:**
- Eigentuemer: Passive Einnahmen aus ungenutzter Station
- Gast: Zugang zu DX-Stationen ohne eigene Antenne
- Plattform: Provisionseinnahmen

## Konkurrenz-Vergleich

| Produkt | Preis | Kann | RadioCore-Vorteil |
|---------|-------|------|-------------------|
| **RemoteRig RRC-1258** | 500€ (Paar) | Nur Remote | 5x guenstiger |
| **SignaLink USB** | 120€ | Nur Digimode | 3x guenstiger + mehr Features |
| **Digirig** | 50€ | Nur Digimode | Gleicher Preis, viel mehr Features |
| **Microham** | 200€+ | Interface | Teuer, alt, kein Remote |
| **RadioCore** | 99€ | **Alles drei** | **Bester Preis** |

### Warum RadioCore gewinnt

1. **Preis:** 99€ vs. 500€+ fuer Remote-Loesungen
2. **Vielseitigkeit:** Repeater + Remote + Digimode in einem
3. **Modern:** ESP32-S3, WebSocket, Web-UI statt alter PC-Software
4. **Erweiterbar:** Shields fuer Relais, Rotor, Sensoren
5. **Cloud-Option:** Keine Portfreigaben, funktioniert hinter NAT
6. **Open:** Firmware-Updates, API fuer eigene Software

## Vertriebskanaele

### Direkt

- **Eigener Online-Shop** (bereits vorhanden auf 1xx.is)
- YouTube-Reviews und Demos
- Social Media (Twitter/X, Mastodon, ham radio communities)

### Events

- **HAM Radio Friedrichshafen** (jaehrlich, Juni) - groesste Messe in Europa
- Amateurfunk-Flohmaerkte (regelmaessig in DE)
- DARC-Ortsverbandstreffen (Demos bei lokalen Clubs)

### Online-Plattformen

- Foren: funkamateur.de, darc.de, qrz.com
- Marktplaetze: eBay, Amazon (fuer internationale Reichweite)
- Ham radio Facebook-Gruppen

### Haendler (spaeter)

- Funktechnik Dathe (Berlin)
- WiMo (Herxheim) - groesster deutscher Haendler
- Difona (Polen) - osteuropaeischer Markt

## Fahrplan

### Phase 1 (jetzt): Prototyping
- Hardware-Design finalisieren
- Firmware-Basics (Repeater + Digimode)
- 5 Prototypen fertigen
- Tests mit FT-891, IC-7300, Baofeng

**Dauer:** 1-2 Monate

### Phase 2 (Monat 2-3): Digimode-Launch
- Digimode-Firmware fertig (keine Cloud noetig)
- Shop-Seite mit Produktfotos
- YouTube-Video: "99€ Digimode Interface - besser als SignaLink?"
- Erste Verkaeufe an Early Adopters

**Ziel:** 50 verkaufte Boards

### Phase 3 (Monat 3-6): Cloud-Remote
- Cloud-Backend (FastAPI, Hetzner)
- Remote-Firmware fertig
- Abo-System einrichten (Stripe)
- YouTube-Video: "Remote-Station fuer 99€ statt 500€"

**Ziel:** 200 verkaufte Boards, 50 Remote-Abos

### Phase 4 (Monat 6-12): Station Sharing
- Station-Sharing-Plattform
- Expansion-Boards (Relay-Shields, Rotor)
- HAM Radio Friedrichshafen (Messestand)
- Internationale Expansion (USA, UK)

**Ziel:** 1000 verkaufte Boards, 500 Abos

## Skalierung & Umsatzpotenzial

### Hardware-Umsatz

- 1000 Boards × 99€ = 99.000€ Umsatz
- 1000 Boards × 79€ Marge = 79.000€ Gewinn

### Abo-Einnahmen (monatlich wiederkehrend)

| Szenario | Abos | Preis | MRR (Monthly Recurring Revenue) |
|----------|------|-------|---------------------------------|
| Konservativ | 500 | 4.99€ | 2.495€/Monat |
| Realistisch | 1000 | 4.99€ | 4.990€/Monat |
| Optimistisch | 2000 | 4.99€ | 9.980€/Monat |

**Wichtig:** Abo-Einnahmen sind **passiv** und **wiederkehrend**. Jedes verkaufte Board ist ein potenzieller Abonnent fuer immer.

### Station Sharing Einnahmen

- 100 Stationen teilen ihre Station
- Durchschnittlich 10 Stunden/Monat gebucht à 5€/Stunde
- 100 × 10 × 5€ = 5.000€ Bruttoeinnahmen
- 20% Provision = **1.000€/Monat**

### Server-Kosten

- Hetzner CX21 (4€/Monat) fuer 100 gleichzeitige Audio-Streams
- Bei 500 Abos: ~2-3 Server = 12€/Monat
- Bei 2000 Abos: ~10 Server = 40€/Monat

**Skalierung ist guenstig:** Audio-Streaming braucht nur ~24kbps pro Board.

## Zusammenfassung

RadioCore ist ein **disruptives Produkt** im Amateurfunk-Markt:

- **Preis:** 5x guenstiger als Konkurrenz
- **Vielseitigkeit:** Repeater + Remote + Digimode in einem
- **Umsatzmodell:** Hardware-Marge + wiederkehrende Abo-Einnahmen
- **Skalierbar:** Internationale Vermarktung moeglich
- **Wachstumspotenzial:** Station Sharing eroeffnet neuen Markt

**Break-Even:** ~100 verkaufte Boards decken Entwicklungskosten.
**Ziel Jahr 1:** 1000 Boards, 500 Abos, 5.000€/Monat passive Einnahmen.
