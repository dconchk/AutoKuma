use crate::{
    error::{Error, Result},
    Url,
};
use reqwest::{
    cookie::{CookieStore, Jar},
    header::{HeaderMap, HeaderName, HeaderValue},
    Certificate, Client,
};
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;

const SIGN_IN_PATH: &str = "api/auth/sign-in/username";
const VERIFY_TOTP_PATH: &str = "api/auth/two-factor/verify-totp";
const SESSION_PATH: &str = "api/auth/get-session";

/// Runtime-only credentials for a Better Auth session.
///
/// Deliberately does not implement `Debug` so credential values cannot be
/// printed by ordinary diagnostic formatting.
pub struct BetterAuthCredentials {
    username: String,
    password: String,
    totp: Option<String>,
}

impl BetterAuthCredentials {
    pub fn new(username: String, password: String, totp: Option<String>) -> Self {
        Self {
            username,
            password,
            totp,
        }
    }
}

/// Better Auth HTTP client with a process-local cookie jar.
pub struct BetterAuthClient {
    base_url: Url,
    client: Client,
    cookies: Arc<Jar>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SignInRequest<'a> {
    username: &'a str,
    password: &'a str,
    remember_me: bool,
}

#[derive(Serialize)]
struct TotpRequest<'a> {
    code: &'a str,
}

impl BetterAuthClient {
    pub fn new(
        base_url: Url,
        verify_tls: bool,
        headers: Vec<String>,
        root_certificate: Option<Certificate>,
    ) -> Result<Self> {
        let cookies = Arc::new(Jar::default());
        let mut builder = Client::builder()
            .danger_accept_invalid_certs(!verify_tls)
            .default_headers(parse_headers(&headers))
            .cookie_provider(cookies.clone());

        if let Some(certificate) = root_certificate {
            builder = builder.add_root_certificate(certificate);
        }

        Ok(Self {
            base_url,
            client: builder.build()?,
            cookies,
        })
    }

    pub fn http_client(&self) -> Client {
        self.client.clone()
    }

    /// Return the session cookie header for the initial Socket.IO request.
    ///
    /// The value remains in memory and must never be logged or persisted.
    pub fn socket_cookie_header(&self) -> Result<Option<HeaderValue>> {
        let socket_url = self
            .base_url
            .join("socket.io/")
            .map_err(|error| Error::InvalidUrl(error.to_string()))?;
        Ok(self.cookies.cookies(&socket_url))
    }

    /// Reuse a valid process-local session or perform one sign-in sequence.
    ///
    /// A sign-in sequence contains at most one username/password request and
    /// one TOTP request. Session rejection after that sequence is terminal for
    /// this call.
    pub async fn authenticate(&self, credentials: &BetterAuthCredentials) -> Result<()> {
        if self.validate_session().await? {
            return Ok(());
        }

        let response = self
            .client
            .post(self.url(SIGN_IN_PATH)?)
            .json(&SignInRequest {
                username: &credentials.username,
                password: &credentials.password,
                remember_me: false,
            })
            .send()
            .await?;
        let status = response.status();
        if !status.is_success() {
            return Err(Error::LoginError(format!(
                "Better Auth sign-in failed with HTTP {}",
                status.as_u16()
            )));
        }

        let body: Value = response.json().await?;
        if body
            .get("twoFactorRedirect")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            let token = credentials.totp.as_deref().ok_or(Error::TokenRequired)?;
            let response = self
                .client
                .post(self.url(VERIFY_TOTP_PATH)?)
                .json(&TotpRequest { code: token })
                .send()
                .await?;
            let status = response.status();
            if !status.is_success() {
                return Err(Error::LoginError(format!(
                    "Better Auth TOTP verification failed with HTTP {}",
                    status.as_u16()
                )));
            }
        }

        if !self.validate_session().await? {
            return Err(Error::LoginError(
                "Better Auth session validation failed".to_owned(),
            ));
        }

        Ok(())
    }

    async fn validate_session(&self) -> Result<bool> {
        let response = self.client.get(self.url(SESSION_PATH)?).send().await?;
        let status = response.status();
        if !status.is_success() {
            return Err(Error::LoginError(format!(
                "Better Auth session validation failed with HTTP {}",
                status.as_u16()
            )));
        }

        let body: Value = response.json().await?;
        Ok(!body.is_null()
            && body.get("session").is_some_and(Value::is_object)
            && body.get("user").is_some_and(Value::is_object))
    }

    fn url(&self, path: &str) -> Result<Url> {
        self.base_url
            .join(path)
            .map_err(|error| Error::InvalidUrl(error.to_string()))
    }
}

fn parse_headers(headers: &[String]) -> HeaderMap {
    HeaderMap::from_iter(
        headers
            .iter()
            .filter_map(|header| header.split_once('='))
            .filter_map(|(key, value)| {
                match (
                    HeaderName::from_bytes(key.as_bytes()),
                    HeaderValue::from_bytes(value.as_bytes()),
                ) {
                    (Ok(key), Ok(value)) => Some((key, value)),
                    _ => None,
                }
            }),
    )
}
