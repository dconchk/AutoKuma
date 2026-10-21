use crate::app_state::{AppDBTables, AppState};
use crate::error::Result;
use crate::name::Name;
use futures_util::future::BoxFuture;
use itertools::Itertools as _;
use kuma_client::Client;
use log::{error, info};
use std::env;

pub fn migrate<'a>(
    _tables: &'a AppDBTables,
    state: &'a AppState,
    kuma: &'a Client,
) -> BoxFuture<'a, Result<()>> {
    Box::pin(async move {
        let autokuma_tag = kuma
            .get_tags()
            .await?
            .iter()
            .find(|x| {
                x.name
                    .as_ref()
                    .is_some_and(|name| name == &state.config.tag_name)
            })
            .map(|tag| tag.tag_id)
            .flatten();

        if let Some(autokuma_tag) = autokuma_tag {
            if !env::var("AUTOKUMA__MIGRATE").is_ok_and(|x| x == "true") {
                error!(
                "Migration required, but AUTOKUMA__MIGRATE is not set to 'true', refusing to continue to avoid data loss. Please read the CHANGELOG and then set AUTOKUMA__MIGRATE=true to continue."
            );
                return Ok(());
            }

            let entries = kuma
                .get_monitors()
                .await?
                .iter()
                .filter_map(|(_, monitor)| {
                    monitor
                        .common()
                        .tags()
                        .iter()
                        .find(|x| x.tag_id == Some(autokuma_tag))
                        .map(|tag| tag.value.clone())
                        .flatten()
                        .map(|name| (name, monitor.common().id().unwrap_or(-1)))
                })
                .collect_vec();

            info!("Migrating {} monitors", entries.len());

            for (name, id) in entries {
                state.db.store_id(Name::Monitor(name), id)?;
            }

            kuma.delete_tag(autokuma_tag).await?;
        }

        Ok(())
    })
}
