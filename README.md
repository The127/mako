# 📌 Mako – Simple Zero-Trust KV Store for configuration

Mako is a lightweight, persistent key-value configuration store designed for modern distributed environments. It provides a secure, minimal, consistent config API without the complexity of etcd, Consul, or large distributed databases.

---

## 🎯 What Mako Solves

- Persistent config storage across a small cluster
- Simple KV abstraction
- Zero-trust access with OIDC tokens (or admin API key)
- API-side caching for fast reads
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

### 🪶 Simple API

Mako exposes a minimal HTTPS API for:

- Getting key/value pairs
- Lightweight polling for config updates
- Token-authenticated access with simple semantics

---

## 🧠 Design Principles

| Principle | Description |
|------------|-------------|
| Keep It Simple | No massive feature bloat; focus only on config storage + secure access. |
| Zero Trust | Short-lived identities via OIDC; no implicit trust at network level. |
| Persistent, Not Memory-Only | Config survives restarts and doesn’t depend on ephemeral caches for correctness. |
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
