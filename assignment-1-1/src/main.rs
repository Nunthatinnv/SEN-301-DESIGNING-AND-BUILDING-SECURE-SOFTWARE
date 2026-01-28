use std::alloc::{alloc, dealloc, Layout};
use std::ptr::{null_mut, NonNull};

/// Assignment 1 Part 1: Doubly Linked List in Unsafe Rust
///
/// Lab 1 lessons you must apply:
/// A) Lifetime: nodes must not outlive the list (no dangling pointers, correct Drop).
/// B) Bounds: index-based operations must reject invalid indices.
/// C) Aliasing: list mutations must assume exclusive access; do not overwrite what you still rely on.
/// D) Type integrity: every `*mut Node<T>` must point to a real Node allocated as Node.
///
/// Constraints:
/// - You may use `unsafe` and raw pointers.
/// - Do NOT expose raw node pointers in the public API.
/// - Enforce invariants with checks and `Result` errors.
///
/// Required invariants (must hold after every operation):
/// 1) If len == 0: head == null && tail == null
/// 2) If len > 0: head != null && tail != null
/// 3) head.prev == null, tail.next == null
/// 4) For any node n: if n.next != null then n.next.prev == n; if n.prev != null then n.prev.next == n
/// 5) len matches the number of reachable nodes from head

#[repr(C)]
struct Node<T> {
    value: T,
    prev: *mut Node<T>,
    next: *mut Node<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListError {
    Empty,
    IndexOutOfBounds { index: usize, len: usize },
    InvariantViolation,
}

pub struct DoublyLinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self { head: null_mut(), tail: null_mut(), len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // ---------- Allocation helpers (Lesson D: type integrity) ----------

    /// Allocate a Node on the heap using alloc(Layout::new::<Node<T>>()).
    /// Initialize prev/next to null.
    ///
    /// TODO: implement
    unsafe fn alloc_node(value: T) -> *mut Node<T> {
        todo!("alloc_node")
    }

    /// Deallocate a Node allocated by alloc_node.
    /// IMPORTANT: must be called exactly once per node (Lesson A: lifetime).
    ///
    /// TODO: implement
    unsafe fn dealloc_node(node: *mut Node<T>) {
        todo!("dealloc_node")
    }

    // ---------- Internal traversal (Lesson B: bounds) ----------

    /// Return pointer to node at `index` (0-based).
    /// Must return Err if index >= len.
    /// Must not dereference null.
    ///
    /// TODO: implement bounds check + traversal
    unsafe fn node_at(&self, index: usize) -> Result<*mut Node<T>, ListError> {
        todo!("node_at")
    }

    // ---------- Core operations ----------

    pub fn push_front(&mut self, value: T) {
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
        // Optional: in debug builds, you can validate invariants
        debug_assert!(self.check_invariants().is_ok());
    }

    pub fn push_back(&mut self, value: T) {
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
        debug_assert!(self.check_invariants().is_ok());
    }

    pub fn pop_front(&mut self) -> Result<T, ListError> {
        if self.len == 0 {
            return Err(ListError::Empty);
        }

        unsafe {
            let n = self.head; // valid because len > 0

            let next = (*n).next;
            if next.is_null() {
                // removing last element
                self.head = null_mut();
                self.tail = null_mut();
            } else {
                (*next).prev = null_mut();
                self.head = next;
            }

            self.len -= 1;

            // Move value out, then free node.
            // NOTE: this is unsafe because we're moving out of raw pointer memory.
            let value = std::ptr::read(&(*n).value);
            Self::dealloc_node(n);

            debug_assert!(self.check_invariants().is_ok());
            Ok(value)
        }
    }

    pub fn pop_back(&mut self) -> Result<T, ListError> {
        if self.len == 0 {
            return Err(ListError::Empty);
        }

        unsafe {
            let n = self.tail;

            let prev = (*n).prev;
            if prev.is_null() {
                self.head = null_mut();
                self.tail = null_mut();
            } else {
                (*prev).next = null_mut();
                self.tail = prev;
            }

            self.len -= 1;

            let value = std::ptr::read(&(*n).value);
            Self::dealloc_node(n);

            debug_assert!(self.check_invariants().is_ok());
            Ok(value)
        }
    }

    /// Insert at position index:
    /// - index == 0 => push_front
    /// - index == len => push_back
    /// - otherwise splice in the middle
    ///
    /// Lesson B: must bounds-check (index <= len)
    /// Lesson C: rewire pointers carefully; do not overwrite pointers you still need
    ///
    /// TODO: implement
    pub fn insert_at(&mut self, index: usize, value: T) -> Result<(), ListError> {
        todo!("insert_at")
    }

    /// Remove at position index:
    /// - index must be < len
    /// - return removed value
    ///
    /// TODO: implement
    pub fn remove_at(&mut self, index: usize) -> Result<T, ListError> {
        todo!("remove_at")
    }

    // ---------- Invariant checking (ties everything together) ----------

    /// Checks invariants listed at the top.
    /// Students should use this while debugging.
    pub fn check_invariants(&self) -> Result<(), ListError> {
        unsafe {
            if self.len == 0 {
                if !self.head.is_null() || !self.tail.is_null() {
                    return Err(ListError::InvariantViolation);
                }
                return Ok(());
            }

            if self.head.is_null() || self.tail.is_null() {
                return Err(ListError::InvariantViolation);
            }

            if !(*self.head).prev.is_null() {
                return Err(ListError::InvariantViolation);
            }
            if !(*self.tail).next.is_null() {
                return Err(ListError::InvariantViolation);
            }

            // Walk from head, verify links + count
            let mut count = 0usize;
            let mut cur = self.head;
            let mut prev = null_mut();

            while !cur.is_null() {
                if (*cur).prev != prev {
                    return Err(ListError::InvariantViolation);
                }
                if !prev.is_null() && (*prev).next != cur {
                    return Err(ListError::InvariantViolation);
                }
                prev = cur;
                cur = (*cur).next;
                count += 1;

                // Guard against cycles causing infinite loop
                if count > self.len + 1 {
                    return Err(ListError::InvariantViolation);
                }
            }

            if prev != self.tail {
                return Err(ListError::InvariantViolation);
            }
            if count != self.len {
                return Err(ListError::InvariantViolation);
            }
            Ok(())
        }
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    /// Lesson A: Lifetime safety.
    /// Must free each node exactly once, and leave no dangling pointers.
    fn drop(&mut self) {
        // TODO: implement Drop by walking from head and deallocating nodes.
        // You MUST NOT call any public methods that might rely on invariants after partial teardown.
        //
        // Hint: while cur != null { next = (*cur).next; drop value; dealloc_node(cur); cur = next; }
        todo!("Drop for DoublyLinkedList")
    }
}

// Minimal smoke test harness
fn main() {
    let mut list = DoublyLinkedList::new();
    list.push_back(10);
    list.push_back(20);
    list.push_front(5);
    println!("len={}", list.len());

    // After implementing insert/remove, these can be tested:
    // list.insert_at(1, 99).unwrap();
    // println!("pop_front={}", list.pop_front().unwrap());
    // println!("remove_at(1)={}", list.remove_at(1).unwrap());

    println!("invariants: {:?}", list.check_invariants());
}
