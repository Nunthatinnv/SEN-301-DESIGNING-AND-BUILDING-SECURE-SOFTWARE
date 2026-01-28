mod mem_layout;
mod lifetime;
mod bounds;
mod aliasing;
mod type_confusion;

fn usage() {
    eprintln!("Usage: cargo run -- <lifetime|bounds|aliasing|type_confusion|all>");
}

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| "all".to_string());

    match arg.as_str() {
        "mem_layout" => mem_layout::run(),
        "lifetime" => lifetime::run(),
        "bounds" => bounds::run(),
        "aliasing" => aliasing::run(),
        "type_confusion" => type_confusion::run(),
        "all" => {
            lifetime::run();
            bounds::run();
            aliasing::run();
            type_confusion::run();
        }
        _ => usage(),
    }
}
