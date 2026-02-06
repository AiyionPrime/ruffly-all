mod ruff_code;

use ruff_code::RuffCode;
use std::collections::HashSet;
use std::process::Command;
use std::str::FromStr;

fn main() {
    let output = Command::new("ruff")
        .arg("check")
        .arg("--select")
        .arg("ALL")
        .arg("--output-format")
        .arg("concise")
        .output()
        .expect("failed to execute process");
    let out = String::from_utf8_lossy(&output.stdout);

    let mut unique_groups = HashSet::new();

    for line in out.lines() {
        let maybe_code = &line
            .split_whitespace()
            .nth(1)
            .expect("line without second word encountered");
        if maybe_code
            .chars()
            .next()
            .expect("no char")
            .is_ascii_alphabetic()
            && *maybe_code != "invalid-syntax:"
            && *maybe_code != "checks"
            && *maybe_code != "fixes"
        {
            let error_message = format!("Could not parse ruff code: '{}'", &maybe_code);
            let code = RuffCode::from_str(maybe_code).expect(&error_message);
            unique_groups.insert(code.group);
        }
    }

    let mut listed: Vec<String> = unique_groups.into_iter().collect();
    listed.sort();

    println!("[tool.ruff]");
    println!("lint.select = [ \"ALL\" ]");
    println!("lint.ignore = [");
    println!("  \"D203\",    # incompatible to D211");
    println!("  \"D212\",    # incompatible to D213");
    for group in listed {
        println!("  \"{}\",", &group);
    }
    println!("]");
}
