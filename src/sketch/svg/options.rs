
/// A struct containing the options for the graphics output
pub struct SvgOptions {
    /// The font size in Pixel
    pub font_size: usize,
    /// The height of individual 
    pub box_height: usize,
}

impl Default for SvgOptions {
    fn default() -> SvgOptions {
        SvgOptions {
            font_size: 12usize,
            box_height: 16usize,
        }
    }
}
impl SvgOptions {
    /// Set the default font size
    pub fn with_font_size(mut self, new_font_size: usize) -> Self {
        self.font_size = new_font_size;
        self
    }

    /// Set the default box height
    pub fn with_box_height(mut self, new_box_height: usize) -> Self {
        self.box_height = new_box_height;
        self
    }
    
}

