// =======================
// Assignment Part 1: Unsafe Doubly Linked List
// =======================
//
// Purpose:
// - This file implements a doubly linked list using raw pointers.
// - List logic (rewiring prev/next, head/tail) is fully implemented.
// - Your job is to correctly implement:
//     1) allocation
//     2) deallocation
//     3) cleanup in Drop
//
// This file intentionally uses unsafe Rust to expose the risks of
// manual memory management.
//
// ---------------------------------
// What you MUST implement
// ---------------------------------
//
// TODOs:
//   - alloc_node
//   - dealloc_node
//   - Drop for DoublyLinkedList
//
// Constraints:
// - You must use std::alloc::{alloc, dealloc}
// - You must use Layout::new::<Node>()
// - Each node must be freed EXACTLY ONCE
// - No memory leaks
// - No double frees
//

use std::alloc::{alloc, dealloc, Layout};
use std::ptr::null_mut;

/// The value stored in the list.
///
/// In the safe version, this becomes an enum.
/// Here, we keep it as i64 to focus on memory safety.
pub type Value = i64;

#[repr(C)]
struct Node {
    value: Value,
    prev: *mut Node,
    next: *mut Node,
}

pub struct DoublyLinkedList {
    head: *mut Node,
    tail: *mut Node,
    len: usize,
}

impl DoublyLinkedList {
    pub fn new() -> Self {
        Self {
            head: null_mut(),
            tail: null_mut(),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // --------------------------------------------------
    // TODO #1: Allocate and initialize a Node
    // --------------------------------------------------
    //
    // Requirements:
    // - Allocate memory using alloc(Layout::new::<Node>())
    // - Panic or abort if allocation fails
    // - Initialize:
    //     value
    //     prev = null
    //     next = null
    // - Return a *mut Node
    //
    unsafe fn alloc_node(value: Value) -> *mut Node {
        todo!("Allocate Node using alloc and initialize its fields");
    }

    // --------------------------------------------------
    // TODO #2: Deallocate a Node
    // --------------------------------------------------
    //
    // Requirements:
    // - Free memory allocated by alloc_node
    // - Use dealloc with the SAME Layout
    // - Node must not be freed more than once
    //
    unsafe fn dealloc_node(node: *mut Node) {
        todo!("Deallocate Node using dealloc with Layout::new::<Node>()");
    }

    pub fn push_front(&mut self, value: Value) {
        unsafe {
            let n = Self::alloc_node(value);

            if self.len == 0 {
                self.head = n;
                self.tail = n;
            } else {
                (*n).next = self.head;
                (*self.head).prev = n;
                self.head = n;
            }

            self.len += 1;
        }
    }

    pub fn push_back(&mut self, value: Value) {
        unsafe {
            let n = Self::alloc_node(value);

            if self.len == 0 {
                self.head = n;
                self.tail = n;
            } else {
                (*n).prev = self.tail;
                (*self.tail).next = n;
                self.tail = n;
            }

            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<Value> {
        if self.len == 0 {
            return None;
        }

        unsafe {
            let n = self.head;
            let out = (*n).value;

            let next = (*n).next;
            if next.is_null() {
                self.head = null_mut();
                self.tail = null_mut();
            } else {
                (*next).prev = null_mut();
                self.head = next;
            }

            self.len -= 1;

            // hygiene: detach
            (*n).prev = null_mut();
            (*n).next = null_mut();

            Self::dealloc_node(n);
            Some(out)
        }
    }

    pub fn pop_back(&mut self) -> Option<Value> {
        if self.len == 0 {
            return None;
        }

        unsafe {
            let n = self.tail;
            let out = (*n).value;

            let prev = (*n).prev;
            if prev.is_null() {
                self.head = null_mut();
                self.tail = null_mut();
            } else {
                (*prev).next = null_mut();
                self.tail = prev;
            }

            self.len -= 1;

            (*n).prev = null_mut();
            (*n).next = null_mut();

            Self::dealloc_node(n);
            Some(out)
        }
    }

    /// Forward iterator (head → tail)
    pub fn iter_forward(&self) -> IterForward {
        IterForward {
            cur: self.head,
            remaining: self.len,
        }
    }

    /// Backward iterator (tail → head)
    pub fn iter_backward(&self) -> IterBackward {
        IterBackward {
            cur: self.tail,
            remaining: self.len,
        }
    }

    /// Optional invariant checker (useful for debugging)
    pub fn check_invariants(&self) -> bool {
        unsafe {
            if self.len == 0 {
                return self.head.is_null() && self.tail.is_null();
            }
            if self.head.is_null() || self.tail.is_null() {
                return false;
            }
            if !(*self.head).prev.is_null() {
                return false;
            }
            if !(*self.tail).next.is_null() {
                return false;
            }

            let mut count = 0usize;
            let mut cur = self.head;
            let mut prev = null_mut();

            while !cur.is_null() {
                if (*cur).prev != prev {
                    return false;
                }
                prev = cur;
                cur = (*cur).next;
                count += 1;
                if count > self.len + 1 {
                    return false; // cycle guard
                }
            }

            count == self.len && prev == self.tail
        }
    }
}

/// Forward iterator (unsafe list)
pub struct IterForward {
    cur: *mut Node,
    remaining: usize,
}

impl Iterator for IterForward {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 || self.cur.is_null() {
            return None;
        }
        unsafe {
            let v = (*self.cur).value;
            self.cur = (*self.cur).next;
            self.remaining -= 1;
            Some(v)
        }
    }
}

/// Backward iterator (unsafe list)
pub struct IterBackward {
    cur: *mut Node,
    remaining: usize,
}

impl Iterator for IterBackward {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 || self.cur.is_null() {
            return None;
        }
        unsafe {
            let v = (*self.cur).value;
            self.cur = (*self.cur).prev;
            self.remaining -= 1;
            Some(v)
        }
    }
}

// --------------------------------------------------
// TODO #3: Drop implementation
// --------------------------------------------------
//
// When the list is dropped, all remaining nodes must be freed.
//
// Requirements:
// - Walk from head → tail
// - Store next pointer BEFORE freeing current node
// - Call dealloc_node exactly once per node
// - After Drop completes, no memory should be leaked
//
impl Drop for DoublyLinkedList {
    fn drop(&mut self) {
        todo!("Walk list and deallocate all nodes");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_list_basics() {
        let mut list = DoublyLinkedList::new();

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());

        assert_eq!(list.iter_forward().collect::<Vec<_>>(), vec![]);
        assert_eq!(list.iter_backward().collect::<Vec<_>>(), vec![]);

        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);

        assert!(list.check_invariants());
    }

    #[test]
    fn single_element_push_front_pop_front() {
        let mut list = DoublyLinkedList::new();

        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert!(!list.is_empty());
        assert!(list.check_invariants());

        assert_eq!(list.iter_forward().collect::<Vec<_>>(), vec![10]);
        assert_eq!(list.iter_backward().collect::<Vec<_>>(), vec![10]);

        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
        assert!(list.check_invariants());
    }

    #[test]
    fn single_element_push_back_pop_back() {
        let mut list = DoublyLinkedList::new();

        list.push_back(-7);
        assert_eq!(list.len(), 1);
        assert!(list.check_invariants());

        assert_eq!(list.iter_forward().collect::<Vec<_>>(), vec![-7]);
        assert_eq!(list.iter_backward().collect::<Vec<_>>(), vec![-7]);

        assert_eq!(list.pop_back(), Some(-7));
        assert_eq!(list.len(), 0);
        assert!(list.check_invariants());
    }

    #[test]
    fn push_front_order_and_iterators() {
        // push_front makes forward iteration LIFO
        let mut list = DoublyLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3); // [3,2,1]

        assert_eq!(list.len(), 3);
        assert!(list.check_invariants());

        assert_eq!(list.iter_forward().collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(list.iter_backward().collect::<Vec<_>>(), vec![1, 2, 3]);
    }

    #[test]
    fn push_back_order_and_iterators() {
        // push_back makes forward iteration FIFO
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3); // [1,2,3]

        assert_eq!(list.len(), 3);
        assert!(list.check_invariants());

        assert_eq!(list.iter_forward().collect::<Vec<_>>(), vec![1, 2, 3]);
        assert_eq!(list.iter_backward().collect::<Vec<_>>(), vec![3, 2, 1]);
    }

