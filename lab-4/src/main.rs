mod int;
mod float;

use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let mode = args.next().unwrap_or_else(|| "help".to_string());

    match mode.as_str() {
        "int" => int::run(),
        "float" => float::run(),
        "help" | "-h" | "--help" => print_help(),
        other => {
            eprintln!("Unknown mode: {other}");
            print_help();
            std::process::exit(2);
        }
    }
}

fn print_help() {
    println!(
        r#"Numeric Safety Labs

Usage:
  cargo run -- int     # Lab A: Integer safety
  cargo run -- float   # Lab B: Floating-point safety
"#
    );
}
