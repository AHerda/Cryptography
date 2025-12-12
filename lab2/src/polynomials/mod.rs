use super::traits::Field;

mod poly_trait_impls;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polynomial<T: Field> {
    coef: Vec<T>,
}

impl<T: Field> Field for Polynomial<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let p1: Polynomial<isize> = Polynomial::new(vec![8, 0, -6, 0, 0, 0]);
        let expected: Polynomial<isize> = Polynomial::new(vec![8, 0, -6]);
        assert_eq!(p1, expected);
    }

    #[test]
    fn test_addition() {
        let p1: Polynomial<isize> = Polynomial::new(vec![1, 2, 3]);
        let p2 = Polynomial::new(vec![4, 5, 6]);
        let expected = Polynomial::new(vec![5, 7, 9]);
        assert_eq!(p1 + p2, expected);
    }

    #[test]
    fn test_subtraction() {
        let p1: Polynomial<isize> = Polynomial::new(vec![1, 2, 3]);
        let p2 = Polynomial::new(vec![4, 5, 6]);
        let expected = Polynomial::new(vec![-3, -3, -3]);
        assert_eq!(p1 - p2, expected);
    }

    #[test]
    fn test_multiplication() {
        let p1: Polynomial<isize> = Polynomial::new(vec![1, 2, 3]);
        let p2 = Polynomial::new(vec![4, 5, 6]);
        let expected = Polynomial::new(vec![4, 13, 28, 27, 18]);
        assert_eq!(p1 * p2, expected);
    }
}
