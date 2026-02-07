use crate::ruff_code::RuffCode;
use serde::Deserialize;
use std::collections::HashSet;
use std::process::Command;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct RuffSparseJson {
    code: String,
}

pub fn read_codes() -> HashSet<String> {
    let output = Command::new("ruff")
        .arg("check")
        .arg("--select")
        .arg("ALL")
        .arg("--output-format")
        .arg("json")
        .output()
        .expect("failed to execute process");
    let out = String::from_utf8_lossy(&output.stdout);

    let items: Vec<RuffSparseJson> =
        serde_json::from_str(&out).expect("Could not read json from ruff.");

    let mut unique_groups = HashSet::new();

    for item in items {
        let error_message = format!("Could not parse ruff code: '{}'", &item.code);
        let code = RuffCode::from_str(&item.code).expect(&error_message);
        unique_groups.insert(code.group);
    }

    unique_groups
}
