/// Lab 1D - Type Confusion
///
/// Implicit assumption:
///   "If I stored an i64, I will only ever read it as an i64."
///
/// Bug:
///   This code stores values of different logical types (i64 and f64)
///   in a `union` without recording which variant is active.
///   It then reads the union using the wrong field, interpreting the same
///   bits as a different type.
///
/// Your tasks:
///   1) Replace the `union`-based storage (`RawNumber`) with the provided `Number` enum.
///   2) Update the vector to store `Number` instead of `RawNumber`.
///   3) Update the print logic to correctly match on the enum variant.
///
/// Note:
///   This is type confusion: the memory exists and is in-bounds,
///   but the program violates its own type interpretation invariant.
///   This may not crash, but it is logically incorrect.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Int(i64),
    Float(f64),
}

#[repr(C)]
union RawNumber {
    i: i64,
    f: f64,
}

pub fn run() {
    println!("\n== type confusion: union without a tag ==");

    unsafe {
        // A vector storing logically different types using a union.
        let values: Vec<RawNumber> = vec![
            RawNumber { i: 42 },
            RawNumber { f: 3.5 },
            RawNumber { i: -7 },
        ];

        // BUG: blindly read every element as f64,
        for (idx, v) in values.iter().enumerate() {
            let x: f64 = v.f;
            println!("values[{}] read as f64 = {}", idx, x);
        }
    }

    // TODO:
    //   - Change Vec<RawNumber> to Vec<Number>
    //   - Store values as Number::Int / Number::Float
    //   - Replace the unsafe loop with:
    //
    //     for v in values {
    //         match v {
    //             Number::Int(x) => println!("Int  = {}", x),
    //             Number::Float(x) => println!("Float = {}", x),
    //         }
    //     }
}
