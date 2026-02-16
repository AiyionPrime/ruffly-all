mod linter;
mod ruff_code;
mod ruff_reader;

use linter::Linter;
use ruff_reader::read_codes;

fn stderr_summary(postponed_problems: usize) {
    if postponed_problems == 0 {
        eprintln!("This project has no problems, given the above ruleset is in place.");
    } else {
        eprintln!(
            "This project has {} problems, which can be postponed by the above ruleset.",
            postponed_problems
        );
    }
    eprintln!("That's ruffly-all to migrate to ruff right now.")
}

fn main() {
    let (unique_groups, ignored_problems) = read_codes();

    let mut listed: Vec<Linter> = unique_groups.into_iter().collect();
    listed.sort();

    println!("[tool.ruff]");
    println!("lint.select = [ \"ALL\" ]");
    println!("lint.ignore = [");
    println!("  \"COM812\",  # incompatible with ruff format in 0.14");
    println!("  \"D203\",    # incompatible to D211");
    println!("  \"D212\",    # incompatible to D213");
    for linter in listed {
        let rule_toml = format!("\"{}\",", &linter);
        println!("  {:11}# {}", rule_toml, linter.generate_rule_url());
    }
    println!("]");
    stderr_summary(ignored_problems);
}
