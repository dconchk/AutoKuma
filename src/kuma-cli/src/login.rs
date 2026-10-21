use crate::{
    cli::Cli,
    utils::{self, print_value},
};
use clap::{CommandFactory, Parser};
use kuma_client::Config;
use serde_json::json;

fn login_success() -> serde_json::Value {
    json!({"ok": true, "message": "login ok"})
}

#[derive(Parser, Clone, Debug)]
#[command()]
pub(crate) struct Command {
    /// The username for logging in
    username: Option<String>,

    /// The password for logging in
    password: Option<String>,

    /// Clear any stored auth token
    #[arg(long)]
    clear: bool,
}

pub(crate) async fn handle(command: &Command, config: &Config, cli: &Cli) {
    if command.clear {
        utils::clear_auth_token().await;
        print_value(
            &json!({"ok": true, "message" : "legacy auth token cleared"}),
            cli,
        );
        return;
    }

    if command.username.is_none() {
        Cli::command()
            .error(
                clap::error::ErrorKind::MissingRequiredArgument,
                "the following required arguments were not provided:\n  \x1b[32m<USERNAME|--clear>\x1b[0m",
            )
            .exit();
    }

    let username = command.username.clone().unwrap();
    let password = command
        .password
        .clone()
        .unwrap_or_else(|| rpassword::prompt_password("Password: ").unwrap());

    let config = Config {
        username: Some(username),
        password: Some(password),
        ..config.clone()
    };

    let _client = utils::connect(&config, cli).await;
    print_value(&login_success(), cli);
}

#[cfg(test)]
mod tests {
    #[test]
    fn login_success_never_contains_reusable_session_material() {
        let value = super::login_success();
        assert_eq!(
            value,
            serde_json::json!({"ok": true, "message": "login ok"})
        );
        assert!(value.get("token").is_none());
        assert!(value.get("cookie").is_none());
    }
}
