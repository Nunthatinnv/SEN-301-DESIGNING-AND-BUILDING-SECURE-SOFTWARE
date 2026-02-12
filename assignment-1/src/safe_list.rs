// =======================
// Assignment Part 2: Safe Doubly Linked List
// =======================
//
// In this task, you will implement a **safe doubly linked list** using
// Rust's smart pointers.
//
// This is the safe counterpart to `unsafe_list.rs`. The public API is the same,
// but the implementation must rely on Rust's ownership and borrowing rules
// instead of raw pointers and manual memory management.
//
// ---------------------------------
// Core constraints (READ CAREFULLY)
// ---------------------------------
//
// 1) NO `unsafe` code is allowed in this file.
// 2) You MUST use smart pointers:
//      - `Rc<RefCell<Node>>` for forward (`next`) links
//      - `Weak<RefCell<Node>>` for backward (`prev`) links
// 3) You MUST NOT use raw pointers or `union` here.
// 4) The list stores `Value` (an enum provided in value.rs).
// 5) Your implementation must not panic under correct usage.
//
// ---------------------------------
// TODOs (What you must implement)
// ---------------------------------
//
// You must implement ALL of the following:
//
// [value.rs]
//   - Value::as_int
//   - Value::as_float
//
// [Node]
//   - Node::new
//   - Node::next
//   - Node::prev
//
// [DoublyLinkedList]
//   - push_front
//   - push_back
//   - pop_front
//   - pop_back
//
// ---------------------------------
// What this assignment is testing
// ---------------------------------
//
// - Shared ownership with `Rc`
// - Interior mutability with `RefCell`
// - Cycle avoidance with `Weak`
// - Correct borrow scoping (no nested mutable borrows)
// - Encoding invariants using types instead of comments
//

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::value::Value;

/// A link to another node in the list.
///
/// We use `Option` because:
/// - the head has no previous node
/// - the tail has no next node
type Link = Option<Rc<RefCell<Node>>>;

/// A node in the doubly linked list.
///
/// IMPORTANT DESIGN DECISIONS:
/// - `next` uses `Rc` because the next node is owned by the list
/// - `prev` uses `Weak` to avoid reference cycles
///
/// If `prev` were `Rc`, the list would leak memory.
struct Node {
    value: Value,
    prev: Option<Weak<RefCell<Node>>>,
    next: Link,
}

impl Node {
    /// Create a new node containing `value`.
    ///
    /// The new node:
    ///   - has no previous node
    ///   - has no next node
    ///
    /// TODO:
    ///   - Allocate a `Node`
    ///   - Wrap it in `Rc<RefCell<_>>`
    ///   - Initialize `prev = None`, `next = None`
    fn new(value: Value) -> Rc<RefCell<Self>> {
        todo!("Create Rc<RefCell<Node>> with prev=None and next=None");
    }

    /// Return the next node in the list.
    ///
    /// IMPORTANT:
    ///   - This returns an owned `Rc`, not a reference.
    ///   - You must CLONE the `Rc` stored in `self.next`.
    ///
    /// TODO:
    ///   - Return `self.next.clone()`
    fn next(&self) -> Link {
        todo!("Return self.next cloned");
    }

    /// Return the previous node in the list.
    ///
    /// IMPORTANT:
    ///   - `prev` is stored as `Weak`
    ///   - You must call `upgrade()` to get `Option<Rc<_>>`
    ///
    /// TODO:
    ///   - If `self.prev` is `Some(w)`, return `w.upgrade()`
    ///   - Otherwise, return `None`
    fn prev(&self) -> Link {
        todo!("Upgrade self.prev (Weak) into Rc and return it");
    }
}

/// A safe doubly linked list.
///
/// INVARIANTS (must always hold):
///   - If `len == 0`: `head == None` and `tail == None`
///   - If `len > 0`: `head` and `tail` are `Some`
///   - `head.prev == None`
///   - `tail.next == None`
///   - Forward traversal from head reaches exactly `len` nodes
///   - Backward traversal from tail reaches exactly `len` nodes
pub struct DoublyLinkedList {
    head: Link,
    tail: Link,
    len: usize,
}

