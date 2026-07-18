use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use kuma_client::{
    better_auth::{BetterAuthClient, BetterAuthCredentials},
    Url,
};
use serde_json::{json, Value};
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};
use tokio::{net::TcpListener, sync::Mutex};

const USERNAME: &str = "auth-user";
const PASSWORD: &str = "password-value-that-must-not-leak";
const TOTP: &str = "123456";
const SESSION: &str = "session-value-that-must-not-leak";

#[derive(Default)]
struct FixtureState {
    sign_ins: AtomicUsize,
    totp_checks: AtomicUsize,
    session_checks: AtomicUsize,
    accept_session: AtomicBool,
    requests: Mutex<Vec<(String, HeaderMap, Value)>>,
}

async fn record(state: &FixtureState, path: &str, headers: HeaderMap, body: Value) {
    state
        .requests
        .lock()
        .await
        .push((path.to_owned(), headers, body));
}

async fn sign_in(
    State(state): State<Arc<FixtureState>>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    state.sign_ins.fetch_add(1, Ordering::SeqCst);
    record(&state, "/api/auth/sign-in/username", headers, body.clone()).await;

    if body["username"] != USERNAME || body["password"] != PASSWORD || body["rememberMe"] != false {
        return (
            StatusCode::UNAUTHORIZED,
            HeaderMap::new(),
            Json(json!({"message": "invalid credentials"})),
        );
    }

    let mut response_headers = HeaderMap::new();
    response_headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_static(
            "better-auth.two_factor=challenge; Path=/; HttpOnly; SameSite=Lax",
        ),
    );
    (
        StatusCode::OK,
        response_headers,
        Json(json!({"twoFactorRedirect": true})),
    )
}

async fn verify_totp(
    State(state): State<Arc<FixtureState>>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    state.totp_checks.fetch_add(1, Ordering::SeqCst);
    record(
        &state,
        "/api/auth/two-factor/verify-totp",
        headers.clone(),
        body.clone(),
    )
    .await;

    let challenge = headers
        .get(header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.contains("better-auth.two_factor=challenge"));
    if body["code"] != TOTP || !challenge {
        return (
            StatusCode::UNAUTHORIZED,
            HeaderMap::new(),
            Json(json!({"message": "invalid two factor code"})),
        );
    }

    let mut response_headers = HeaderMap::new();
    response_headers.append(
        header::SET_COOKIE,
        HeaderValue::from_static(
            "better-auth.two_factor=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax",
        ),
    );
    response_headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&format!(
            "better-auth.session_token={SESSION}; Path=/; HttpOnly; SameSite=Lax"
        ))
        .unwrap(),
    );
    (
        StatusCode::OK,
        response_headers,
        Json(json!({"status": true})),
    )
}

async fn get_session(
    State(state): State<Arc<FixtureState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    state.session_checks.fetch_add(1, Ordering::SeqCst);
    record(
        &state,
        "/api/auth/get-session",
        headers.clone(),
        Value::Null,
    )
    .await;

    let has_cookie = headers
        .get(header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.contains(&format!("better-auth.session_token={SESSION}")));

    if state.accept_session.load(Ordering::SeqCst) && has_cookie {
        Json(json!({
            "session": {"id": "session-id"},
            "user": {"id": "user-id", "username": USERNAME}
        }))
    } else {
        Json(Value::Null)
    }
}

async fn fixture() -> (Url, Arc<FixtureState>) {
    let state = Arc::new(FixtureState {
        accept_session: AtomicBool::new(true),
        ..FixtureState::default()
    });
    let app = Router::new()
        .route("/api/auth/sign-in/username", post(sign_in))
        .route("/api/auth/two-factor/verify-totp", post(verify_totp))
        .route("/api/auth/get-session", get(get_session))
        .with_state(state.clone());
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    (Url::parse(&format!("http://{address}/")).unwrap(), state)
}

