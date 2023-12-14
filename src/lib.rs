/// This enum represents the sum of a sequence of numbers that may be integers or floating point.
/// Integer is the default. When a floating point number is added to the sum, the type is converted
/// to Float.
#[derive(Debug, PartialEq)]
pub enum Sum {
    Integer(i128),
    Float(f64),
}

impl Sum {
    fn new() -> Self {
        Self::Integer(0)
    }

    /// Adds `n` to the sum. The type (Integer or Float) of the Sum is unchanged.
    fn add_integer(&mut self, n: i128) {
        *self = match *self {
            Sum::Integer(i) => Sum::Integer(i + n),
            Sum::Float(f) => Sum::Float(f + n as f64),
        };
    }

    /// Adds `n` to the sum. If the type is Intger, it's changed to Float after this operation.
    fn add_float(&mut self, n: f64) {
        *self = match *self {
            Sum::Integer(i) => Sum::Float(i as f64 + n),
            Sum::Float(f) => Sum::Float(f + n),
        };
    }
}

impl Default for Sum {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_works() {
        assert_eq!(Sum::new(), Default::default());
    }

    #[test]
    fn integer_sum_works() {
        let mut sum = Sum::Integer(0);
        assert_eq!(sum, Sum::Integer(0));
        sum.add_integer(2);
        assert_eq!(sum, Sum::Integer(2));
    }

    #[test]
    fn float_sum_works() {
        let mut sum = Sum::Float(1.2);
        assert_eq!(sum, Sum::Float(1.2));
        sum.add_float(0.7);
        assert_eq!(sum, Sum::Float(1.9));
    }

    #[test]
    fn mixed_sum_works() {
        let mut sum = Sum::Integer(1);
        sum.add_float(0.2);
        assert_eq!(sum, Sum::Float(1.2));
    }
}
