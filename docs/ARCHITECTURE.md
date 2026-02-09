# RadioCore - Cloud-Architektur

## System-Uebersicht

RadioCore besteht aus drei Komponenten:

```
RadioCore Board ──wss://──► Cloud Server ◄──https──► Browser/App
   (am Funkgeraet)  outbound   (Hetzner EU)         (ueberall)
```

- **Board:** ESP32-S3 mit Audio-Codec, verbunden mit Funkgeraet
- **Cloud:** FastAPI + PostgreSQL + Redis auf Hetzner
- **Client:** Browser (Web-App) oder Mobile App

### Offline-First Design

**Wichtig:** RadioCore funktioniert **ohne Cloud** fuer Repeater und Digimode!

- Board bootet → prueft WiFi-Config
- Keine Config? → WiFi-AP "RadioCore-XXXX" fuer Setup
- Config vorhanden? → verbindet WiFi → versucht Cloud-Verbindung
- Cloud nicht erreichbar? → **Lokaler Modus** (Repeater/Digimode funktionieren weiter)
- Cloud erreichbar? → WebSocket-Verbindung fuer Remote-Zugriff

## Board-Seite (ESP32-S3 Firmware)

### Boot-Ablauf

1. **Power-On** → ESP32-S3 bootet von Flash
2. **Config lesen** → NVS (Non-Volatile Storage) pruefen:
   - WiFi SSID + Passwort
   - Server URL (z.B. `wss://api.radiocore.xx/ws`)
   - Board Token (UUID)
3. **WiFi-Setup**:
   - Config vorhanden? → Verbinde mit WiFi
   - Keine Config? → Starte AP "RadioCore-XXXX" fuer Ersteinrichtung
4. **Cloud-Verbindung**:
   - DNS lookup fuer api.radiocore.xx
   - TLS-Handshake (wss://)
   - WebSocket-Upgrade
   - Authentifizierung mit Board Token
5. **Reconnect-Logik**:
   - Verbindung verloren? → Exponential Backoff (1s, 2s, 4s, 8s, max 60s)
   - Auto-Reconnect bei WiFi-Stoerung

### WebSocket-Protokoll

**Bidirektional:** JSON fuer Control, Binary fuer Audio

#### JSON Control-Messages (Board → Cloud)

```json
{
  "type": "status",
  "smeter": -73,
  "squelch": true,
  "ptt": false,
  "freq": 14205000,
  "mode": "USB",
  "power": 50
}
```

```json
{
  "type": "heartbeat",
  "timestamp": 1678901234
}
```

#### JSON Control-Messages (Cloud → Board)

```json
{
  "type": "ptt",
  "state": true
}
```

```json
{
  "type": "freq",
  "value": 14205000
}
```

```json
{
  "type": "mode",
  "value": "USB"
}
```

#### Binary Audio-Frames

- **Format:** Opus encoded, 48kHz mono
- **Bitrate:** 24kbps (ausreichend fuer Sprachqualitaet)
- **Frame-Size:** 20ms (960 Samples bei 48kHz)
- **Payload:** `[0x01] [4 Byte Timestamp] [Opus-Daten]`

**Richtung Board → Cloud:**
- RX-Audio vom Funkgeraet
- Kontinuierlich wenn Squelch offen

**Richtung Cloud → Board:**
- TX-Audio fuer Funkgeraet
- Nur wenn PTT aktiv

### OTA Firmware-Updates

- Cloud sendet: `{"type": "ota_available", "version": "1.2.3", "url": "https://..."}`
- Board laedt Firmware herunter (HTTPS)
- Prueft Signatur (optional)
- Flasht neue Firmware
- Reboot → neue Version aktiv

## Cloud Server (FastAPI auf Hetzner)

### Tech-Stack

- **Framework:** FastAPI (Python 3.11+)
- **Datenbank:** PostgreSQL 16
- **Cache/Session:** Redis 7
- **Reverse Proxy:** nginx mit SSL (Let's Encrypt wildcard *.radiocore.xx)
- **OS:** Ubuntu 24.04 LTS

### Architektur

```
Internet
   |
   v
nginx (SSL Termination)
   |
   +----> FastAPI (ASGI: uvicorn)
   |         |
   |         +----> PostgreSQL (User, Boards, Subscriptions)
   |         +----> Redis (Sessions, Audio-Routing)
   |
   +----> WebSocket-Handler
            |
            +----> Board WebSockets (/ws/board/{token})
            +----> Client WebSockets (/ws/client/{board_id})
```

### API-Endpoints

#### Authentifizierung

- `POST /api/auth/register` - User-Registrierung
  - Input: `{"email": "...", "password": "...", "callsign": "DO1XX"}`
  - Output: `{"user_id": "...", "token": "JWT..."}`

- `POST /api/auth/login` - Login
  - Input: `{"email": "...", "password": "..."}`
  - Output: `{"token": "JWT...", "expires": 1678901234}`

- `POST /api/auth/refresh` - Token erneuern
  - Header: `Authorization: Bearer <JWT>`
  - Output: `{"token": "JWT_NEW..."}`

#### Board-Management

- `POST /api/boards/pair` - Board mit Account verknuepfen
  - Input: `{"board_id": "UUID", "name": "Meine Station"}`
  - Output: `{"board": {...}}`

- `GET /api/boards` - Boards des Users auflisten
  - Output: `[{"id": "...", "name": "...", "online": true, ...}, ...]`

- `PUT /api/boards/{id}` - Board-Einstellungen aendern
  - Input: `{"name": "...", "location": "JO62...", "public": false}`

- `DELETE /api/boards/{id}` - Board entfernen

#### WebSocket-Verbindungen

- `WS /api/ws/board/{token}` - **Board verbindet sich hier**
  - Auth: Token in URL (UUID aus ESP32 Flash)
  - Protokoll: JSON + Binary (siehe oben)

- `WS /api/ws/client/{board_id}` - **Browser verbindet sich hier**
  - Auth: JWT in Query-Parameter `?token=...`
  - Protokoll: JSON + Binary (Audio-Relay)

#### Station Sharing

- `GET /api/stations` - Oeffentliche Stationen auflisten
  - Filter: `?grid=JO62&bands=20m,40m`
  - Output: `[{"id": "...", "callsign": "DO1XX", "location": "JO62", ...}, ...]`

- `POST /api/stations/{id}/book` - Zeitfenster buchen
  - Input: `{"start": "2026-02-15T14:00:00Z", "duration_minutes": 60}`
  - Output: `{"booking_id": "...", "price": 5.00, "payment_url": "..."}`

- `GET /api/bookings` - Eigene Buchungen anzeigen
  - Output: `[{"id": "...", "station": {...}, "start": "...", "end": "..."}, ...]`

### Datenbank-Schema (PostgreSQL)

#### Tabelle: users

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    callsign VARCHAR(20),
    created_at TIMESTAMP DEFAULT NOW()
);
```

#### Tabelle: boards

```sql
CREATE TABLE boards (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(100),
    location VARCHAR(10), -- Maidenhead grid
    public BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW(),
    last_seen TIMESTAMP
);
```

#### Tabelle: subscriptions

```sql
CREATE TABLE subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    tier VARCHAR(20), -- 'basic', 'pro', 'station_share'
    status VARCHAR(20), -- 'active', 'cancelled', 'expired'
    stripe_subscription_id VARCHAR(100),
    expires_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);
