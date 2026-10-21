use crate::{
    config::Config,
    error::{Error, Result},
};
use serde_json::json;
use std::error::Error as StdError;
use std::{collections::BTreeMap, sync::Arc};
use tera::Tera;
pub fn print_error_chain(error: &dyn StdError) -> String {
    let mut result = "\n".to_owned();
    let mut current_error = Some(error);

    while let Some(err) = current_error {
        result.push_str(&format!("Caused by: {}\n", err));
        current_error = err.source();
    }

    result
}

pub fn group_by_prefix<A, B, I>(v: I, delimiter: &str) -> BTreeMap<String, Vec<(String, String)>>
where
    A: AsRef<str>,
    B: AsRef<str>,
    I: IntoIterator<Item = (A, B)>,
{
    v.into_iter()
        .fold(BTreeMap::new(), |mut groups, (key, value)| {
            if let Some((prefix, key)) = key.as_ref().split_once(delimiter) {
                groups
                    .entry(prefix.to_owned())
                    .or_default()
                    .push((key.to_owned(), value.as_ref().to_owned()));
            }
            groups
        })
}

pub trait ResultOrDie<T> {
    fn unwrap_or_die(self, exit_code: i32) -> T;
}

impl<T, E> ResultOrDie<T> for std::result::Result<T, E> {
    fn unwrap_or_die(self, exit_code: i32) -> T {
        match self {
            Ok(t) => t,
            Err(_) => std::process::exit(exit_code),
        }
    }
}

pub trait FlattenValue {
    fn flatten(&self) -> Result<Vec<(String, serde_json::Value)>>;
}

impl FlattenValue for serde_json::Value {
    fn flatten(&self) -> Result<Vec<(String, serde_json::Value)>> {
        let mut map = serde_json::Map::new();
        insert_object(
            &mut map,
            None,
            self.as_object()
                .ok_or_else(|| Error::DeserializeError("Not an object".to_string()))?,
        )?;
        Ok(map.into_iter().collect())
    }
}

fn insert_object(
    base_json: &mut serde_json::Map<String, serde_json::Value>,
    base_key: Option<&str>,
    object: &serde_json::Map<String, serde_json::Value>,
) -> Result<()> {
    for (key, value) in object {
        let new_key = base_key.map_or_else(|| key.clone(), |base_key| format!("{base_key}.{key}"));

        if let Some(object) = value.as_object() {
            insert_object(base_json, Some(&new_key), object)?;
        } else if let Some(array) = value.as_array() {
            base_json.insert(
                new_key.to_string(),
                json!(serde_json::to_string(&array)
                    .map_err(|e| Error::DeserializeError(e.to_string()))?),
            );
        } else {
            base_json.insert(new_key.to_string(), json!(value));
        }
    }

    Ok(())
}

struct GetEnvFunction {
    config: Arc<Config>,
}

impl tera::Function for GetEnvFunction {
    fn call(
        &self,
        args: &std::collections::HashMap<String, tera::Value>,
    ) -> tera::Result<tera::Value> {
        let name = match args.get("name") {
            Some(val) => match tera::from_value::<String>(val.clone()) {
                Ok(v) => v,
                Err(_) => {
                    return Err(tera::Error::msg(format!(
                        "Function `get_env` received name={} but `name` can only be a string",
                        val
                    )));
                }
            },
            None => {
                return Err(tera::Error::msg(
                    "Function `get_env` didn't receive a `name` argument",
                ))
            }
        };

        if !self.config.insecure_env_access && !name.starts_with("AUTOKUMA__ENV__") {
            return Err(tera::Error::msg(format!(
                "Access to environment variable `{}` is not allowed",
                &name
            )));
        }

        match std::env::var(&name).ok() {
            Some(res) => Ok(tera::Value::String(res)),
            None => match args.get("default") {
                Some(default) => Ok(default.clone()),
                None => Err(tera::Error::msg(format!(
                    "Environment variable `{}` not found",
                    &name
                ))),
            },
        }
    }
}

