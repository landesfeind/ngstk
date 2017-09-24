use sketch::scale::Scale;

#[derive(Clone, Debug)]
pub struct NumericalScale {
    domain: Vec<f64>,
    range: Vec<f64>,
    exponent: f64,
}

impl NumericalScale {
    pub fn new(domain: Vec<f64>, range: Vec<f64>, exponent: f64) -> NumericalScale {
        assert_eq!(domain.len(), range.len());
        assert!(exponent > 0f64);
        NumericalScale {
            domain: domain,
            range: range,
            exponent: exponent,
        }
    }
}

impl Scale<f64, f64> for NumericalScale {
    fn scale(&self, d: f64) -> f64 {
        if d <= self.domain[0] {
            return self.range[0];
        }

        if d >= self.domain[self.domain.len() - 1] {
            return self.range[self.domain.len() - 1];
        }

        let mut min_domain = self.domain[0];
        let mut max_domain = self.domain[self.domain.len() - 1];
        let mut min_range = self.range[0];
        let mut max_range = self.range[self.range.len() - 1];

        for i in 0..self.domain.len() {
            if self.domain[i] <= d && d <= self.domain[i + 1] {
                min_domain = self.domain[i];
                max_domain = self.domain[i + 1];
                min_range = self.range[i];
                max_range = self.range[i + 1];
            }
        }

        let scale_factor = (max_range - min_range) / (max_domain - min_domain);
        min_range + scale_factor * (d - min_domain).powf(self.exponent)
    }
}



#[cfg(test)]
mod test {

    use sketch::scale::Scale;
    use sketch::scale::numerical::NumericalScale;

    #[test]
    fn test_linear_scale() {
        let s = NumericalScale::new(vec![0f64, 1f64], vec![0f64, 1f64], 1f64);
        assert_eq!(s.scale(-0.000001), 0f64);
        assert_eq!(s.scale(0f64), 0f64);
        assert_eq!(s.scale(0.000001), 0.000001);
        assert_eq!(s.scale(0.1), 0.1);
        assert_eq!(s.scale(0.5), 0.5);
        assert_eq!(s.scale(0.99999), 0.99999);
        assert_eq!(s.scale(1.0), 1.0);
        assert_eq!(s.scale(1.0000001), 1.0);
    }


    #[test]
    fn test_linear_scale_shift() {
        let s = NumericalScale::new(vec![0f64, 1f64], vec![10f64, 11f64], 1f64);
        assert_eq!(s.scale(-0.000001), 10f64);
        assert_eq!(s.scale(0f64), 10f64);
        assert_eq!(s.scale(0.000001), 10.000001);
        assert_eq!(s.scale(0.1), 10.1);
        assert_eq!(s.scale(0.5), 10.5);
        assert_eq!(s.scale(0.99999), 10.99999);
        assert_eq!(s.scale(1.0), 11.0);
        assert_eq!(s.scale(1.0000001), 11.0);
    }

    #[test]
    fn test_sqrt_scale() {
        let s = NumericalScale::new(vec![0f64, 10f64], vec![0f64, 10f64], 0.5f64);
        assert_eq!(s.scale(-0.000001), 0f64);
        assert_eq!(s.scale(0f64), 0f64);
        assert_eq!(s.scale(1.0), 1.0);
        assert_eq!(s.scale(2.0), 2f64.sqrt());
    }


    #[test]
    fn test_sqrt_pow2() {
        let s = NumericalScale::new(vec![0f64, 10f64], vec![0f64, 10f64], 2f64);
        assert_eq!(s.scale(-0.000001), 0f64);
        assert_eq!(s.scale(0f64), 0f64);
        assert_eq!(s.scale(1.0), 1.0);
        assert_eq!(s.scale(2.0), 2f64.powi(2));
    }
}