    #[test]
    fn mixed_pushes_then_iterators() {
        // Build: [4, 2, 1, 3]
        let mut list = DoublyLinkedList::new();
        list.push_back(1); // [1]
        list.push_front(2); // [2,1]
        list.push_back(3); // [2,1,3]
        list.push_front(4); // [4,2,1,3]

        assert_eq!(list.len(), 4);
        assert!(list.check_invariants());

        assert_eq!(list.iter_forward().collect::<Vec<_>>(), vec![4, 2, 1, 3]);
        assert_eq!(list.iter_backward().collect::<Vec<_>>(), vec![3, 1, 2, 4]);
    }

    #[test]
    fn pop_front_updates_iteration_view() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3); // [1,2,3]

        assert_eq!(list.pop_front(), Some(1)); // [2,3]
        assert_eq!(list.len(), 2);
        assert!(list.check_invariants());

        assert_eq!(list.iter_forward().collect::<Vec<_>>(), vec![2, 3]);
        assert_eq!(list.iter_backward().collect::<Vec<_>>(), vec![3, 2]);
    }

    #[test]
    fn pop_back_updates_iteration_view() {
        let mut list = DoublyLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3); // [3,2,1]

        assert_eq!(list.pop_back(), Some(1)); // [3,2]
        assert_eq!(list.len(), 2);
        assert!(list.check_invariants());

        assert_eq!(list.iter_forward().collect::<Vec<_>>(), vec![3, 2]);
        assert_eq!(list.iter_backward().collect::<Vec<_>>(), vec![2, 3]);
    }

    #[test]
    fn alternating_ops_stress() {
        let mut list = DoublyLinkedList::new();

        for i in 0..200 {
            if i % 2 == 0 {
                list.push_front(i);
            } else {
                list.push_back(i);
            }
            assert!(list.check_invariants());

            // Pop from opposite side to stress rewiring
            let out = if i % 2 == 0 {
                list.pop_back()
            } else {
                list.pop_front()
            };
            assert!(out.is_some());
            assert!(list.check_invariants());
        }

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
        assert_eq!(list.iter_forward().collect::<Vec<_>>(), vec![]);
        assert_eq!(list.iter_backward().collect::<Vec<_>>(), vec![]);
        assert!(list.check_invariants());
    }

    #[test]
    fn iterators_stop_after_len() {
        // Defensive: iterator uses remaining guard to avoid infinite loop if a bug creates a cycle.
        let mut list = DoublyLinkedList::new();
        for i in 0..50 {
            list.push_back(i);
        }

        let fwd = list.iter_forward().collect::<Vec<_>>();
        let bwd = list.iter_backward().collect::<Vec<_>>();

        assert_eq!(fwd.len(), list.len());
        assert_eq!(bwd.len(), list.len());
        assert!(list.check_invariants());
    }
}
