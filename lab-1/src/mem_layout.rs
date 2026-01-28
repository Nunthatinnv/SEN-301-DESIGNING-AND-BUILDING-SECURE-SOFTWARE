// Global variable
static X: i32 = 1;

pub fn run() {
    println!("Global:\t {:p}", &X);

    // Stack
    let y = 2;
    println!("Main:\t {:p}", &y);
    test_fn_1();

    // Heap 
    let b_1 = Box::new([1; 100]);
    println!("Heap (box 1):\t {:p}", b_1);

    // Heap 
    let b_2 = Box::new([0; 100]);
    println!("Heap (box 2):\t {:p}", b_2);

    let l = vec![1, 2, 3, 4];
    // Stack 
    println!("List (struct):\t {:p}", &l);
    // Heap 
    println!("List (buffer)\t {:p}", &l[..]);

    // Heap overflow
    // zsh: killed     cargo run --bin main2

    //let l_too_large = vec![2u64; 1<<40];
    
    // Stack overflow 
    // thread 'main' (2069616) has overflowed its stack
    // fatal runtime error: stack overflow, aborting
    // zsh: abort      cargo run -- mem_layout

    //_test_recurse();

}

fn test_fn_1() {
    // Stack
    let x_1 = 3;
    println!("fn 1:\t {:p}", &x_1);
    test_fn_2();
}

fn test_fn_2() {
    // Stack
    let x_2 = 3;
    println!("fn 2:\t {:p}", &x_2);
}

fn _test_recurse() {
    if true {
        _test_recurse()
    }
}
