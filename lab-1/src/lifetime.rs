/// Lab 1A - Lifetime (Use-after-free)
///
/// Implicit assumption:
///   "If I have a pointer, the object it points to still exists."
///
/// What this code does (intentionally wrong):
///   1) Allocate a Box<u64>
///   2) Convert it into a raw pointer `p`
///   3) Free the allocation (drop the Box)
///   4) Read through `p` anyway
///
/// Your tasks:
///   1) Identify the exact line where `p` becomes invalid (dangling).
///   2) In class: Explain why `p` still *looks* usable (it has an address), even though it's invalid.
///   3) Fix the bug by ensuring the program only frees `p` after it is read.
///
/// Note:
///   This is undefined behavior (UB). Output may vary across runs/machines.
pub fn run() {
    println!("\n== lifetime (use-after-free) ==");

    let p: *mut u64 = Box::into_raw(Box::new(0xDEAD_BEEF_DEAD_BEEF));
    println!("Allocated the first Box at p: {:p}", p);

    unsafe {
        println!("Allocated value at p: 0x{:016X}", *p);

        drop(Box::from_raw(p));

        let q: *mut u64 = Box::into_raw(Box::new(0x1111_2222_3333_4444));
        println!("Allocated a second Box at q: {:p}", q);

        let leaked_read = *p;

        println!("Read via dangling p after free: 0x{:016X}", leaked_read);

        drop(Box::from_raw(q));
    }
}
