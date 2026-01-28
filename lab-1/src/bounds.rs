use std::alloc::{alloc, dealloc, Layout};

/// Lab 1B - Bounds (Out-of-bounds write)
///
/// Implicit assumption:
///   "My index fits within the allocated buffer."
///
/// Minimal Buffer:
///   - allocation layout: [ data bytes | canary u64 ]
///   - write() is intentionally BUGGY: no bounds check, always returns Ok(())
///
/// Your tasks:
///   1) Implement bounds checking in write().
///   2) If the index is out of bound, return Err(BufferError::OutOfBounds { index, len }) 
///      and DO NOT write.
///   3) After fixing, the canary must remain unchanged after an out-of-bounds write attempt.
///
/// Note:
///   Writing out of bounds is undefined behavior (UB). The canary makes corruption visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferError {
    OutOfBounds { index: usize, len: usize },
}

pub struct Buffer {
    base: *mut u8,
    layout: Layout,
    len: usize,
}

impl Buffer {
    const CANARY_INIT: u64 = 0xDEAD_C0FFEE_C0FFEE;

    pub fn new(len: usize) -> Self {
        let total = len + std::mem::size_of::<u64>();
        let layout = Layout::from_size_align(total, 8).expect("layout");
        let base = unsafe { alloc(layout) };
        if base.is_null() {
            panic!("alloc failed");
        }

        let buf = Self { base, layout, len };

        unsafe {
            // Initialize data region to a known value.
            for i in 0..len {
                *buf.base.add(i) = b'A';
            }
            // Initialize canary right after the data region.
            *buf.canary_ptr() = Self::CANARY_INIT;
        }

        buf
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn canary(&self) -> u64 {
        unsafe { *self.canary_ptr() }
    }

    /// Write one byte at index.
    ///
    /// CURRENTLY BUGGY:
    ///   - no bounds check
    ///   - always returns Ok
    ///
    /// TODO:
    ///   - if out of bound: return Err(BufferError::OutOfBounds { index, len: self.len })
    ///   - otherwise write and return Ok(())
    pub fn write(&mut self, index: usize, value: u8) -> Result<(), BufferError> {
        unsafe {
            *self.base.add(index) = value; // BUG: may be OOB
            Ok(())
        }
    }

    #[inline]
    fn canary_ptr(&self) -> *mut u64 {
        unsafe {
            self.base.add(self.len) as *mut u64
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { dealloc(self.base, self.layout) }
    }
}

pub fn run() {
    println!("\n== bounds: Buffer::write without checks ==");

    let mut buf = Buffer::new(16);
    println!("len={} canary=0x{:016X}", buf.len(), buf.canary());

    // In-bounds write (should succeed)
    buf.write(3, b'X').unwrap();
    println!("after ok canary=0x{:016X}", buf.canary());

    // Out-of-bounds write (should become Err after fixing write()).
    let oob = buf.len() + 3;
    println!("attempt OOB write at index {}", oob);

    let res = buf.write(oob, b'Z');
    println!("write result: {:?}  canary=0x{:016X}", res, buf.canary());
}