```

#### Tabelle: bookings

```sql
CREATE TABLE bookings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    station_id UUID REFERENCES boards(id) ON DELETE CASCADE,
    guest_user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    price_eur DECIMAL(10, 2),
    status VARCHAR(20), -- 'pending', 'confirmed', 'completed', 'cancelled'
    created_at TIMESTAMP DEFAULT NOW()
);
```

### Redis-Nutzung

#### Session-State

```
SET session:{user_id} "{...json...}" EX 86400
```

#### Audio-Routing

```
SET board:{board_id}:clients "[client_ws_1, client_ws_2, ...]"
SET client:{client_id}:board "{board_id}"
```

**Audio-Flow:**
1. Board sendet Audio → Server schaut in Redis: welche Clients sind verbunden?
2. Server leitet Audio an alle Clients weiter (Fan-Out)
3. Client sendet Audio → Server schaut in Redis: welches Board?
4. Server leitet Audio an Board weiter

### WebSocket-Handler Implementierung

```python
# Pseudo-Code
active_boards = {}  # board_id -> WebSocket
active_clients = {}  # client_id -> WebSocket

@app.websocket("/ws/board/{token}")
async def board_websocket(websocket: WebSocket, token: str):
    board = authenticate_board(token)
    active_boards[board.id] = websocket

    try:
        while True:
            data = await websocket.receive()

            if data['type'] == 'bytes':
                # Audio von Board → an alle Clients
                clients = redis.smembers(f"board:{board.id}:clients")
                for client_id in clients:
                    await active_clients[client_id].send_bytes(data['bytes'])

            elif data['type'] == 'text':
                # JSON Control → an alle Clients
                msg = json.loads(data['text'])
                clients = redis.smembers(f"board:{board.id}:clients")
                for client_id in clients:
                    await active_clients[client_id].send_text(data['text'])

    finally:
        del active_boards[board.id]