struct JsonEscapeFilter;

impl tera::Filter for JsonEscapeFilter {
    fn filter(
        &self,
        value: &tera::Value,
        _: &std::collections::HashMap<String, tera::Value>,
    ) -> tera::Result<tera::Value> {
        let Some(s) = value.as_str() else {
            return Ok(value.clone());
        };
        let json = serde_json::to_string(s).map_err(|e| tera::Error::msg(e.to_string()))?;
        Ok(tera::Value::String(json[1..json.len() - 1].to_owned()))
    }
}

struct JsonUnescapeFilter;

impl tera::Filter for JsonUnescapeFilter {
    fn filter(
        &self,
        value: &tera::Value,
        _: &std::collections::HashMap<String, tera::Value>,
    ) -> tera::Result<tera::Value> {
        let Some(s) = value.as_str() else {
            return Ok(value.clone());
        };
        let wrapped = format!("\"{}\"", s);
        let unescaped: String =
            serde_json::from_str(&wrapped).map_err(|e| tera::Error::msg(e.to_string()))?;
        Ok(tera::Value::String(unescaped))
    }
}

struct TomlEscapeFilter;

impl tera::Filter for TomlEscapeFilter {
    fn filter(
        &self,
        value: &tera::Value,
        _: &std::collections::HashMap<String, tera::Value>,
    ) -> tera::Result<tera::Value> {
        let Some(s) = value.as_str() else {
            return Ok(value.clone());
        };
        let mut result = String::with_capacity(s.len());
        for c in s.chars() {
            match c {
                '\u{0008}' => result.push_str(r"\b"),
                '\t' => result.push_str(r"\t"),
                '\n' => result.push_str(r"\n"),
                '\u{000C}' => result.push_str(r"\f"),
                '\r' => result.push_str(r"\r"),
                '"' => result.push_str("\\\""),
                '\\' => result.push_str(r"\\"),
                c if (c as u32) < 0x20 => result.push_str(&format!("\\u{:04X}", c as u32)),
                c => result.push(c),
            }
        }
        Ok(tera::Value::String(result))
    }
}

struct TomlUnescapeFilter;

impl tera::Filter for TomlUnescapeFilter {
    fn filter(
        &self,
        value: &tera::Value,
        _: &std::collections::HashMap<String, tera::Value>,
    ) -> tera::Result<tera::Value> {
        let Some(s) = value.as_str() else {
            return Ok(value.clone());
        };
        let table_str = format!("v = \"{}\"", s);
        let table: toml::Table =
            toml::from_str(&table_str).map_err(|e| tera::Error::msg(e.to_string()))?;
        let result = table
            .get("v")
            .and_then(|v| v.as_str())
            .ok_or_else(|| tera::Error::msg("toml_unescape: failed to extract value"))?
            .to_owned();
        Ok(tera::Value::String(result))
    }
}

