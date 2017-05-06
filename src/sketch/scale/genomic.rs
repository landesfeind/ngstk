use sketch::scale::Scale;

/// A sequence scale scales a defined region into 
/// a coordinate.
#[derive(Clone,Debug)]
pub struct SequenceScale {
    template_offset: usize,
    template_length: usize,
    element_width: f64
}

impl SequenceScale {

    /// Create a new scale using a the given per element width.
    pub fn new_with_element_width(template_offset: usize, template_length: usize, elem_width: f64) -> Self {
        SequenceScale {
            template_offset: template_offset,
            template_length: template_length,
            element_width: elem_width as f64
        }
    }

    /// Create a new scale that has a maximum width/height of `max`.
    pub fn new_with_max_width(template_offset:usize, template_length: usize, max: usize) -> Self {
        let w = (max as f64) / ((template_length) as f64);
        Self::new_with_element_width(template_offset, template_length, w)
    }
}

impl Scale<usize, f64> for SequenceScale {
    fn scale(&self, d: usize) -> f64 {
        ((d - self.template_offset) as f64) * self.element_width
    }
}


#[cfg(test)]
mod test {

    use genomicrange::GenomicRange;
    use sketch::scale::Scale;
    use sketch::scale::genomic::GenomicScale;
    
    #[test]
    fn test_simple_scale() {
        let s = GenomicScale::new_with_element_width(0, 100, 1f64);
        assert_eq!( s.scale(   0usize ),   0f64);
        assert_eq!( s.scale(   1usize ),   1f64);
        assert_eq!( s.scale(  10usize ),  10f64);
        assert_eq!( s.scale( 100usize ), 100f64);
    }

}



