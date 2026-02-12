// Minimal demo to try both lists.
//
// Run:
//   cargo run --bin demo -- safe
//   cargo run --bin demo -- unsafe
//
// Notes:
// - unsafe demo requires alloc/dealloc/Drop implemented in unsafe_list.rs
// - safe demo requires safe_list.rs + value.rs TODOs implemented

fn main() {
    let which = std::env::args().nth(1).unwrap_or_else(|| "safe".to_string());
    println!("{:?}", which);
    match which.as_str() {
        "safe" => demo_safe(),
        "unsafe" => demo_unsafe(),
        _ => {
            eprintln!("Usage: cargo run --bin demo -- <safe|unsafe>");
            eprintln!("Defaulting to: safe");
            demo_safe();
        }
    }
}

// -----------------------------
// Helpers
// -----------------------------

fn fmt_value(v: &assignment_1::value::Value) -> String {
    if let Some(x) = v.as_int() {
        format!("Int({})", x)
    } else if let Some(x) = v.as_float() {
        format!("Float({})", x)
    } else {
        // Should be impossible if Value is implemented correctly.
        "<?>".to_string()
    }
}

fn print_safe_list(label: &str, list: &assignment_1::safe_list::DoublyLinkedList) {
    let fwd = list.iter_forward().collect::<Vec<_>>();
    let bwd = list.iter_backward().collect::<Vec<_>>();

    println!("{label}");
    println!("  len={} empty? {}", list.len(), list.is_empty());
    println!(
        "  forward : [{}]",
        fwd.iter().map(fmt_value).collect::<Vec<_>>().join(", ")
    );
    println!(
        "  backward: [{}]",
        bwd.iter().map(fmt_value).collect::<Vec<_>>().join(", ")
    );
    println!("  invariants ok? {}", list.check_invariants());
}

fn print_unsafe_list(label: &str, list: &assignment_1::unsafe_list::DoublyLinkedList) {
    let fwd = list.iter_forward().collect::<Vec<_>>();
    let bwd = list.iter_backward().collect::<Vec<_>>();

    println!("{label}");
    println!("  len={} empty? {}", list.len(), list.is_empty());
    println!("  forward : {:?}", fwd);
    println!("  backward: {:?}", bwd);
    println!("  invariants ok? {}", list.check_invariants());
}

// -----------------------------
// SAFE demo
// -----------------------------
fn demo_safe() {
    use assignment_1::safe_list::DoublyLinkedList;
    use assignment_1::value::Value;

    println!("== Demo: SAFE doubly linked list ==");

    let mut list = DoublyLinkedList::new();
    print_safe_list("start", &list);

    list.push_back(Value::int(10));
    list.push_back(Value::float(2.5));
    list.push_front(Value::int(3));
    print_safe_list("after pushes", &list);

    let p1 = list.pop_front();
    let p2 = list.pop_back();

    println!(
        "pop_front = {}",
        p1.as_ref().map(fmt_value).unwrap_or_else(|| "None".to_string())
    );
    println!(
        "pop_back  = {}",
        p2.as_ref().map(fmt_value).unwrap_or_else(|| "None".to_string())
    );

    print_safe_list("after pops", &list);
}

// -----------------------------
// UNSAFE demo
// -----------------------------
fn demo_unsafe() {
    use assignment_1::unsafe_list::DoublyLinkedList;

    println!("== Demo: UNSAFE doubly linked list ==");

    let mut list = DoublyLinkedList::new();
    print_unsafe_list("start", &list);

    list.push_back(10);
    list.push_back(25);
    list.push_front(3);
    print_unsafe_list("after pushes", &list);

    println!("pop_front = {:?}", list.pop_front());
    println!("pop_back  = {:?}", list.pop_back());

    print_unsafe_list("after pops", &list);
}