pub fn fill_templates(
    config: Arc<Config>,
    template: impl Into<String>,
    template_values: &tera::Context,
) -> Result<String> {
    let template = template.into();
    let mut tera = Tera::default();
    let get_env = GetEnvFunction {
        config: config.clone(),
    };

    tera.register_function("get_env", get_env);
    tera.register_filter("json_escape", JsonEscapeFilter);
    tera.register_filter("json_unescape", JsonUnescapeFilter);
    tera.register_filter("toml_escape", TomlEscapeFilter);
    tera.register_filter("toml_unescape", TomlUnescapeFilter);

    tera.add_raw_template(&template, &template)
        .and_then(|_| tera.render(&template, template_values))
        .map_err(|e| Error::LabelParseError(print_error_chain(&e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    fn test_config() -> Arc<Config> {
        Arc::new(
            serde_json::from_value(json!({
                "kuma": {"url": "http://localhost:3001", "tls": {}},
                "docker": {},
                "files": {},
                "kubernetes": {}
            }))
            .unwrap(),
        )
    }

    fn render(template: &str, context: tera::Context) -> String {
        fill_templates(test_config(), template, &context).unwrap()
    }

    fn ctx(key: &str, value: &str) -> tera::Context {
        let mut c = tera::Context::new();
        c.insert(key, value);
        c
    }

    #[test]
    fn json_escape_escapes_double_quotes() {
        assert_eq!(
            render("{{ v | json_escape }}", ctx("v", r#"hello "world""#)),
            r#"hello \"world\""#
        );
    }

    #[test]
    fn json_escape_escapes_newlines() {
        assert_eq!(
            render("{{ v | json_escape }}", ctx("v", "line1\nline2")),
            r#"line1\nline2"#
        );
    }

    #[test]
    fn json_escape_escapes_backslashes() {
        assert_eq!(
            render("{{ v | json_escape }}", ctx("v", r#"back\slash"#)),
            r#"back\\slash"#
        );
    }

    #[test]
    fn json_unescape_unescapes_double_quotes() {
        assert_eq!(
            render("{{ v | json_unescape }}", ctx("v", r#"hello \"world\""#)),
            r#"hello "world""#
        );
    }

    #[test]
    fn json_unescape_unescapes_newlines() {
        assert_eq!(
            render("{{ v | json_unescape }}", ctx("v", r#"line1\nline2"#)),
            "line1\nline2"
        );
    }

    #[test]
    fn json_escape_unescape_round_trip() {
        let raw = "password: \"p@ss/w0rd\"\nwith newline";
        let result = render(
            "{{ v | json_escape | json_unescape }}",
            ctx("v", raw),
        );
        assert_eq!(result, raw);
    }

    #[test]
    fn toml_escape_escapes_double_quotes() {
        assert_eq!(
            render("{{ v | toml_escape }}", ctx("v", r#"hello "world""#)),
            r#"hello \"world\""#
        );
    }

    #[test]
    fn toml_escape_escapes_newlines() {
        assert_eq!(
            render("{{ v | toml_escape }}", ctx("v", "line1\nline2")),
            r#"line1\nline2"#
        );
    }

    #[test]
    fn toml_escape_escapes_backslashes() {
        assert_eq!(
            render("{{ v | toml_escape }}", ctx("v", r#"back\slash"#)),
            r#"back\\slash"#
        );
    }

    #[test]
    fn toml_unescape_unescapes_double_quotes() {
        assert_eq!(
            render("{{ v | toml_unescape }}", ctx("v", r#"hello \"world\""#)),
            r#"hello "world""#
        );
    }

    #[test]
    fn toml_unescape_unescapes_newlines() {
        assert_eq!(
            render("{{ v | toml_unescape }}", ctx("v", r#"line1\nline2"#)),
            "line1\nline2"
        );
    }

    #[test]
    fn toml_escape_unescape_round_trip() {
        let raw = "password: \"p@ss/w0rd\"\nwith newline";
        let result = render(
            "{{ v | toml_escape | toml_unescape }}",
            ctx("v", raw),
        );
        assert_eq!(result, raw);
    }

    #[test]
    fn json_escape_passes_through_non_string() {
        let mut c = tera::Context::new();
        c.insert("v", &42);
        assert_eq!(render("{{ v | json_escape }}", c), "42");
    }

    #[test]
    fn json_unescape_passes_through_non_string() {
        let mut c = tera::Context::new();
        c.insert("v", &true);
        assert_eq!(render("{{ v | json_unescape }}", c), "true");
    }

    #[test]
    fn toml_escape_passes_through_non_string() {
        let mut c = tera::Context::new();
        c.insert("v", &3.14f64);
        assert_eq!(render("{{ v | toml_escape }}", c), "3.14");
    }

    #[test]
    fn toml_unescape_passes_through_non_string() {
        let mut c = tera::Context::new();
        c.insert("v", &false);
        assert_eq!(render("{{ v | toml_unescape }}", c), "false");
    }
}
