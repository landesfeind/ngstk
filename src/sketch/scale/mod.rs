pub mod numerical;
pub mod genomic;

pub trait Scale<D, R> {
    fn scale(&self, d: D) -> R;
    fn to(&self, d: D) -> R {
        self.scale(d)
    }
}

pub fn linear(domain: Vec<f64>, range: Vec<f64>) -> numerical::NumericalScale {
    numerical::NumericalScale::new(domain, range, 1f64)   
}

pub fn sqrt(domain: Vec<f64>, range: Vec<f64>) -> numerical::NumericalScale {
    numerical::NumericalScale::new(domain, range, 0.5f64)   
}

pub fn pow(domain: Vec<f64>, range: Vec<f64>, exponent: f64) -> numerical::NumericalScale {
    numerical::NumericalScale::new(domain, range, exponent)   
}
