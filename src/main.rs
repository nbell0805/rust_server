use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener; // ✅ Required for axum 0.7
use dlcplazacryptlib::derive_xpub_from_mnemonic;

#[derive(Deserialize)]
struct MnemonicRequest {
    mnemonic: String,
    network: String,
}

#[derive(Serialize)]
struct XpubResponse {
    xpub: String,
}

async fn derive_xpub(Json(payload): Json<MnemonicRequest>) -> Result<Json<XpubResponse>, (axum::http::StatusCode, String)> {
    match derive_xpub_from_mnemonic(&payload.mnemonic, &payload.network) {
        Ok(xpub) => Ok(Json(XpubResponse { xpub })),
        Err(e) => Err((axum::http::StatusCode::BAD_REQUEST, e)),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/derive_xpub", post(derive_xpub));
    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    println!("🔌 Server running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap(); // ✅ Axum 0.7 style
    axum::serve(listener, app).await.unwrap();             // ✅ Axum 0.7 style
}
