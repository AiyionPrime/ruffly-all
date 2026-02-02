use std::process::Command;

fn main() {
    let output = Command::new("ruff")
        .current_dir("../pylint/")
        .arg("check")
        .arg("--isolated")
        .arg("--select")
        .arg("ALL")
        .arg("--output-format")
        .arg("concise")
        .output()
        .expect("failed to execute process");
    println!("{:?}", &output);
}
