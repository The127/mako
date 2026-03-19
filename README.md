# 🦈 Mako – Simple Zero-Trust KV Store for configuration

Mako is a lightweight, persistent key-value configuration store designed for modern distributed environments. It provides a secure, minimal, consistent config API without the complexity of etcd, Consul, or large distributed databases.

Mako is built on top of [rqlite](https://github.com/rqlite/rqlite), a lightweight, distributed SQL database built on top of Raft.

---

## Feature complete

Mako is considered **feature complete** and ready for production use.

Bugfixes and security updates are applied as needed.
New features will be considered for inclusion in the future if the need arises or if they are requested.

---

## 🎯 What Mako Solves

- Persistent config storage across a small cluster
- Simple KV abstraction
- Zero-trust access with OIDC tokens (or admin API key)
- API-side caching for fast reads
- Efficient client polling via HTTP caching (ETags)
- Designed for hundreds to thousands of clients

---

## 🚀 Key Features

### ✅ Persistent & Durable

Mako uses a Raft-backed embedded store (via rqlite) to ensure data is replicated and consistent across nodes, with persistent on-disk storage.

### 🔐 Zero-Trust Auth

Clients authenticate using short-lived OIDC tokens obtained via a secure token exchange. This ensures:

- No shared static credentials
- Easy revocation through the identity provider
- Secure identity propagation without a service mesh

### ⚡ Fast Reads with RAM Cache

API instances keep config data in memory for very fast read performance. Most reads are served directly from cache with minimal load on the backend store.

### 🏷️ ETag Support (Efficient Polling)

`GET` responses include an `ETag` representing the current version of the requested key.

Clients can send `If-None-Match` on subsequent reads:

- If the value has **not** changed, the API returns **`304 Not Modified`** (no body)
- If the value **has** changed, the API returns **`200 OK`** with the updated value and a new `ETag`

This is the recommended way to implement lightweight polling without repeatedly downloading unchanged configuration.

### 🪶 Simple API

Mako exposes a minimal HTTP API for:

- Getting and setting key/value pairs (with ETag-based conditional requests)
- Token-authenticated access with simple semantics

---

## 🏃 Running the Server

### Docker (recommended)

```sh
docker run -d \
  -p 8080:8080 \
  -e MAKO_DATABASE_CONNECTION=http://rqlite:4001 \
  -e MAKO_ISSUER=https://your-idp.example.com/oidc/mako \
  -e MAKO_CLIENT_ID=mako \
  ghcr.io/the127/mako:latest
```

### Configuration

| Flag | Env var | Default | Description |
|------|---------|---------|-------------|
| `--host` | `MAKO_HOST` | `0.0.0.0` | Listen address |
| `--port` | `MAKO_PORT` | `8080` | Listen port |
| `--database-connection` | `MAKO_DATABASE_CONNECTION` | — | rqlite connection URL |
| `--issuer` | `MAKO_ISSUER` | — | OIDC issuer URL |
| `--client-id` | `MAKO_CLIENT_ID` | — | OIDC client ID |
| `--admin-role` | `MAKO_ADMIN_ROLE` | `mako:admin` | Role granting full access |
| `--writer-role` | `MAKO_WRITER_ROLE` | `mako:writer` | Role granting write access to permitted namespaces |
| `--reader-role` | `MAKO_READER_ROLE` | `mako:reader` | Role granting read access to permitted namespaces |

An admin API token can be set via the `MAKO_ADMIN_TOKEN` environment variable. Requests using this token bypass OIDC entirely and have full access.

### Dependencies

Mako requires a running [rqlite](https://github.com/rqlite/rqlite) instance. A minimal `compose.yaml`:

```yaml
services:
  rqlite:
    image: rqlite/rqlite
    command: ["-fk"]
    ports:
      - "4001:4001"

  mako:
    image: ghcr.io/the127/mako:latest
    ports:
      - "8080:8080"
    environment:
      MAKO_DATABASE_CONNECTION: http://rqlite:4001
      MAKO_ISSUER: https://your-idp.example.com/oidc/mako
      MAKO_CLIENT_ID: mako
    depends_on:
      - rqlite
```

---

## 💻 CLI

### Installation

Download the latest binary for your platform from the [releases page](https://github.com/The127/mako/releases):

| Platform | Binary |
|----------|--------|
| Linux x86_64 | `mako-linux-x86_64` |
| Linux aarch64 | `mako-linux-aarch64` |
| macOS Apple Silicon | `mako-macos-aarch64` |
| macOS Intel | `mako-macos-x86_64` |
| Windows x86_64 | `mako-windows-x86_64.exe` |

```sh
# Example: Linux x86_64
curl -L https://github.com/The127/mako/releases/latest/download/mako-linux-x86_64 -o mako
chmod +x mako
sudo mv mako /usr/local/bin/
```

### Authentication

Mako uses OIDC device flow for authentication. Run once to log in:

```sh
mako auth login --issuer https://your-idp.example.com/oidc/mako --client-id mako
```

Credentials are saved to `~/.config/mako/credentials.json` and used automatically for subsequent commands.

You can also set the issuer and client ID via environment variables to avoid passing them every time:

```sh
export MAKO_OIDC_ISSUER=https://your-idp.example.com/oidc/mako
export MAKO_OIDC_CLIENT_ID=mako
```

### Usage

All commands require `--url` (or `MAKO_URL`) pointing to the Mako server:

```sh
export MAKO_URL=http://localhost:8080
```

#### Namespaces

```sh
mako namespaces list
mako namespaces create <path>
mako namespaces delete <path>
mako namespaces kvs <path>       # list all keys in a namespace
```

#### Key/Value

```sh
mako kv set <namespace> <key> <value>
mako kv get <namespace> <key>
mako kv delete <namespace> <key>
```

#### ACL

Access to KV entries is controlled per-namespace, per-subject (OIDC `sub`). Users with the admin role bypass ACL checks entirely.

```sh
mako acl set <namespace> <subject> <permissions...>   # permissions: read, write
mako acl get <namespace> <subject>
mako acl delete <namespace> <subject>
mako acl list <namespace>
```

#### Output format

All commands support `--format` (`MAKO_FORMAT`): `plain` (default) or `json`.

```sh
mako --format json kv get myapp version
```

---

## 🧠 Design Principles

| Principle | Description |
|------------|-------------|
| Keep It Simple | No massive feature bloat; focus only on config storage + secure access. |
| Zero Trust | Short-lived identities via OIDC; no implicit trust at network level. |
| Persistent, Not Memory-Only | Config survives restarts and doesn't depend on ephemeral caches for correctness. |
| Easy to Operate | Designed for easy deployment and low operational burden. |

## 📈 When to Use Mako

### Ideal for:

- A simple, consistent config store
- Secure identity validation without a mesh
- Persistent storage with minimal distributed complexity
- Hundreds to thousands of config readers

### Not suitable for:

- Massive-scale distributed data replication beyond config
- Advanced coordination primitives (leader election, locks, etc.)
- Heavy transactional workloads
