use crate::{
    cli::Cli,
    utils::{connect, print_value, ResultOrDie as _},
};
use kuma_client::Config;
use serde_json::json;

pub(crate) async fn handle(config: &Config, cli: &Cli) {
    let client = connect(config, cli).await;
    let monitors = client.get_monitors().await.unwrap_or_die(cli);
    let tags = client.get_tags().await.unwrap_or_die(cli);

    print_value(
        &json!({
            "monitors": monitors,
            "tags": tags,
        }),
        cli,
    );
}
