#[cfg(test)]
#[macro_use]
extern crate assert_approx_eq;

use std::ops;

#[derive(Default, Debug)]
struct UFloat {
    n: f64,
    s: f64,
}

impl UFloat {
    fn new(n: f64, s: f64) -> Self {
        UFloat { n, s }
    }

    fn fractional_err(&self) -> f64 {
        self.s / self.n
    }
}

impl ops::Add for UFloat {
    type Output = UFloat;

    fn add(self, other: UFloat) -> UFloat {
        let n = self.n + other.n;
        let s = (self.s * self.s + other.s * other.s).sqrt();

        UFloat::new(n, s)
    }
}

impl ops::Sub for UFloat {
    type Output = UFloat;

    fn sub(self, other: UFloat) -> UFloat {
        let n = self.n - other.n;
        let s = (self.s * self.s + other.s * other.s).sqrt();

        UFloat::new(n, s)
    }
}

impl ops::Mul for UFloat {
    type Output = UFloat;

    fn mul(self, other: UFloat) -> UFloat {
        let n = self.n * other.n;
        let s = n * (self.fractional_err().powf(2.0) + other.fractional_err().powf(2.0)).sqrt();

        UFloat::new(n, s)
    }
}

impl ops::Div for UFloat {
    type Output = UFloat;

    fn div(self, other: UFloat) -> UFloat {
        let n = self.n / other.n;
        let s = n * (self.fractional_err().powf(2.0) + other.fractional_err().powf(2.0)).sqrt();

        UFloat::new(n, s)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! assert_ufloat_eq {
        ($a:expr, $b:expr) => {
            assert_ufloat_eq!($a, $b, 1E-6);
        };
        ($a:expr, $b:expr, $eps:expr) => {
            assert_ufloat_eq!($a, $b, $eps, $eps);
        };
        ($a:expr, $b:expr, $eps_a:expr, $eps_b:expr) => {
            assert_approx_eq!($a.n, $b.n, $eps_a);
            assert_approx_eq!($a.s, $b.s, $eps_b);
        };
    }

    #[test]
    fn test_single_addition() {
        let a = UFloat::new(1f64, 10f64);
        let b = UFloat::new(1f64, 20f64);
        let res = a + b;

        let expected_s = (10f64.powf(2.0) + 20f64.powf(2.0)).sqrt();

        assert_ufloat_eq!(res, UFloat::new(2f64, expected_s));
    }

    #[test]
    fn test_multiple_addition() {
        let a = UFloat::new(1f64, 10f64);
        let b = UFloat::new(1f64, 20f64);
        let c = UFloat::new(1f64, 30f64);
        let res = a + b + c;

        let expected_s = (10f64.powf(2.0) + 20f64.powf(2.0) + 30f64.powf(2.0)).sqrt();

        assert_ufloat_eq!(res, UFloat::new(3f64, expected_s));
    }

    #[test]
    fn test_single_subtraction() {
        let a = UFloat::new(2f64, 10f64);
        let b = UFloat::new(1f64, 20f64);
        let res = a - b;

        let expected_s = (10f64.powf(2.0) + 20f64.powf(2.0)).sqrt();

        assert_ufloat_eq!(res, UFloat::new(1f64, expected_s));
    }

    #[test]
    fn test_multiple_subtraction() {
        let a = UFloat::new(3f64, 10f64);
        let b = UFloat::new(1f64, 20f64);
        let c = UFloat::new(1f64, 30f64);
        let res = a - b - c;

        let expected_s = (10f64.powf(2.0) + 20f64.powf(2.0) + 30f64.powf(2.0)).sqrt();

        assert_ufloat_eq!(res, UFloat::new(1f64, expected_s));
    }

    #[test]
    fn test_single_multiplication() {
        let a = UFloat::new(100f64, 10f64);
        let b = UFloat::new(200f64, 20f64);
        let res = a * b;

        let expected_s = 2828.42712474619;

        assert_ufloat_eq!(res, UFloat::new(20000f64, expected_s));
    }

    #[test]
    fn test_multiple_multiplication() {
        let a = UFloat::new(100f64, 10f64);
        let b = UFloat::new(200f64, 20f64);
        let c = UFloat::new(0.3f64, 0.2f64);
        let res = a * b * c;

        let expected_s = 4089.0096600521747;

        assert_ufloat_eq!(res, UFloat::new(6000f64, expected_s));
    }

    #[test]
    fn test_single_division() {
        let a = UFloat::new(100f64, 10f64);
        let b = UFloat::new(200f64, 20f64);
        let res = a / b;

        let expected_s = 0.07071067811865477;

        assert_ufloat_eq!(res, UFloat::new(0.5, expected_s));
    }

    #[test]
    fn test_multiple_division() {
        let a = UFloat::new(100f64, 10f64);
        let b = UFloat::new(200f64, 20f64);
        let c = UFloat::new(0.3f64, 0.2f64);
        let res = a / b / c;

        let expected_s = 1.1358360166811596;

        assert_ufloat_eq!(res, UFloat::new(1.6666666666666667, expected_s));
    }
}
