---
layout: default
title: "API Reference"
description: "Complete API reference for the Lontar Balinese Dictionary backend service"
prev:
  title: "Contributing Guide"
  url: "/docs/contributing/"
next:
  title: "Changelog"
  url: "/docs/changelog/"
---

# 📡 API Documentation

Complete API reference for the Lontar Balinese Dictionary backend service.

## 🔧 Base Configuration

- **Base URL**: `http://localhost:3000`
- **Content-Type**: `application/json`
- **Authentication**: JWT (for admin endpoints)

---

## 🏥 Health Check

### GET `/health`

Check if the service is running.

**Response:**
```json
"ok"
```

**Status Codes:**
- `200 OK` - Service is healthy
- `503 Service Unavailable` - Service is down

---

## 🔍 Search API

### GET `/entries/search`

Search for dictionary entries with full-text search.

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `q` | string | ✅ | Search query |
| `lang` | string | ❌ | Language filter (`bali`, `id`, `en`) |
| `limit` | number | ❌ | Results limit (default: 20) |
| `offset` | number | ❌ | Results offset (default: 0) |
| `filter` | string | ❌ | Additional filters |

#### Examples

```bash
# Basic search
curl "http://localhost:3000/entries/search?q=padem"

# Language-specific search
curl "http://localhost:3000/entries/search?q=padem&lang=bali"

# With pagination
curl "http://localhost:3000/entries/search?q=padem&limit=10&offset=20"
```

#### Response

```json
{
  "hits": [
    {
      "id": "11111111-1111-1111-1111-111111111111",
      "lemma_latin": "padem",
      "register": "umum",
      "first_sense": "meninggal dunia (untuk orang terhormat)",
      "first_sense_en": "to die (honorific)",
      "domain": "ritual"
    }
  ],
  "estimated_total_hits": 1,
  "limit": 20,
  "offset": 0,
  "processing_time_ms": 0,
  "query": "padem"
}
```

#### Response Fields

| Field | Type | Description |
|-------|------|-------------|
| `hits` | array | Search results |
| `estimated_total_hits` | number | Total matching entries |
| `limit` | number | Results limit used |
| `offset` | number | Results offset used |
| `processing_time_ms` | number | Search processing time |
| `query` | string | Original search query |

---

## 📖 Entry API

### GET `/entries/{lemma}`

Get detailed information about a specific entry.

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `lemma` | string | ✅ | Dictionary entry lemma |

#### Example

```bash
curl "http://localhost:3000/entries/padem"
```

#### Response

```json
{
  "entry": {
    "id": "11111111-1111-1111-1111-111111111111",
    "lemma_latin": "padem",
    "lemma_aksara": null,
    "pos": "verb",
    "status": "published",
    "created_at": "2026-03-31T13:00:00Z",
    "updated_at": "2026-03-31T13:00:00Z"
  },
  "senses": [
    {
      "id": "22222222-2222-2222-2222-222222222222",
      "entry_id": "11111111-1111-1111-1111-111111111111",
      "sense_order": 1,
      "def_indonesian": "meninggal dunia (untuk orang terhormat)",
      "def_english": "to die (honorific)",
      "domain": "ritual"
    }
  ],
  "registers": [
    {
      "entry_id": "11111111-1111-1111-1111-111111111111",
      "level": "umum"
    }
  ],
  "etymologies": []
}
```

### POST `/entries`

Create a new dictionary entry.

#### Request Body

```json
{
  "lemma_latin": "baru",
  "pos": "adjective",
  "status": "draft"
}
```

#### Response

```json
{
  "entry": {
    "id": "new-uuid-here",
    "lemma_latin": "baru",
    "lemma_aksara": null,
    "pos": "adjective",
    "status": "draft",
    "created_at": "2026-03-31T13:00:00Z",
    "updated_at": "2026-03-31T13:00:00Z"
  },
  "senses": [],
  "registers": [],
  "etymologies": []
}
```

### PUT `/entries/{lemma}`

Update an existing entry.

#### Request Body

```json
{
  "pos": "verb",
  "status": "published"
}
```

#### Response

Same format as GET `/entries/{lemma}`.

---

## 📋 Events API

### GET `/entries/{id}/events`

Get the event history for a specific entry.

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | string | ✅ | Entry UUID |

#### Example

```bash
curl "http://localhost:3000/entries/11111111-1111-1111-1111-111111111111/events"
```

#### Response

```json
[
  {
    "id": "event-uuid",
    "entry_id": "11111111-1111-1111-1111-111111111111",
    "editor_id": null,
    "event_type": "create",
    "diff": {
      "lemma_latin": "padem",
      "pos": "verb",
      "status": "draft"
    },
    "created_at": "2026-03-31T13:00:00Z"
  }
]
```

### GET `/events`

Get events with optional filtering.

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `entry_id` | string | ❌ | Filter by entry UUID |
| `event_type` | string | ❌ | Filter by event type |
| `limit` | number | ❌ | Results limit |

#### Example

```bash
# All events
curl "http://localhost:3000/events"

# Events for specific entry
curl "http://localhost:3000/events?entry_id=11111111-1111-1111-1111-111111111111"

# Events by type
curl "http://localhost:3000/events?event_type=create"
```

---

## 📜 Corpus API

### GET `/corpus/{id}`

Get information about a corpus source.

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | string | ✅ | Corpus UUID |

#### Response

```json
{
  "id": "corpus-uuid",
  "title": "Prasasti Sukawana A1",
  "type": "prasasti",
  "date_range": "~882 CE",
  "location_held": "Gedong Kirtya, Singaraja",
  "digitization_status": "complete",
  "source_url": "https://example.com/prasasti-sukawana",
  "description": "Copper plate inscription from the 9th century"
}
```

