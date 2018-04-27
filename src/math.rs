use super::UFloat;
use std::ops;

/* Addition */

impl ops::Add for UFloat {
    type Output = UFloat;

    fn add(self, other: UFloat) -> UFloat {
        let n = self.n + other.n;
        let s = (self.s * self.s + other.s * other.s).sqrt();

        UFloat::new(n, s)
    }
}

impl<'a> ops::Add for &'a UFloat {
    type Output = UFloat;

    fn add(self, other: &'a UFloat) -> UFloat {
        let n = self.n + other.n;
        let s = (self.s * self.s + other.s * other.s).sqrt();

        UFloat::new(n, s)
    }
}

impl<'a> ops::Add<UFloat> for &'a UFloat {
    type Output = UFloat;

    fn add(self, other: UFloat) -> UFloat {
        let n = self.n + other.n;
        let s = (self.s * self.s + other.s * other.s).sqrt();

        UFloat::new(n, s)
    }
}

impl<'a> ops::Add<&'a UFloat> for UFloat {
    type Output = UFloat;

    fn add(self, other: &'a UFloat) -> UFloat {
        let n = self.n + other.n;
        let s = (self.s * self.s + other.s * other.s).sqrt();

        UFloat::new(n, s)
    }
}

/* Subtraction */

impl ops::Sub for UFloat {
    type Output = UFloat;

    fn sub(self, other: UFloat) -> UFloat {
        let n = self.n - other.n;
        let s = (self.s * self.s + other.s * other.s).sqrt();

        UFloat::new(n, s)
    }
}

impl<'a> ops::Sub for &'a UFloat {
    type Output = UFloat;

    fn sub(self, other: &'a UFloat) -> UFloat {
        let n = self.n - other.n;
        let s = (self.s * self.s + other.s * other.s).sqrt();

        UFloat::new(n, s)
    }
}

impl<'a> ops::Sub<UFloat> for &'a UFloat {
    type Output = UFloat;

    fn sub(self, other: UFloat) -> UFloat {
        let n = self.n - other.n;
        let s = (self.s * self.s + other.s * other.s).sqrt();

        UFloat::new(n, s)
    }
}

impl<'a> ops::Sub<&'a UFloat> for UFloat {
    type Output = UFloat;

    fn sub(self, other: &'a UFloat) -> UFloat {
        let n = self.n - other.n;
        let s = (self.s * self.s + other.s * other.s).sqrt();

        UFloat::new(n, s)
    }
}

/* Multiplication */

impl ops::Mul for UFloat {
    type Output = UFloat;

    fn mul(self, other: UFloat) -> UFloat {
        let n = self.n * other.n;
        let s = n * (self.fractional_err().powf(2.0) + other.fractional_err().powf(2.0)).sqrt();

        UFloat::new(n, s)
    }
}

impl<'a> ops::Mul for &'a UFloat {
    type Output = UFloat;

    fn mul(self, other: &'a UFloat) -> UFloat {
        let n = self.n * other.n;
        let s = n * (self.fractional_err().powf(2.0) + other.fractional_err().powf(2.0)).sqrt();

        UFloat::new(n, s)
    }
}

impl<'a> ops::Mul<UFloat> for &'a UFloat {
    type Output = UFloat;

    fn mul(self, other: UFloat) -> UFloat {
        let n = self.n * other.n;
        let s = n * (self.fractional_err().powf(2.0) + other.fractional_err().powf(2.0)).sqrt();

        UFloat::new(n, s)
    }
}

impl<'a> ops::Mul<&'a UFloat> for UFloat {
    type Output = UFloat;

    fn mul(self, other: &'a UFloat) -> UFloat {
        let n = self.n * other.n;
        let s = n * (self.fractional_err().powf(2.0) + other.fractional_err().powf(2.0)).sqrt();

        UFloat::new(n, s)
    }
}

/* Division */

impl ops::Div for UFloat {
    type Output = UFloat;

