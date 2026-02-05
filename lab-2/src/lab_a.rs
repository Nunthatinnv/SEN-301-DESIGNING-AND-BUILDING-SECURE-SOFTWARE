 use std::rc::Rc;

/// Lab A: Ownership vs Shared Ownership (Box vs Rc), using String.
///
/// What you learn:
/// 1) Box chain: exclusive ownership, easy mutation, but no sharing.
/// 2) Rc chain: shared ownership, node handle can outlive the creator scope.
/// 3) Rc counts: observe strong_count before and after scope ends.
///
/// Student tasks:
/// A1) Implement Box mutation: change node B's value from "B" to "BB".
/// A2) Implement Rc count demo: clone an Rc handle to node B and print refcounts.

pub fn run() {
    println!("\n== lab_a ==");

    demo_box_nodes_and_mutation();
    demo_rc_nodes_refcount_outlives_scope();
}

// ----------------------------
// Part 1: Box nodes + mutability
// ----------------------------

#[derive(Debug)]
struct BoxNode {
    value: String,
    next: Option<Box<BoxNode>>,
}

fn demo_box_nodes_and_mutation() {
    println!("\n-- Lab A1: Box nodes (exclusive ownership) + mutation --");

    // Build A -> B -> C
    let mut node_a = BoxNode {
        value: "A".to_string(),
        next: Some(Box::new(BoxNode {
            value: "B".to_string(),
            next: Some(Box::new(BoxNode {
                value: "C".to_string(),
                next: None,
            })),
        })),
    };

    println!("Before mutation: {}", format_box_chain(&node_a));

    // STUDENT TODO A1:
    // Change node B's value from "B" to "BB" by mutating through Box ownership.
    // Hint: use as_mut() to reach the next node.
    set_second_value_box(&mut node_a, "BB");

    println!("After mutation : {}", format_box_chain(&node_a));
    println!("Checkpoint: chain should contain A -> BB -> C");
}

fn format_box_chain(head: &BoxNode) -> String {
    let mut out = vec![head.value.clone()];
    let mut cur = head.next.as_deref();
    while let Some(n) = cur {
        out.push(n.value.clone());
        cur = n.next.as_deref();
    }
    out.join(" -> ")
}

/// STUDENT TODO A1:
/// Mutate the second node's value.
fn set_second_value_box(head: &mut BoxNode, new_value: &str) {
    todo!("Implement Box mutation: set second node's value");
}

// ----------------------------
// Part 2: Rc nodes + refcounts
// ----------------------------

#[derive(Debug)]
struct RcNode {
    value: String,
    next: Option<Rc<RcNode>>,
}

fn demo_rc_nodes_refcount_outlives_scope() {
    println!("\n-- Lab A2: Rc nodes (shared ownership) + strong_count --");

    let external: Rc<RcNode>;

    {
        // Build A -> B -> C
        let node_c = Rc::new(RcNode {
            value: "C".to_string(),
            next: None,
        });

        let node_b = Rc::new(RcNode {
            value: "B".to_string(),
            next: Some(node_c.clone()),
        });

        let _node_a = Rc::new(RcNode {
            value: "A".to_string(),
            next: Some(node_b.clone()),
        });

        // STUDENT TODO A2:
        // Print strong_count of node_b before and after cloning into external.
        // Then clone node_b into external.
        external = clone_b_and_print_counts(&node_b);

        // Optional: show counts for curiosity
        println!("(inside scope) external.value = {}", external.value);
    }

    println!("-- creator scope ended --");

    // This must still work: the node should outlive the scope because external owns it.
    println!("outside scope: external.value = {}", external.value);
    println!(
        "outside scope: external strong_count = {}",
        Rc::strong_count(&external)
    );

    println!("Checkpoint: external should still print \"B\" (unchanged), and strong_count >= 1.");
}

/// STUDENT TODO A2:
/// Clone node_b into an external handle and print refcounts.
///
/// Required prints:
/// - strong_count before cloning into external
/// - strong_count after cloning into external
///
/// Return:
/// - the cloned Rc handle
fn clone_b_and_print_counts(node_b: &Rc<RcNode>) -> Rc<RcNode> {
    todo!("Implement Rc clone + strong_count prints");
}