impl DoublyLinkedList {
    /// Create an empty list.
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    /// Return the number of elements in the list.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Return true if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Insert a value at the front of the list.
    ///
    /// TODO:
    ///   Handle the following cases:
    ///
    ///   1) Empty list:
    ///      - head = tail = new_node
    ///
    ///   2) Non-empty list:
    ///      - new_node.next = old_head
    ///      - old_head.prev = Weak(new_node)
    ///      - head = new_node
    ///
    /// HINT:
    ///   - Keep `borrow_mut()` scopes SHORT.
    ///   - Never hold two mutable borrows at once.
    pub fn push_front(&mut self, value: Value) {
        todo!("Implement push_front");
    }

    /// Insert a value at the back of the list.
    ///
    /// TODO:
    ///   Handle the following cases:
    ///
    ///   1) Empty list:
    ///      - head = tail = new_node
    ///
    ///   2) Non-empty list:
    ///      - old_tail.next = new_node
    ///      - new_node.prev = Weak(old_tail)
    ///      - tail = new_node
    pub fn push_back(&mut self, value: Value) {
        todo!("Implement push_back");
    }

    /// Remove and return the value at the front of the list.
    ///
    /// TODO:
    ///   Handle the following cases:
    ///
    ///   1) Empty list:
    ///      - return None
    ///
    ///   2) Single element:
    ///      - clear head and tail
    ///      - len becomes 0
    ///
    ///   3) Multiple elements:
    ///      - new_head = old_head.next
    ///      - new_head.prev = None
    ///
    /// NOTE:
    ///   - Detaching the removed node's links is good hygiene.
    pub fn pop_front(&mut self) -> Option<Value> {
        todo!("Implement pop_front");
    }

    /// Remove and return the value at the back of the list.
    ///
    /// TODO:
    ///   Handle the following cases:
    ///
    ///   1) Empty list:
    ///      - return None
    ///
    ///   2) Single element:
    ///      - clear head and tail
    ///      - len becomes 0
    ///
    ///   3) Multiple elements:
    ///      - new_tail = old_tail.prev (upgrade Weak)
    ///      - new_tail.next = None
    pub fn pop_back(&mut self) -> Option<Value> {
        todo!("Implement pop_back");
    }

    /// Iterate from head to tail.
    ///
    /// This iterator:
    /// - yields `Value` by copy
    /// - uses `Node::next()` internally
    /// - stops after `len` elements (cycle guard)
    pub fn iter_forward(&self) -> IterForward {
        IterForward {
            cur: self.head.clone(),
            remaining: self.len,
        }
    }

    /// Iterate from tail to head.
    ///
    /// This iterator:
    /// - yields `Value` by copy
    /// - uses `Node::prev()` internally
    /// - stops after `len` elements (cycle guard)
    pub fn iter_backward(&self) -> IterBackward {
        IterBackward {
            cur: self.tail.clone(),
            remaining: self.len,
        }
    }

    /// Check all list invariants.
    ///
    /// This function is extremely helpful for debugging and testing.
    /// You may call it inside tests to ensure your list is always valid.
    pub fn check_invariants(&self) -> bool {
        // Basic head/tail shape checks
        if self.len == 0 {
            return self.head.is_none() && self.tail.is_none();
        }
        if self.head.is_none() || self.tail.is_none() {
            return false;
        }

        // head.prev must be None
        if let Some(h) = &self.head {
            if h.borrow().prev.is_some() {
                return false;
            }
        }

        // tail.next must be None
        if let Some(t) = &self.tail {
            if t.borrow().next.is_some() {
                return false;
            }
        }

        // Traverse forward, count, and verify back-links
        let mut count = 0usize;
        let mut cur = self.head.clone();
        let mut prev: Link = None;

        while let Some(node_rc) = cur {
            count += 1;
            if count > self.len + 1 {
                // cycle guard
                return false;
            }

            let node = node_rc.borrow();

            // verify node.prev matches prev
            match (&prev, node.prev.as_ref()) {
                (None, None) => {}
                (Some(prev_rc), Some(w)) => {
                    if let Some(up) = w.upgrade() {
                        if !Rc::ptr_eq(&up, prev_rc) {
                            return false;
                        }
                    } else {
                        // weak couldn't upgrade => broken back pointer
                        return false;
                    }
                }
                _ => return false,
            }

            prev = Some(node_rc.clone());
            cur = node.next();
        }

        if count != self.len {
            return false;
        }

        // Ensure last visited is tail
        if let (Some(last), Some(tail)) = (prev, &self.tail) {
            if !Rc::ptr_eq(&last, tail) {
                return false;
            }
        } else {
            return false;
        }

        true
    }
}

