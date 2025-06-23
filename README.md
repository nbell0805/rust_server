# Cadena Rust Backend API

This backend exposes secure wallet and DLC-related functionality over HTTP using a Rust + Axum server. It wraps internal cryptographic logic from `dlcplazacryptlib`.

## 📡 Base URL

**Production**: `https://api.cadenabitcoin.dev`

All requests should be made to this base URL.

## ✅ Currently Exposed Endpoint

---

### `POST https://api.cadenabitcoin.dev/derive_xpub`

Derives a master extended public key (XPUB) from a BIP-39 mnemonic phrase.

#### Request

```json
{
  "mnemonic": "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
  "network": "bitcoin"
}
```

- `mnemonic`: A valid BIP-39 12 or 24 word phrase.
- `network`: One of `"bitcoin"` or `"signet"`.

#### Response

```json
{
  "xpub": "xpub6CatWdiZiodmUeTDp8LT5or8nmbKNcuyvz7WyksVFkKB4RHwCD3XyuvPEbvqAQY3rAPshWcMLoP2fMFMKHPJ4ZeZXYVUhLv1VMrjPC7PW6V"
}
```

#### Errors

- `400 Bad Request`: If the mnemonic or network is invalid
- `500 Internal Server Error`: If derivation fails

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

