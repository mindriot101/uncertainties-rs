use super::UFloat;

/* Methods */
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

    pub fn exp(&self) -> Self {
        let n = self.n.exp();
        let s = n * self.s;
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

    #[test]
    fn test_exp() {
        let a = UFloat::new(2f64, 0.1f64);
        let res = a.exp();
        assert_ufloat_eq!(res, UFloat::new(7.38905609893065, 0.7389056098930651));
    }
}