### GET `/corpus/{id}/attestations`

Get attestations from a specific corpus.

#### Response

```json
[
  {
    "id": "attestation-uuid",
    "entry_lemma": "padem",
    "quote_latin": "...sang hyang paḍem...",
    "quote_translation": "...the honored one who has passed...",
    "page_number": "A1",
    "confidence": 0.95
  }
]
```

---

## 🔐 Authentication (Future)

Authentication endpoints are planned for Phase 4A.

### POST `/auth/login`

Login and receive JWT token.

#### Request Body

```json
{
  "username": "editor@example.com",
  "password": "password"
}
```

#### Response

```json
{
  "token": "jwt-token-here",
  "user": {
    "id": "user-uuid",
    "username": "editor@example.com",
    "role": "editor"
  }
}
```

### POST `/auth/refresh`

Refresh JWT token.

#### Request Body

```json
{
  "token": "expired-jwt-token"
}
```

---

## 📝 Admin API (Future)

Admin endpoints are planned for Phase 4A.

### GET `/admin/entries/queue`

Get entries in the editorial queue.

#### Response

```json
{
  "draft": [
    {
      "id": "entry-uuid",
      "lemma_latin": "word",
      "status": "draft",
      "created_at": "2026-03-31T13:00:00Z"
    }
  ],
  "awaiting_review": [],
  "disputed": []
}
```

### POST `/admin/entries/{id}/publish`

Publish an entry.

#### Response

```json
{
  "id": "entry-uuid",
  "status": "published",
  "published_at": "2026-03-31T13:00:00Z"
}
```

---

## 🚫 Error Responses

All endpoints may return error responses:

```json
{
  "error": "Error message",
  "code": "ERROR_CODE",
  "details": {}
}
```

### Common Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `NOT_FOUND` | 404 | Resource not found |
| `VALIDATION_ERROR` | 400 | Invalid input data |
| `UNAUTHORIZED` | 401 | Authentication required |
| `FORBIDDEN` | 403 | Insufficient permissions |
| `DATABASE_ERROR` | 500 | Database operation failed |
| `SEARCH_ERROR` | 500 | Search operation failed |

---

## 📊 Rate Limiting

Rate limiting is planned for Phase 4B.

- **Search**: 100 requests/minute
- **Entry details**: 200 requests/minute
- **Admin**: 50 requests/minute

---

## 🔍 Search Syntax

The search uses Meilisearch syntax:

### Basic Search
```
padem          # Exact word
padem mati     # Multiple words
```

### Advanced Features (Future)
```
padem*          # Prefix search
"meninggal"     # Phrase search
register:alus  # Field search
-padem          # Negation
```

---

## 🧪 Testing API Endpoints

### Health Check
```bash
curl -v http://localhost:3000/health
```

### Search
```bash
curl -v "http://localhost:3000/entries/search?q=padem&lang=bali"
```

### Entry Details
```bash
curl -v "http://localhost:3000/entries/padem"
```

### Events
```bash
curl -v "http://localhost:3000/events?limit=5"
```

---

## 📚 SDK Examples

### JavaScript/TypeScript

```typescript
class LBDClient {
  constructor(private baseUrl: string) {}

  async search(query: string, lang?: string) {
    const params = new URLSearchParams({ q: query });
    if (lang) params.append('lang', lang);

    const response = await fetch(
      `${this.baseUrl}/entries/search?${params}`
    );
    return response.json();
  }

  async getEntry(lemma: string) {
    const response = await fetch(
      `${this.baseUrl}/entries/${encodeURIComponent(lemma)}`
    );
    return response.json();
  }
}

// Usage
const client = new LBDClient('http://localhost:3000');
const results = await client.search('padem', 'bali');
```

### Python (requests)

```python
import requests

class LBDClient:
    def __init__(self, base_url: str):
        self.base_url = base_url

    def search(self, query: str, lang: str = None):
        params = {'q': query}
        if lang:
            params['lang'] = lang

        response = requests.get(
            f'{self.base_url}/entries/search',
            params=params
        )
        return response.json()

    def get_entry(self, lemma: str):
        response = requests.get(f'{self.base_url}/entries/{lemma}')
        return response.json()

# Usage
client = LBDClient('http://localhost:3000')
results = client.search('padem', 'bali')
```

---

## 🔄 Version History

### v0.1.0 (Current)
- Basic search functionality
- Entry retrieval
- Event logging
- Corpus information

### Planned v0.2.0
- Authentication system
- Admin endpoints
- Rate limiting
- Advanced search features

---

## 📞 Support

### Resources
- **[README](../README.md)** - Project overview and setup
- **[DEVELOPMENT.md](./DEVELOPMENT.md)** - Detailed development guide
- **[TODO](./TODO.md)** - Current roadmap and tasks
- **[Issues](https://github.com/lontar-rs/lbd/issues)** - Open questions and discussions
- **[Discussions](https://github.com/lontar-rs/lbd/discussions)** - Community conversations

### Contact
- **[GitHub Issues](https://github.com/lontar-rs/lbd/issues)** - For API issues
- **[GitHub Discussions](https://github.com/lontar-rs/lbd/discussions)** - For general questions
- **[Maintainers](https://github.com/lontar-rs/lbd)** - For specific project inquiries

---

**API Version: v0.1.0**

**Base URL: http://localhost:3000**

**Last Updated: {{ site.time | date: "%Y-%m-%d" }}**

---

*ᬮᭀᬦ᭄ᬢᬭᬄ ᬩᬲ ᬩᬮᬮᬶ — Lontar Basa Bali*
