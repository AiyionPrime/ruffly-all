mod ruff_code;
mod ruff_reader;

use ruff_reader::read_codes;

fn main() {
    let unique_groups = read_codes();

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