@app.websocket("/ws/client/{board_id}")
async def client_websocket(websocket: WebSocket, board_id: str):
    user = authenticate_jwt(request.query_params['token'])
    client_id = str(uuid.uuid4())
    active_clients[client_id] = websocket
    redis.sadd(f"board:{board_id}:clients", client_id)

    try:
        while True:
            data = await websocket.receive()

            if data['type'] == 'bytes':
                # Audio von Client → an Board
                await active_boards[board_id].send_bytes(data['bytes'])

            elif data['type'] == 'text':
                # JSON Control → an Board
                msg = json.loads(data['text'])
                await active_boards[board_id].send_text(data['text'])

    finally:
        del active_clients[client_id]
        redis.srem(f"board:{board_id}:clients", client_id)
```

## Board Provisioning (Pairing)

### Manufacturing

1. **Board-Fertigung:** UUID wird in ESP32 Flash gebrannt (einmalig)
2. **QR-Code-Druck:** `https://radiocore.xx/pair?id={UUID}` auf Aufkleber
3. **Verpackung:** QR-Code-Aufkleber auf Board kleben

### Customer Journey

1. **Account erstellen:** User registriert sich auf radiocore.xx
2. **QR-Code scannen:** User scannt QR-Code mit Smartphone
3. **Board verknuepfen:** Browser oeffnet `/pair?id=UUID`, User gibt Board einen Namen
4. **API-Call:** `POST /api/boards/pair {"board_id": "UUID", "name": "Meine Station"}`
5. **Datenbank:** Board wird mit User-Account verknuepft
6. **Board bootet:** Board verbindet sich mit Cloud, Server authentifiziert UUID
7. **Web-App:** User oeffnet radiocore.xx/dashboard → sieht sein Board (online/offline)

## Audio-Streaming

### Codec-Wahl: Opus

**Warum Opus?**
- Beste Sprachqualitaet bei niedriger Bitrate (24kbps)
- Open-Source (royalty-free)
- WebRTC-Standard (Browser-Support)
- Niedrige Latenz (~20ms Encoding + Decoding)

### Latenz-Analyse

| Strecke | Latenz |
|---------|--------|
| Board → Cloud (EU) | ~20-30ms |
| Cloud → Browser | ~10-20ms |
| Opus Encoding | ~10ms |
| Opus Decoding | ~10ms |
| **Gesamt** | **~50-70ms** |

**Fuer SSB/FM-Sprachfunk:** 50-70ms ist nicht wahrnehmbar (Telefonie hat ~150ms).

### WebRTC-Alternative (Future)

Statt Server-Relay koennte WebRTC direkte Peer-to-Peer-Verbindung nutzen:

```
Browser ◄──WebRTC──► Board
   ^                    ^
   |                    |
   +---STUN/TURN Server-+
```

**Vorteile:**
- Noch niedrigere Latenz (~20ms)
- Weniger Server-Last

**Nachteile:**
- NAT-Traversal kompliziert (TURN-Server noetig fuer ~20% der Verbindungen)
- Keine Aufzeichnung/Logging moeglich

## Multi-Tenancy

### Isolation

- **User-Isolation:** Jeder User sieht nur seine eigenen Boards
- **Board-Token:** UUID ist geheim, nur Board und Server kennen ihn
- **JWT-Auth:** Clients authentifizieren sich mit JWT
- **Audio-Isolation:** Redis sorgt dafuer, dass Audio nur an berechtigte Clients geht

### Station Sharing (Explizite Freigabe)

User kann Board als "public" markieren:

```sql
UPDATE boards SET public = TRUE WHERE id = '...';
```

Dann erscheint Board in `/api/stations` und andere User koennen buchen.

**Booking-System verhindert Konflikte:**
- Nur ein Gast zur Zeit
- Zeitfenster gesperrt waehrend Buchung
- Owner kann jederzeit uebernehmen (Emergency-Override)

## Skalierung

### Server-Kapazitaet

**Ein Hetzner CX21 (4€/Monat) schafft:**
- ~100 gleichzeitige Audio-Streams (Board ↔ Cloud ↔ Client)
- 24kbps × 100 = 2.4 Mbps Bandbreite
- FastAPI ist async → 1 Worker kann viele WebSockets halten

**Bei 500 Abos:**
- ~50-100 gleichzeitige Verbindungen (10-20% Usage-Rate)
- 2-3 Server (Load-Balancing mit nginx)
- Kosten: ~12€/Monat

**Bei 2000 Abos:**
- ~200-400 gleichzeitige Verbindungen
- ~10 Server (Load-Balancing)
- Kosten: ~40€/Monat

