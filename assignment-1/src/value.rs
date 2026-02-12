// =======================
// Value Representation
// =======================
// --------------------------------------------------
// Your tasks
// --------------------------------------------------
//
// 1) Implement `as_int()`:
//    - If `self` is `Value::Int(x)`, return `Some(x)`
//    - Otherwise, return `None`
//
// 2) Implement `as_float()`:
//    - If `self` is `Value::Float(x)`, return `Some(x)`
//    - Otherwise, return `None`
//
// Constraints:
// - Do NOT perform any casting between types.
// - Do NOT reinterpret bits.
// - Use pattern matching to distinguish variants.

/// Value stored in the doubly linked list.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
}

impl Value {
    /// Construct an integer value.
    pub fn int(x: i64) -> Self {
        Value::Int(x)
    }

    /// Construct a floating-point value.
    pub fn float(x: f64) -> Self {
        Value::Float(x)
    }

    /// Attempt to extract the value as an `i64`.
    ///
    /// Returns:
    /// - `Some(i64)` if this value is an integer
    /// - `None` if this value is a float
    pub fn as_int(&self) -> Option<i64> {
        todo!("Return Some(i64) if Value::Int, otherwise None");
    }

    /// Attempt to extract the value as an `f64`.
    ///
    /// Returns:
    /// - `Some(f64)` if this value is a float
    /// - `None` if this value is an integer
    pub fn as_float(&self) -> Option<f64> {
        todo!("Return Some(f64) if Value::Float, otherwise None");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_int_on_int_returns_some() {
        let v = Value::int(42);
        assert_eq!(v.as_int(), Some(42));
        assert_eq!(v.as_float(), None);
    }

    #[test]
    fn as_float_on_float_returns_some() {
        let v = Value::float(3.5);
        assert_eq!(v.as_float(), Some(3.5));
        assert_eq!(v.as_int(), None);
    }

    #[test]
    fn as_int_on_float_returns_none() {
        let v = Value::float(-1.25);
        assert_eq!(v.as_int(), None);
    }

    #[test]
    fn as_float_on_int_returns_none() {
        let v = Value::int(-7);
        assert_eq!(v.as_float(), None);
    }

    #[test]
    fn round_trip_construction_and_extraction() {
        let ints = [0, 1, -1, 123456789];
        for &x in &ints {
            let v = Value::int(x);
            assert_eq!(v.as_int(), Some(x));
            assert_eq!(v.as_float(), None);
        }

        let floats = [0.0, 1.5, -2.25, 3.141592653589793];
        for &x in &floats {
            let v = Value::float(x);
            assert_eq!(v.as_float(), Some(x));
            assert_eq!(v.as_int(), None);
        }
    }

    #[test]
    fn as_int_and_as_float_are_pure_and_do_not_modify_value() {
        let v1 = Value::int(10);
        let v2 = Value::float(2.0);

        // Call multiple times to ensure no internal state is changed.
        assert_eq!(v1.as_int(), Some(10));
        assert_eq!(v1.as_int(), Some(10));
        assert_eq!(v1.as_float(), None);

        assert_eq!(v2.as_float(), Some(2.0));
        assert_eq!(v2.as_float(), Some(2.0));
        assert_eq!(v2.as_int(), None);
    }
}
