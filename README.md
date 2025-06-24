# Cadena Rust Backend API

This backend exposes secure wallet and DLC-related functionality over HTTP using a Rust + Axum server. It wraps internal cryptographic logic from `dlcplazacryptlib`.

## 📡 Base URL

**Production**: `https://api.cadenabitcoin.dev`

All requests should be made to this base URL.

## ✅ Currently Exposed Endpoint

---

## 🔐 Endpoints

### `POST /init_with_entropy`
Initialize the system with a raw entropy hex string.

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
  "xpub": "xpub..."
}
```

---

### `GET /get_xpub`
Returns the currently initialized XPUB.

**Response:**
```json
{
  "xpub": "xpub..."
}
```

---

### `GET /get_public_key/:index`
Returns the public key at the given child index.

**Response:**
```json
{
  "value": "<public_key_hex>"
}
```

---

### `GET /get_address/:index`
Returns the Bitcoin address at the given child index.

**Response:**
```json
{
  "value": "<bitcoin_address>"
}
```

---

## 🚧 More Endpoints Coming Soon

This backend will expose additional endpoints such as:

- `/init_with_entropy`
- `/get_public_key`
- `/sign_hash_ecdsa`
- `/create_cet_adaptor_sigs`
- and others...

They’ll be documented here as they are made available.

---

## 🔒 Security Notes

- All endpoints should eventually require API keys or JWTs for authentication.
- Avoid exposing signing or key-manipulation functions without user verification.

## 📂 Source Repositories

- Rust Server: https://github.com/nbell0805/rust_server
- App Frontend: (coming soon)

