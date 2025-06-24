use axum::{
    routing::{get, post},
    extract::{Path},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use axum::http::StatusCode;

use dlcplazacryptlib::{
    init_with_entropy_intern as init_with_entropy,
    get_public_key_intern as get_public_key,
    sign_hash_ecdsa_intern as sign_hash_ecdsa,
    create_cet_adaptor_sigs_intern as create_cet_adaptor_sigs,
    create_deterministic_nonce_intern as create_deterministic_nonce,
};

// ----------------------------
// Request/Response Models
// ----------------------------

#[derive(Deserialize)]
struct InitEntropyRequest {
    entropy: String,
    network: String,
}

#[derive(Deserialize)]
struct SignHashRequest {
    hash: String,
    index: u32,
    signer_pubkey: String,
}

#[derive(Deserialize)]
struct CETAdaptorSigsRequest {
    num_digits: u8,
    num_cets: u64,
    digit_string_template: String,
    oracle_pubkey: String,
    signing_key_index: u32,
    signing_pubkey: String,
    nonces: String,
    interval_wildcards: String,
    sighashes: String,
}

#[derive(Deserialize)]
struct DeterministicNonceRequest {
    event_id: String,
    index: u32,
}

#[derive(Serialize)]
struct XpubResponse {
    xpub: String,
}

#[derive(Serialize)]
struct StringResponse {
    value: String,
}

#[derive(Serialize)]
struct KeyPairResponse {
    secret: String,
    public: String,
}

// ----------------------------
// Route Handlers
// ----------------------------

async fn init_with_entropy_handler(Json(payload): Json<InitEntropyRequest>) -> Result<Json<XpubResponse>, (StatusCode, String)> {
    match init_with_entropy(&payload.entropy, &payload.network) {
        Ok(xpub) => Ok(Json(XpubResponse { xpub })),
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
    }
}

async fn get_public_key_handler(Path(index): Path<u32>) -> Result<Json<StringResponse>, (StatusCode, String)> {
    match get_public_key(index) {
        Ok(pubkey) => Ok(Json(StringResponse { value: pubkey })),
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
    }
}

async fn sign_hash_handler(Json(payload): Json<SignHashRequest>) -> Result<Json<StringResponse>, (StatusCode, String)> {
    match sign_hash_ecdsa(&payload.hash, payload.index, &payload.signer_pubkey) {
        Ok(sig) => Ok(Json(StringResponse { value: sig })),
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
    }
}

async fn create_cet_adaptor_sigs_handler(Json(payload): Json<CETAdaptorSigsRequest>) -> Result<Json<StringResponse>, (StatusCode, String)> {
    match create_cet_adaptor_sigs(
        payload.num_digits,
        payload.num_cets,
        &payload.digit_string_template,
        &payload.oracle_pubkey,
        payload.signing_key_index,
        &payload.signing_pubkey,
        &payload.nonces,
        &payload.interval_wildcards,
        &payload.sighashes,
    ) {
        Ok(sigs) => Ok(Json(StringResponse { value: sigs })),
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
    }
}

async fn create_deterministic_nonce_handler(Json(payload): Json<DeterministicNonceRequest>) -> Result<Json<KeyPairResponse>, (StatusCode, String)> {
    match create_deterministic_nonce(&payload.event_id, payload.index) {
        Ok((secret, public)) => Ok(Json(KeyPairResponse { secret, public })),
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
    }
}

// ----------------------------
// Main Entrypoint
// ----------------------------

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/init_with_entropy", post(init_with_entropy_handler))
        .route("/get_public_key/:index", get(get_public_key_handler))
        .route("/sign_hash_ecdsa", post(sign_hash_handler))
        .route("/create_cet_adaptor_sigs", post(create_cet_adaptor_sigs_handler))
        .route("/create_deterministic_nonce", post(create_deterministic_nonce_handler));

    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    println!("🚀 Server running on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
