use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 { return; }
    let bin = format!("day{}", &args[1]);
    Command::new("cargo")
        .args(&["run", "--bin", &bin])
        .status()
        .expect("Command failed to start");
}
