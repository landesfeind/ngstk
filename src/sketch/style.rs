use sketch::Color;

#[derive(Clone,Copy,Debug)]
pub struct Style {
    image_width: u64,

    background_color: Color,
    background_color_section: Color,

    // Font
    font_padding: u64,
    font_size: u64,
    font_size_section: u64,
    font_color: Color,
    font_color_section: Color,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            image_width: 500,

            background_color: Color::transparent(),
            background_color_section: Color::black().lighten_by(20),

            // Font
            font_padding: 5u64,
            font_size: 12u64,
            font_size_section: 14u64,
            font_color: Color::black().lighten_by(20),
            font_color_section: Color::white().darken_by(20),
        }

    }
}

impl Style {
    pub fn with_image_width(mut self, new_image_width: u64) -> Self {
        assert!(new_image_width > 0);
        self.image_width = new_image_width;
        self
    }

    pub fn image_width(&self) -> u64 {
        self.image_width
    }


    pub fn with_background_color(mut self, new_color: Color) -> Self {
        self.background_color = new_color;
        self
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }

    pub fn with_background_color_section(mut self, new_color: Color) -> Self {
        self.background_color_section = new_color;
        self
    }

    pub fn background_color_section(&self) -> Color {
        self.background_color_section
    }




    pub fn with_font_padding(mut self, new_font_padding: u64) -> Self {
        self.font_padding = new_font_padding;
        self
    }

    pub fn font_padding(&self) -> u64 {
        self.font_padding
    }

    pub fn with_font_size(mut self, new_font_size: u64) -> Self {
        assert!(new_font_size > 0);
        self.font_size = new_font_size;
        self
    }

    pub fn font_size(&self) -> u64 {
        self.font_size
    }

    pub fn font_size_section(&self) -> u64 {
        self.font_size_section
    }

    pub fn font_color(&self) -> Color {
        self.font_color
    }

    pub fn font_color_section(&self) -> Color {
        self.font_color_section
    }
}
