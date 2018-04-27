use std::fmt;

#[derive(Default, Debug)]
pub struct UFloat {
    pub n: f64,
    pub s: f64,
}

impl UFloat {
    pub fn new(n: f64, s: f64) -> Self {
        UFloat { n, s }
    }

    pub(crate) fn fractional_err(&self) -> f64 {
        self.s / self.n
    }
}

impl fmt::Display for UFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}+/-{}", self.n, self.s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let a = UFloat::new(2.6f64, 10.3f64);
        let text = format!("{}", a);
        assert_eq!(text, "2.6+/-10.3");
    }
}
