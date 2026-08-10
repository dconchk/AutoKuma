#![allow(dead_code)]

use serde::Deserialize;
use serde_json::json;
use std::{
    fs,
    path::{Path, PathBuf},
};

include!("mod.rs");

const SCHEMA: &str = "autokuma.circuit-ownership-adoption/v1";
const OWNER_TAG_NAME: &str = "dcon-circuit-owner";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OwnerTagPlan {
    name: String,
    expected_id: i32,
    adopted_id: i32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MonitorPlan {
    identity: String,
    expected_id: Option<i32>,
    adopted_id: i32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AdoptionPlan {
    schema: String,
    owner_tag: OwnerTagPlan,
    monitors: Vec<MonitorPlan>,
}

enum Command {
    Inventory {
        data_path: PathBuf,
    },
    Adopt {
        data_path: PathBuf,
        plan_path: PathBuf,
    },
}

fn require_real_path(value: &str, directory: bool) -> Result<PathBuf, String> {
    let path = Path::new(value);
    if !path.is_absolute() {
        return Err("path must be absolute".to_owned());
    }
    let metadata = fs::symlink_metadata(path).map_err(|_| "path is unavailable".to_owned())?;
    if metadata.file_type().is_symlink()
        || (directory && !metadata.is_dir())
        || (!directory && !metadata.is_file())
    {
        return Err("path must be a real file or directory".to_owned());
    }
    let canonical = fs::canonicalize(path).map_err(|_| "path cannot be resolved".to_owned())?;
    if canonical != path {
        return Err("path must be normalized".to_owned());
    }
    Ok(canonical)
}

fn parse_arguments(arguments: &[String]) -> Result<Command, String> {
    if arguments.len() == 3 && arguments[0] == "inventory" && arguments[1] == "--data-path" {
        return Ok(Command::Inventory {
            data_path: require_real_path(&arguments[2], true)?,
        });
    }
    if arguments.len() == 5
        && arguments[0] == "adopt"
        && arguments[1] == "--data-path"
        && arguments[3] == "--plan"
    {
        return Ok(Command::Adopt {
            data_path: require_real_path(&arguments[2], true)?,
            plan_path: require_real_path(&arguments[4], false)?,
        });
    }
    Err("usage: autokuma-state <inventory|adopt> --data-path ABSOLUTE [--plan ABSOLUTE]".to_owned())
}

fn read_plan(plan_path: &Path) -> Result<AdoptionPlan, String> {
    let plan: AdoptionPlan = serde_json::from_str(
        &fs::read_to_string(plan_path).map_err(|_| "adoption plan cannot be read".to_owned())?,
    )
    .map_err(|_| "adoption plan is invalid".to_owned())?;
    let identity = regex::Regex::new(r"^circuit_v1_[^_]+_[^_]+_(primary|backup)$")
        .map_err(|_| "identity contract is invalid".to_owned())?;
    if plan.schema != SCHEMA
        || plan.owner_tag.name != OWNER_TAG_NAME
        || plan.monitors.is_empty()
        || plan.monitors.iter().any(|monitor| {
            !identity.is_match(&monitor.identity)
                || monitor.expected_id.is_some_and(|id| id <= 0)
                || monitor.adopted_id <= 0
        })
    {
        return Err("adoption plan does not match the circuit contract".to_owned());
    }
    Ok(plan)
}

fn run(arguments: &[String]) -> Result<serde_json::Value, String> {
    let command = parse_arguments(arguments)?;
    let data_path = match &command {
        Command::Inventory { data_path } | Command::Adopt { data_path, .. } => data_path,
    };
    let db = app_state::AppDB::new(
        data_path
            .to_str()
            .ok_or_else(|| "data path is invalid".to_owned())?,
    )
    .map_err(|_| "AutoKuma ownership database could not be opened".to_owned())?;
    if let Command::Inventory { .. } = command {
        let mut monitors = db
            .get_monitors()
            .map_err(|_| "AutoKuma monitor inventory failed".to_owned())?;
        let mut tags = db
            .get_tags()
            .map_err(|_| "AutoKuma tag inventory failed".to_owned())?;
        monitors.sort();
        tags.sort();
        return Ok(json!({
            "schema": "autokuma.ownership-state-inventory/v1",
            "monitors": monitors.into_iter().map(|(identity, id)| json!({
                "identity": identity,
                "id": id,
            })).collect::<Vec<_>>(),
            "tags": tags.into_iter().map(|(name, id)| json!({
                "name": name,
                "id": id,
            })).collect::<Vec<_>>(),
        }));
    }
    let Command::Adopt { plan_path, .. } = command else {
        unreachable!();
    };
    let plan = read_plan(&plan_path)?;
    let mappings = plan
        .monitors
        .iter()
        .map(|monitor| {
            (
                monitor.identity.as_str(),
                monitor.expected_id,
                monitor.adopted_id,
            )
        })
        .collect::<Vec<_>>();
    db.adopt_monitor_ids(
        &plan.owner_tag.name,
        plan.owner_tag.expected_id,
        plan.owner_tag.adopted_id,
        &mappings,
    )
    .map_err(|_| "AutoKuma ownership adoption failed safely".to_owned())?;
    for monitor in &plan.monitors {
        let observed = db
            .get_id::<i32>(name::Name::Monitor(monitor.identity.clone()))
            .map_err(|_| "AutoKuma ownership readback failed".to_owned())?;
        if observed != Some(monitor.adopted_id) {
            return Err("AutoKuma ownership readback did not converge".to_owned());
        }
    }
    if db
        .get_id::<i32>(name::Name::Tag(plan.owner_tag.name.clone()))
        .map_err(|_| "AutoKuma owner-tag readback failed".to_owned())?
        != Some(plan.owner_tag.adopted_id)
    {
        return Err("AutoKuma owner-tag readback did not converge".to_owned());
    }
    Ok(json!({
        "schema": "autokuma.circuit-ownership-adoption-result/v1",
        "adopted_monitor_count": plan.monitors.len(),
        "owner_tag_adopted": true,
        "verified": true,
    }))
}

fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match run(&arguments) {
        Ok(result) => println!("{}", result),
        Err(message) => {
            eprintln!("autokuma-state: {}", message);
            std::process::exit(1);
        }
    }
}