/// Forward iterator: head → tail
pub struct IterForward {
    cur: Link,
    remaining: usize,
}

impl Iterator for IterForward {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let node_rc = self.cur.clone()?;
        let node = node_rc.borrow();

        let value = node.value;
        self.cur = node.next();

        self.remaining -= 1;
        Some(value)
    }
}

/// Backward iterator: tail → head
pub struct IterBackward {
    cur: Link,
    remaining: usize,
}

impl Iterator for IterBackward {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let node_rc = self.cur.clone()?;
        let node = node_rc.borrow();

        let value = node.value;
        self.cur = node.prev();

        self.remaining -= 1;
        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v_int(x: i64) -> Value {
        Value::int(x)
    }
    fn v_float(x: f64) -> Value {
        Value::float(x)
    }

    #[test]
    fn empty_list_iterators_and_pops() {
        let mut list = DoublyLinkedList::new();

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());

        assert_eq!(list.iter_forward().collect::<Vec<_>>(), vec![]);
        assert_eq!(list.iter_backward().collect::<Vec<_>>(), vec![]);

        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);

        // If you implement check_invariants, this should hold.
        assert!(list.check_invariants());
    }

    #[test]
    fn single_push_front_then_iterate_and_pop() {
        let mut list = DoublyLinkedList::new();
        list.push_front(v_int(7));

        assert_eq!(list.len(), 1);
        assert!(!list.is_empty());

        assert_eq!(list.iter_forward().collect::<Vec<_>>(), vec![v_int(7)]);
        assert_eq!(list.iter_backward().collect::<Vec<_>>(), vec![v_int(7)]);
        assert!(list.check_invariants());

        assert_eq!(list.pop_front(), Some(v_int(7)));
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
        assert_eq!(list.iter_forward().collect::<Vec<_>>(), vec![]);
        assert!(list.check_invariants());
    }

    #[test]
    fn single_push_back_then_iterate_and_pop() {
        let mut list = DoublyLinkedList::new();
        list.push_back(v_float(3.5));

        assert_eq!(list.len(), 1);
        assert_eq!(list.iter_forward().collect::<Vec<_>>(), vec![v_float(3.5)]);
        assert_eq!(list.iter_backward().collect::<Vec<_>>(), vec![v_float(3.5)]);
        assert!(list.check_invariants());

        assert_eq!(list.pop_back(), Some(v_float(3.5)));
        assert!(list.is_empty());
        assert!(list.check_invariants());
    }

    #[test]
    fn push_front_order_and_iterators() {
        // push_front is LIFO in forward iteration
        let mut list = DoublyLinkedList::new();
        list.push_front(v_int(1));
        list.push_front(v_int(2));
        list.push_front(v_int(3)); // [3,2,1]

        assert_eq!(
            list.iter_forward().collect::<Vec<_>>(),
            vec![v_int(3), v_int(2), v_int(1)]
        );
        assert_eq!(
            list.iter_backward().collect::<Vec<_>>(),
            vec![v_int(1), v_int(2), v_int(3)]
        );
        assert!(list.check_invariants());
    }

    #[test]
    fn push_back_order_and_iterators() {
        // push_back is FIFO in forward iteration
        let mut list = DoublyLinkedList::new();
        list.push_back(v_int(1));
        list.push_back(v_int(2));
        list.push_back(v_int(3)); // [1,2,3]

        assert_eq!(
            list.iter_forward().collect::<Vec<_>>(),
            vec![v_int(1), v_int(2), v_int(3)]
        );
        assert_eq!(
            list.iter_backward().collect::<Vec<_>>(),
            vec![v_int(3), v_int(2), v_int(1)]
        );
        assert!(list.check_invariants());
    }

    #[test]
    fn mixed_pushes_then_iterators() {
        // Build: [4.0, 2, 1, 3.0]
        let mut list = DoublyLinkedList::new();
        list.push_back(v_int(1)); // [1]
        list.push_front(v_int(2)); // [2,1]
        list.push_back(v_float(3.0)); // [2,1,3.0]
        list.push_front(v_float(4.0)); // [4.0,2,1,3.0]

        assert_eq!(
            list.iter_forward().collect::<Vec<_>>(),
            vec![v_float(4.0), v_int(2), v_int(1), v_float(3.0)]
        );
        assert_eq!(
            list.iter_backward().collect::<Vec<_>>(),
            vec![v_float(3.0), v_int(1), v_int(2), v_float(4.0)]
        );
        assert!(list.check_invariants());
    }

    #[test]
    fn pop_front_updates_structure_and_iteration() {
        let mut list = DoublyLinkedList::new();
        list.push_back(v_int(1));
        list.push_back(v_int(2));
        list.push_back(v_int(3)); // [1,2,3]

        assert_eq!(list.pop_front(), Some(v_int(1))); // [2,3]
        assert_eq!(list.len(), 2);

        assert_eq!(
            list.iter_forward().collect::<Vec<_>>(),
            vec![v_int(2), v_int(3)]
        );
        assert_eq!(
            list.iter_backward().collect::<Vec<_>>(),
            vec![v_int(3), v_int(2)]
        );
        assert!(list.check_invariants());
    }

    #[test]
    fn pop_back_updates_structure_and_iteration() {
        let mut list = DoublyLinkedList::new();
        list.push_front(v_int(1));
        list.push_front(v_int(2));
        list.push_front(v_int(3)); // [3,2,1]

        assert_eq!(list.pop_back(), Some(v_int(1))); // [3,2]
        assert_eq!(list.len(), 2);

        assert_eq!(
            list.iter_forward().collect::<Vec<_>>(),
            vec![v_int(3), v_int(2)]
        );
        assert_eq!(
            list.iter_backward().collect::<Vec<_>>(),
            vec![v_int(2), v_int(3)]
        );
        assert!(list.check_invariants());
    }

    #[test]
    fn alternating_ops_does_not_break_invariants() {
        let mut list = DoublyLinkedList::new();

        // Alternate front/back push and pop to stress edge cases.
        for i in 0..100 {
            if i % 2 == 0 {
                list.push_front(v_int(i));
            } else {
                list.push_back(v_int(i));
            }
            assert!(list.check_invariants());

            // Pop from opposite side to force rewiring.
            let popped = if i % 2 == 0 {
                list.pop_back()
            } else {
                list.pop_front()
            };
            assert!(popped.is_some());
            assert!(list.check_invariants());
        }

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
        assert_eq!(list.iter_forward().collect::<Vec<_>>(), vec![]);
        assert_eq!(list.iter_backward().collect::<Vec<_>>(), vec![]);
        assert!(list.check_invariants());
    }

    #[test]
    fn iterators_stop_after_len_even_if_cycle_bug_exists() {
        // This test is defensive: your iterators include a `remaining` guard.
        // If students accidentally create a cycle, iteration should still not hang forever.
        //
        // We don't *create* a cycle here (no unsafe access), but we assert the guard behavior
        // indirectly by checking iterator lengths match len.
        let mut list = DoublyLinkedList::new();
        for i in 0..20 {
            list.push_back(v_int(i));
        }

        let fwd = list.iter_forward().collect::<Vec<_>>();
        let bwd = list.iter_backward().collect::<Vec<_>>();

        assert_eq!(fwd.len(), list.len());
        assert_eq!(bwd.len(), list.len());
        assert!(list.check_invariants());
    }
}
