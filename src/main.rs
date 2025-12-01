use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return;
    }

    let day = &args[1];
    let day_formatted = if day.len() == 1 {
        format!("0{}", day)
    } else {
        day.to_string()
    };
    let bin = format!("day{}", day_formatted);

    Command::new("cargo")
        .args(&["run", "--bin", &bin])
        .status()
        .expect("Command failed to start");
}
