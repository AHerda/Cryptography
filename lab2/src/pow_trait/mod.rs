pub trait Pow: std::ops::Mul<Output = Self> + Clone {
    fn one(&self) -> Self;
    fn pow(self, mut exp: usize) -> Self {
        let mut base = self.clone();
        let mut result = self.one();

        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base.clone();
            }
            base = base.clone() * base;
            exp /= 2;
        }

        result
    }
}