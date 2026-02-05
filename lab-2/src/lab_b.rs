use std::cell::RefCell;
use std::rc::Rc;

/// Lab B: Mutability with Rc<RefCell<Node>> (interior mutability).
///
/// What you learn:
/// 1) How to mutate shared data safely with borrow_mut().
/// 2) How a node can be mutated both inside and outside the creator scope.
/// 3) What happens when you attempt two mutable borrows at once (panic).
///
/// Student tasks:
/// B1) Implement mutation helper: set node value.
/// B2) Implement "double borrow" demo using catch_unwind.

pub fn run() {
    println!("\n== lab_b ==");

    demo_mutate_in_scope_and_out_of_scope();
    demo_double_mut_borrow_panics();
}

type NodeRef = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    value: String,
    next: Option<NodeRef>,
}

fn build_chain() -> (NodeRef, NodeRef, NodeRef) {
    // Build A -> B -> C
    let c = Rc::new(RefCell::new(Node {
        value: "C".to_string(),
        next: None,
    }));

    let b = Rc::new(RefCell::new(Node {
        value: "B".to_string(),
        next: Some(c.clone()),
    }));

    let a = Rc::new(RefCell::new(Node {
        value: "A".to_string(),
        next: Some(b.clone()),
    }));

    (a, b, c)
}

fn demo_mutate_in_scope_and_out_of_scope() {
    println!("\n-- Lab B1: mutate shared node in and out of scope --");

    let external_b: NodeRef;

    {
        let (_a, b, _c) = build_chain();

        // Mutate B while inside scope
        set_value(&b, "BB");
        println!("inside scope: b.value = {}", b.borrow().value);

        // Keep B alive outside scope
        external_b = b.clone();
    }

    println!("-- creator scope ended --");

    // Mutate B outside scope via external handle
    set_value(&external_b, "BBB");
    println!("outside scope: b.value = {}", external_b.borrow().value);

    println!("Checkpoint: value should now be \"BBB\".");
}

/// STUDENT TODO B1:
/// Mutate the node's value using RefCell.
///
/// Requirements:
/// - Must use borrow_mut()
/// - Must not keep the mutable borrow alive longer than necessary
fn set_value(node: &NodeRef, new_value: &str) {
    todo!("Implement set_value using borrow_mut()");
}

fn demo_double_mut_borrow_panics() {
    println!("\n-- Lab B2: double mutable borrow causes runtime failure --");

    let (_a, b, _c) = build_chain();

    // Mutate once to show it works
    set_value(&b, "BB");
    println!("before double borrow: b.value = {}", b.borrow().value);

    // STUDENT TODO B2:
    // Cause a double mutable borrow panic, but catch it so the lab keeps running.
    //
    // Required behavior:
    // - Attempt to borrow_mut twice without releasing the first borrow.
    // - Use std::panic::catch_unwind to catch the panic.
    // - Print whether a panic occurred.
    double_borrow_demo(&b);

    println!("Checkpoint: you should see that double borrow panicked and was caught.");
}

/// STUDENT TODO B2:
/// Demonstrate that two simultaneous borrow_mut calls cause a panic, and catch it.
///
/// Hint:
/// - Use std::panic::catch_unwind(|| { ... })
/// - You may need std::panic::AssertUnwindSafe because RefCell is not UnwindSafe by default.
fn double_borrow_demo(node: &NodeRef) {
    todo!("Implement double borrow demo with catch_unwind");
}
