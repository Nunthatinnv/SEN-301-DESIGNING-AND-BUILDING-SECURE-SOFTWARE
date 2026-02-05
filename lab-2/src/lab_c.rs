use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// Lab C: Minimal safe doubly linked list wiring.
///
/// What you learn:
/// 1) DLL node shape with next (Rc) and prev (Weak).
/// 2) How to wire two nodes as neighbors without creating strong cycles.
/// 3) Forward and backward traversal using next()/prev().
///
/// No full list type yet. Just nodes and wiring.
///
/// Student tasks:
/// C1) Implement Node::next() and Node::prev().
/// C2) Implement link(a, b): make a <-> b.
/// C3) Demonstrate traversal forward and backward.

pub fn run() {
    println!("\n== lab_c ==");

    demo_two_node_dll();
}

type NodeRef = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    value: String,
    next: Option<NodeRef>,
    prev: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new(value: &str) -> NodeRef {
        Rc::new(RefCell::new(Node {
            value: value.to_string(),
            next: None,
            prev: None,
        }))
    }

    /// STUDENT TODO C1:
    /// Return next node as Option<Rc<RefCell<Node>>> (clone the Rc).
    fn next(&self) -> Option<NodeRef> {
        todo!("Implement next(): clone Rc handle from self.next");
    }

    /// STUDENT TODO C1:
    /// Return prev node as Option<Rc<RefCell<Node>>> by upgrading Weak.
    fn prev(&self) -> Option<NodeRef> {
        todo!("Implement prev(): upgrade Weak from self.prev");
    }
}

/// STUDENT TODO C2:
/// Wire a and b as neighbors: a <-> b.
///
/// Required:
/// - a.next = Some(b)
/// - b.prev = Some(Weak(a))
///
/// Constraints:
/// - Keep borrow_mut scopes short.
/// - Do not hold two mutable borrows at the same time.
fn link(a: &NodeRef, b: &NodeRef) {
    todo!("Implement link(a,b) with Rc and Weak");
}

fn demo_two_node_dll() {
    println!("\n-- Lab C: two-node DLL wiring --");

    let a = Node::new("A");
    let b = Node::new("B");

    link(&a, &b);

    // Forward: A -> B
    let a_next = a.borrow().next();
    println!("a.value = {}", a.borrow().value);
    println!(
        "a.next.value = {}",
        a_next.as_ref().unwrap().borrow().value
    );

    // Backward: B -> A
    let b_prev = b.borrow().prev();
    println!("b.value = {}", b.borrow().value);
    println!(
        "b.prev.value = {}",
        b_prev.as_ref().unwrap().borrow().value
    );

    println!("Checkpoint: forward should be A -> B and backward should be B -> A.");
}
