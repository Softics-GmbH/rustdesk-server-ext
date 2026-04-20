// Isolierter Admin-API-Endpoint fuer hbbs und hbbr.
//
// Liefert ausschliesslich Zaehler (keine IDs/IPs/UUIDs), damit keine
// personenbezogenen Daten per HTTP exponiert werden. Wird nur gestartet,
// wenn die Env-Variable ADMIN_API_TOKEN gesetzt ist — sonst bleibt der
// Port geschlossen (Backward-Compat mit Upstream-Deployments).

use std::{future::Future, net::SocketAddr, pin::Pin, sync::Arc};

use axum::{
    extract::Extension,
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::get,
    Router,
};
use hbb_common::{log, ResultType};
use serde_json::{Map, Number, Value};

const HEADER_NAME: &str = "x-admin-token";
const ENV_TOKEN: &str = "ADMIN_API_TOKEN";

/// Liefert den Admin-Token aus der Umgebung, wenn gesetzt und nicht-leer.
pub fn token_from_env() -> Option<String> {
    std::env::var(ENV_TOKEN).ok().filter(|s| !s.is_empty())
}

/// Boxed async closure: `() -> Future<Output = u64>`.
pub type CountFn =
    Arc<dyn Fn() -> Pin<Box<dyn Future<Output = u64> + Send>> + Send + Sync>;

struct AppState {
    token: String,
    response_key: &'static str,
    count_fn: CountFn,
}

/// Startet die Admin-API fuer hbbs. Pfad: `GET /api/peers/count` -> `{"active": N}`.
pub async fn run_peers_api(
    bind: SocketAddr,
    token: String,
    count_fn: CountFn,
) -> ResultType<()> {
    run(bind, token, "/api/peers/count", "active", count_fn).await
}

/// Startet die Admin-API fuer hbbr. Pfad: `GET /api/relay/count` -> `{"sessions": N}`.
pub async fn run_relay_api(
    bind: SocketAddr,
    token: String,
    count_fn: CountFn,
) -> ResultType<()> {
    run(bind, token, "/api/relay/count", "sessions", count_fn).await
}

async fn run(
    bind: SocketAddr,
    token: String,
    count_path: &'static str,
    response_key: &'static str,
    count_fn: CountFn,
) -> ResultType<()> {
    let state = Arc::new(AppState {
        token,
        response_key,
        count_fn,
    });

    let app = Router::new()
        .route("/health", get(health))
        .route(count_path, get(count_handler))
        .layer(Extension(state));

    log::info!(
        "Admin-API listening on {} (endpoints: /health, {})",
        bind,
        count_path
    );

    axum::Server::bind(&bind)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn health() -> &'static str {
    "ok"
}

async fn count_handler(
    headers: HeaderMap,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Value>, StatusCode> {
    let supplied = headers.get(HEADER_NAME).map(|v| v.as_bytes());
    let expected = state.token.as_bytes();

    // Constant-time compare, damit wir Token-Laenge/Inhalt nicht per Timing leaken.
    let authorized = match supplied {
        Some(s) if s.len() == expected.len() => {
            let mut diff: u8 = 0;
            for (a, b) in s.iter().zip(expected.iter()) {
                diff |= a ^ b;
            }
            diff == 0
        }
        _ => false,
    };
    if !authorized {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let count = (state.count_fn)().await;
    let mut obj = Map::new();
    obj.insert(state.response_key.to_string(), Value::Number(Number::from(count)));
    Ok(Json(Value::Object(obj)))
}