    fn div(self, other: UFloat) -> UFloat {
        let n = self.n / other.n;
        let s = n * (self.fractional_err().powf(2.0) + other.fractional_err().powf(2.0)).sqrt();

        UFloat::new(n, s)
    }
}

impl<'a> ops::Div for &'a UFloat {
    type Output = UFloat;

    fn div(self, other: &'a UFloat) -> UFloat {
        let n = self.n / other.n;
        let s = n * (self.fractional_err().powf(2.0) + other.fractional_err().powf(2.0)).sqrt();

        UFloat::new(n, s)
    }
}

impl<'a> ops::Div<UFloat> for &'a UFloat {
    type Output = UFloat;

    fn div(self, other: UFloat) -> UFloat {
        let n = self.n / other.n;
        let s = n * (self.fractional_err().powf(2.0) + other.fractional_err().powf(2.0)).sqrt();

        UFloat::new(n, s)
    }
}

impl<'a> ops::Div<&'a UFloat> for UFloat {
    type Output = UFloat;

    fn div(self, other: &'a UFloat) -> UFloat {
        let n = self.n / other.n;
        let s = n * (self.fractional_err().powf(2.0) + other.fractional_err().powf(2.0)).sqrt();

        UFloat::new(n, s)
    }
}

/* Operators */
impl UFloat {
    pub fn sin(&self) -> Self {
        let n = self.n.sin();
        let s = (self.n.cos() * self.s).abs();

        UFloat::new(n, s)
    }

    pub fn cos(&self) -> Self {
        let n = self.n.cos();
        let s = (self.n.sin() * self.s).abs();

        UFloat::new(n, s)
    }

    pub fn log(&self, base: f64) -> Self {
        let n = self.n.log(base);
        let s = self.s / (self.n * base.ln());
        UFloat::new(n, s)
    }

    pub fn ln(&self) -> Self {
        use std::f64::consts::E;
        self.log(E)
    }

    pub fn log10(&self) -> Self {
        self.log(10.0)
    }

    pub fn powf(&self, power: f64) -> Self {
        let n = self.n.powf(power);
        let s = power * self.n.powf(power - 1.0) * self.s;
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

    #[test]
    fn test_ref_addition() {
        let a = UFloat::new(1f64, 10f64);
        let b = UFloat::new(1f64, 20f64);
        let res = &a + &b;

        let expected_s = (10f64.powf(2.0) + 20f64.powf(2.0)).sqrt();

        assert_ufloat_eq!(res, UFloat::new(2f64, expected_s));
    }

    #[test]
    fn test_ref_subtraction() {
        let a = UFloat::new(2f64, 10f64);
        let b = UFloat::new(1f64, 20f64);
        let res = &a - &b;

        let expected_s = (10f64.powf(2.0) + 20f64.powf(2.0)).sqrt();

        assert_ufloat_eq!(res, UFloat::new(1f64, expected_s));
    }

    #[test]
    fn test_sin() {
        let a = UFloat::new(2f64, 0.1f64);
        let res = a.sin();
        assert_ufloat_eq!(res, UFloat::new(0.9092974268256817, 0.04161468365471424));
    }

    #[test]
    fn test_cos() {
        let a = UFloat::new(2f64, 0.1f64);
        let res = a.cos();
        assert_ufloat_eq!(res, UFloat::new(-0.4161468365471424, 0.09092974268256818));
    }

    #[test]
    fn test_ln() {
        let a = UFloat::new(2f64, 0.1f64);
        let res = a.ln();
        assert_ufloat_eq!(res, UFloat::new(0.6931471805599453, 0.05));
    }

    #[test]
    fn test_log10() {
        let a = UFloat::new(2f64, 0.1f64);
        let res = a.log10();
        assert_ufloat_eq!(res, UFloat::new(0.3010299956639812, 0.02171472409516259));
    }

    #[test]
    fn test_powf() {
        let a = UFloat::new(2f64, 0.1f64);
        let res = a.powf(5.6);
        assert_ufloat_eq!(res, UFloat::new(48.50293012833273, 13.580820435933163));
    }
}
