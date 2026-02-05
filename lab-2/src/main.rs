pub mod lab_a;
pub mod lab_b;
pub mod lab_c;

fn usage() {
    eprintln!("Usage:");
    eprintln!("  cargo run -- lab_a");
    eprintln!("  cargo run -- lab_b");
    eprintln!("  cargo run -- lab_c");
}

fn main() {
    let which = std::env::args().nth(1).unwrap_or_else(|| "all".to_string());

    match which.as_str() {
        "lab_a" => lab_a::run(),
        "lab_b" => lab_b::run(),
        "lab_c" => lab_c::run(),
        _ => usage(),
    }
}