### Load-Balancing

```
Internet
   |
   v
nginx (Load Balancer)
   |
   +----> FastAPI Worker 1 (CX21 #1)
   +----> FastAPI Worker 2 (CX21 #2)
   +----> FastAPI Worker 3 (CX21 #3)
   |
   v
PostgreSQL (Shared)
Redis (Shared)
```

**Sticky Sessions:** WebSocket muss auf gleichem Worker bleiben → nginx `ip_hash`

### Datenbank-Skalierung

- PostgreSQL schafft tausende Users problemlos
- Indizes auf `users.email`, `boards.user_id`, `bookings.start_time`
- Bei >10.000 Usern: PostgreSQL Read-Replicas fuer `/api/stations`

## Sicherheit

### Verschluesselung

- **TLS ueberall:** wss:// und https:// (Let's Encrypt Wildcard-Cert *.radiocore.xx)
- **Board → Cloud:** wss:// (TLS 1.3)
- **Browser → Cloud:** https:// und wss:// (TLS 1.3)

### Authentifizierung

**Boards:**
- UUID in ESP32 Flash (128-bit, einmalig gebrannt)
- UUID ist **nicht** in URL sichtbar (nur beim Pairing per QR-Code)
- Board sendet UUID bei WebSocket-Verbindung
- Server prueft: UUID in Datenbank + mit User verknuepft?

**Clients (Browser):**
- JWT mit Expiry (24h)
- Refresh-Token fuer laengere Sessions
- JWT enthaelt: `user_id`, `exp`, `iat`

### Rate-Limiting

- `/api/auth/login`: 5 Requests/Minute pro IP
- `/api/boards/pair`: 10 Requests/Stunde pro User
- WebSocket: Max 1000 Messages/Sekunde pro Verbindung

### GDPR-Compliance

- **Server-Standort:** Hetzner Rechenzentrum Falkenstein (Deutschland, EU)
- **Daten-Loeschung:** `DELETE FROM users WHERE id = '...'` loescht alles (CASCADE)
- **Daten-Export:** API-Endpoint `/api/export` fuer User-Daten (JSON)
- **Logging:** IP-Adressen anonymisiert nach 7 Tagen

### Abuse-Prevention

- **Station Sharing:** Rating-System (Gaeste bewerten Station, Owner bewertet Gaeste)
- **Blacklist:** User kann andere User blockieren
- **Report-System:** Missbrauch melden → Admin prueft

## Deployment

### Server-Setup

```bash
# Hetzner CX21 bestellen (4€/Monat)
# Ubuntu 24.04 LTS

apt update && apt upgrade -y
apt install -y python3.11 python3-pip postgresql-16 redis nginx certbot

# PostgreSQL einrichten
sudo -u postgres createdb radiocore
sudo -u postgres createuser radiocore_app

# FastAPI installieren
pip3 install fastapi uvicorn sqlalchemy psycopg2 redis

# nginx config
cat > /etc/nginx/sites-available/radiocore <<EOF
server {
    listen 443 ssl http2;
    server_name api.radiocore.xx;

    ssl_certificate /etc/letsencrypt/live/radiocore.xx/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/radiocore.xx/privkey.pem;

    location / {
        proxy_pass http://127.0.0.1:8000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
EOF

# Wildcard-Cert holen
certbot certonly --dns-cloudflare -d '*.radiocore.xx' -d radiocore.xx

# FastAPI starten (systemd service)
uvicorn main:app --host 127.0.0.1 --port 8000
```

### CI/CD (Future)

- **GitHub Actions:** Push auf `main` → Auto-Deploy
- **Tests:** Pytest fuer API-Endpoints
- **Staging:** test.radiocore.xx fuer Tests vor Production

## Monitoring

- **Uptime:** UptimeRobot (extern)
- **Logs:** journalctl fuer systemd services
- **Metrics:** Prometheus + Grafana (optional)
- **Alerts:** E-Mail bei Server-Down

## Zusammenfassung

RadioCore Cloud-Architektur ist:

- **Einfach:** FastAPI + PostgreSQL + Redis
- **Skalierbar:** Load-Balancing mit nginx, guenstige Hetzner-Server
- **Sicher:** TLS, JWT, GDPR-compliant
- **Zuverlaessig:** Offline-First (Board funktioniert ohne Cloud)
- **Kosteneffizient:** 4€/Monat fuer 100 Streams

**Naechste Schritte:**
1. FastAPI Backend implementieren
2. WebSocket-Handler testen (Lokal)
3. Hetzner Server aufsetzen
4. Domain radiocore.xx registrieren
5. Beta-Test mit 5 Prototypen
