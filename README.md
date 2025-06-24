# 🚀 Cadena Rust Backend API

This Rust backend exposes secure cryptographic and DLC (Discreet Log Contract) functionality over HTTP using the Axum web framework. It wraps a Rust-native library (`dlcplazacryptlib`) for working with entropy-based HD wallets, ECDSA signatures, adaptor signatures, and more.

---

## 🌐 Base URL

**Production:**

```
https://api.cadenabitcoin.dev
```

---

## ✅ Exposed Endpoints

### `POST /init_with_entropy`

**Purpose:**  
Initialize the backend with a raw entropy string to derive a wallet and generate an extended public key (XPUB). This must be called before using any other cryptographic endpoint.

**Request Body:**
```json
{
  "entropy": "000102030405060708090a0b0c0d0e0f",
  "network": "bitcoin"
}
```

**Response:**
```json
{
  "xpub": "xpub6D98itZqvQ6bt..."
}
```

---

### `GET /get_public_key/:index`

**Purpose:**  
Derive a public key from the initialized XPUB at the specified index.

**Example:**
```http
GET /get_public_key/0
```

**Response:**
```json
{
  "value": "020f3efe6f42222512d673de03c22bc76aad45f5d0c3b517c88b7399749b2d4c6d"
}
```

---

### `POST /sign_hash_ecdsa`

**Purpose:**  
Signs a 32-byte hash using the private key at the given index.

**Request Body:**
```json
{
  "hash": "c0ffeec0ffeec0ffeec0ffeec0ffeec0ffeec0ffeec0ffeec0ffeec0ffeec0ff",
  "index": 0,
  "signer_pubkey": "020f3efe6f42222512d673de03c22bc76aad45f5d0c3b517c88b7399749b2d4c6d"
}
```

**Response:**
```json
{
  "value": "3045022100e44d63..."
}
```

---

### `POST /create_deterministic_nonce`

**Purpose:**  
Creates a deterministic nonce (secret and public key pair) based on a unique event ID and key index. Used in adaptor signature workflows.

**Request Body:**
```json
{
  "event_id": "btc-2025-07-01",
  "index": 0
}
```

**Response:**
```json
{
  "secret": "4ddae318d6981d97...",
  "public": "02a620dee33883da..."
}
```

---

### `POST /create_cet_adaptor_sigs`

**Purpose:**  
Generates adaptor signatures for a set of Contract Execution Transactions (CETs).

**Request Body:**
```json
{
  "num_digits": 2,
  "num_cets": 4,
  "digit_string_template": "00 01 10 11",
  "oracle_pubkey": "02b463f3cfbc02db...",
  "signing_key_index": 0,
  "signing_pubkey": "020f3efe6f422225...",
  "nonces": "024d3b... 03a201...",
  "interval_wildcards": "0 1 2 3",
  "sighashes": "cafe00... beef00... dead00... beefdead00..."
}
```

**Response:**
```json
{
  "value": "adaptor_sig1 adaptor_sig2 adaptor_sig3 adaptor_sig4"
}
```

---

## 🧠 Behavior

- The backend is **stateful in memory only**.
- You **must call** `/init_with_entropy` each time the server restarts.
- It holds the derived XPUB and keys in a global singleton.

---

## 🔒 Security

- Authentication (JWT) is **not enabled yet** but planned for sensitive endpoints.
- The backend does not persist entropy or private keys.
- Do not expose this server without a proxy or firewall unless you're confident in the trust model.

---

## 🧪 Testing Locally

Example `curl` call:
```bash
curl -X POST https://api.cadenabitcoin.dev/init_with_entropy \
  -H "Content-Type: application/json" \
  -d '{"entropy":"000102030405060708090a0b0c0d0e0f", "network":"bitcoin"}'
```

---

## 📦 Project Layout

- `/src/main.rs`: Axum server with route handlers
- `/dlcplazacryptlib`: Cryptographic logic (HD wallets, ECDSA, adaptor signatures)

---

## 📂 Repository

- [🔗 GitHub](https://github.com/nbell0805/rust_server)

---

## 🛠 Built With

- [Axum](https://github.com/tokio-rs/axum)
- [Serde](https://serde.rs)
- [Tokio](https://tokio.rs)
- [bip39](https://docs.rs/bip39)
