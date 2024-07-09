pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

pub fn calculate(operation: Operation, num1: f64, num2: f64) -> Option<f64> {
    match operation {
        Operation::Addition => Some(num1 + num2),
        Operation::Subtraction => Some(num1 - num2),
        Operation::Multiplication => Some(num1 * num2),
        Operation::Division => {
            if num2 == 0.0 {
                None
            } else {
                Some(num1 / num2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(calculate(Operation::Addition, 2.0, 2.0), Some(4.0));
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(calculate(Operation::Subtraction, 5.0, 4.0), Some(1.0));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculate(Operation::Multiplication, 2.0, 2.0), Some(4.0));
    }

    #[test]
    fn test_divide() {
        assert_eq!(calculate(Operation::Division, 2.0, 2.0), Some(1.0));
    }

    #[test]
    fn test_division_by_zero() {
        assert_eq!(calculate(Operation::Division, 1.0, 0.0), None);
    }
}