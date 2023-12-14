use std::fmt;
use std::ops::Add;

/// This enum represents the sum of a sequence of numbers that may be integers or floating point.
/// Integer is the default. When a floating point number is added to the sum, the type is converted
/// to Float.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Sum {
    Integer(i128),
    Float(f64),
}

impl Add for Sum {
    type Output = Self;

    /// Adds two Sums. If either is a Float, the result will be a Float.
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Sum::Integer(a), Sum::Integer(b)) => Sum::Integer(a + b),
            (Sum::Float(a), Sum::Float(b)) => Sum::Float(a + b),
            (Sum::Integer(a), Sum::Float(b)) => Sum::Float(a as f64 + b),
            (Sum::Float(a), Sum::Integer(b)) => Sum::Float(a + b as f64),
        }
    }
}

impl fmt::Display for Sum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sum::Integer(n) => write!(f, "{n}"),
            Sum::Float(n) => write!(f, "{n}"),
        }
    }
}

impl fmt::UpperHex for Sum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sum::Integer(n) => fmt::UpperHex::fmt(n, f),
            Sum::Float(n) => fmt::Display::fmt(n, f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_integer_works() {
        let a = Sum::Integer(1);
        let b = Sum::Integer(2);
        assert_eq!(a + b, Sum::Integer(3));
    }

    #[test]
    fn sum_float_works() {
        let a = Sum::Float(0.2);
        let b = Sum::Float(0.8);
        assert_eq!(a + b, Sum::Float(1.0));
    }

    #[test]
    fn sum_mixed_works() {
        let a = Sum::Integer(1);
        let b = Sum::Float(0.2);
        assert_eq!(a + b, Sum::Float(1.2));
        assert_eq!(b + a, Sum::Float(1.2));
    }
}
