extern crate svgdom;
pub use self::svgdom::*;

pub struct Scale {
    pub offset: f64,
    pub factor: f64
}

impl Scale {

    pub fn new() -> Self {
        Scale::new_with_offset(0f64)
    }

    pub fn new_with_offset(offset: f64) -> Self {
        Scale::new_with_offset_and_factor(offset, 1f64)
    }

    pub fn new_with_offset_and_factor(offset: f64, factor: f64) -> Self {
        Scale { offset: offset, factor: factor }
    }

    pub fn to(&self, p: f64) -> f64 {
        self.offset + (p * self.factor)
    }

    pub fn from(&self, p: f64) -> f64 {
        (p - self.offset) / self.factor
    }

    pub fn rescale(&self, new_offset: f64, new_factor: f64) -> Self {
        Scale::new_with_offset_and_factor(self.offset + new_offset, self.factor * new_factor)
    }
}

pub trait SvgDecorator {
    
    /// Returns the width of an element `t`.
    fn width(&self) -> f64;

    /// Returns the height for an element `t`.
    fn height(&self) -> f64;

    /// Returns a SVG::Node
    fn to_svg(&self, doc: &Document, xscale: &Scale, yscale: &Scale) -> Node;
}

