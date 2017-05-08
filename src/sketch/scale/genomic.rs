use sketch::scale::Scale;

/// A sequence scale scales a defined region into 
/// a coordinate.
#[derive(Clone,Debug)]
pub struct SequenceScale {
    template_offset: usize,
    element_width: f64
}

impl SequenceScale {

    /// Create a new scale using a the given per element width.
    pub fn new_with_element_width(template_offset: usize, elem_width: f64) -> Self {
        SequenceScale {
            template_offset: template_offset,
            element_width: elem_width as f64
        }
    }

    /// Create a new scale that has a maximum width/height of `max`.
    pub fn new_with_max_width(template_offset:usize, template_length: usize, max: usize) -> Self {
        let w = (max as f64) / (template_length as f64);
        Self::new_with_element_width(template_offset, w)
    }

    pub fn element_width(&self) -> f64 {
        self.element_width
    }
}

impl Scale<usize, f64> for SequenceScale {
    fn scale(&self, d: usize) -> f64 {
        (d as f64 - self.template_offset as f64) * self.element_width
    }
}


