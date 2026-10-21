use std::sync::Arc;

use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use log::{info, warn};

use crate::{config::Config, metrics::Metrics};

#[derive(Clone)]
struct ServerState {
    metrics: Arc<Metrics>,
    sync_interval: f64,
}

async fn health_handler(State(state): State<ServerState>) -> Response {
    let healthy = state.metrics.is_healthy(state.sync_interval);
    let ts = state
        .metrics
        .last_sync_timestamp
        .load(std::sync::atomic::Ordering::Relaxed);
    let connected = state
        .metrics
        .connected
        .load(std::sync::atomic::Ordering::Relaxed);

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let last_sync_ago = if ts >= 0 { now - ts } else { -1 };

    let body = if healthy {
        format!(
            "{{\"status\":\"healthy\",\"last_sync_ago_seconds\":{last_sync_ago},\"connected\":{connected}}}\n"
        )
    } else {
        let reason = if ts < 0 {
            "never synced".to_owned()
        } else {
            format!(
                "last sync was {last_sync_ago}s ago (threshold: {}s)",
                (state.sync_interval * 3.0) as i64
            )
        };
        format!(
            "{{\"status\":\"unhealthy\",\"reason\":\"{reason}\",\"last_sync_ago_seconds\":{last_sync_ago},\"connected\":{connected}}}\n"
        )
    };

    let status = if healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (status, [("content-type", "application/json")], body).into_response()
}

async fn metrics_handler(State(state): State<ServerState>) -> Response {
    let body = state.metrics.render_prometheus();
    (
        StatusCode::OK,
        [("content-type", "text/plain; version=0.0.4; charset=utf-8")],
        body,
    )
        .into_response()
}

pub async fn start_server(config: Arc<Config>, metrics: Arc<Metrics>) {
    let port = match config.healthcheck_port {
        Some(port) => port,
        None => return,
    };

    let state = ServerState {
        metrics,
        sync_interval: config.sync_interval,
    };

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/metrics", get(metrics_handler))
        .with_state(state);

    let addr = format!("0.0.0.0:{port}");
    match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => {
            info!("Health/metrics server listening on http://{addr}");
            if let Err(e) = axum::serve(listener, app).await {
                warn!("Health/metrics server error: {e}");
            }
        }
        Err(e) => {
            warn!("Failed to bind health/metrics server on {addr}: {e}");
        }
    }
}
