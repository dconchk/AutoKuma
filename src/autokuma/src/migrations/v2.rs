use crate::app_state::{AppDBTables, AppState};
use crate::error::Result;
use futures_util::future::BoxFuture;
use kuma_client::Client;

pub fn migrate<'a>(
    _tables: &'a AppDBTables,
    _state: &'a AppState,
    _kuma: &'a Client,
) -> BoxFuture<'a, Result<()>> {
    Box::pin(async { Ok(()) })
}