fn credentials(totp: Option<&str>) -> BetterAuthCredentials {
    BetterAuthCredentials::new(
        USERNAME.to_owned(),
        PASSWORD.to_owned(),
        totp.map(str::to_owned),
    )
}

#[tokio::test]
async fn authenticates_with_totp_and_exposes_only_the_in_memory_cookie_header() {
    let (url, state) = fixture().await;
    let auth = BetterAuthClient::new(url, true, Vec::new(), None).unwrap();

    auth.authenticate(&credentials(Some(TOTP))).await.unwrap();

    assert_eq!(state.sign_ins.load(Ordering::SeqCst), 1);
    assert_eq!(state.totp_checks.load(Ordering::SeqCst), 1);
    assert_eq!(state.session_checks.load(Ordering::SeqCst), 2);

    let cookie = auth.socket_cookie_header().unwrap().unwrap();
    let cookie = cookie.to_str().unwrap();
    assert!(cookie.contains(&format!("better-auth.session_token={SESSION}")));
    assert!(!cookie.contains(PASSWORD));
    assert!(!cookie.contains(TOTP));

    let requests = state.requests.lock().await;
    let sign_in = requests
        .iter()
        .find(|(path, _, _)| path == "/api/auth/sign-in/username")
        .unwrap();
    assert_eq!(sign_in.2["rememberMe"], false);
    let totp = requests
        .iter()
        .find(|(path, _, _)| path == "/api/auth/two-factor/verify-totp")
        .unwrap();
    assert_eq!(totp.2, json!({"code": TOTP}));
}

#[tokio::test]
async fn missing_totp_is_redacted_and_stops_without_retrying() {
    let (url, state) = fixture().await;
    let auth = BetterAuthClient::new(url, true, Vec::new(), None).unwrap();

    let error = auth.authenticate(&credentials(None)).await.unwrap_err();
    let message = error.to_string();

    assert!(message.contains("TOTP"));
    assert!(!message.contains(USERNAME));
    assert!(!message.contains(PASSWORD));
    assert!(!message.contains(SESSION));
    assert_eq!(state.sign_ins.load(Ordering::SeqCst), 1);
    assert_eq!(state.totp_checks.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn invalid_credentials_are_redacted_and_not_retried() {
    let (url, state) = fixture().await;
    let auth = BetterAuthClient::new(url, true, Vec::new(), None).unwrap();
    let rejected_password = "rejected-password-that-must-not-leak";

    let error = auth
        .authenticate(&BetterAuthCredentials::new(
            USERNAME.to_owned(),
            rejected_password.to_owned(),
            Some(TOTP.to_owned()),
        ))
        .await
        .unwrap_err();
    let message = error.to_string();

    assert!(message.contains("401"));
    assert!(!message.contains(USERNAME));
    assert!(!message.contains(rejected_password));
    assert!(!message.contains(TOTP));
    assert_eq!(state.sign_ins.load(Ordering::SeqCst), 1);
    assert_eq!(state.totp_checks.load(Ordering::SeqCst), 0);
    assert_eq!(state.session_checks.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn rejected_new_session_performs_only_one_sign_in_attempt() {
    let (url, state) = fixture().await;
    state.accept_session.store(false, Ordering::SeqCst);
    let auth = BetterAuthClient::new(url, true, Vec::new(), None).unwrap();

    let error = auth
        .authenticate(&credentials(Some(TOTP)))
        .await
        .unwrap_err();
    let message = error.to_string();

    assert!(message.contains("session"));
    assert!(!message.contains(USERNAME));
    assert!(!message.contains(PASSWORD));
    assert!(!message.contains(TOTP));
    assert!(!message.contains(SESSION));
    assert_eq!(state.sign_ins.load(Ordering::SeqCst), 1);
    assert_eq!(state.totp_checks.load(Ordering::SeqCst), 1);
    assert_eq!(state.session_checks.load(Ordering::SeqCst), 2);
}
