use sketch::Canvas;
use sketch::Color;
use sketch::decorator::Decorator;

#[derive(Clone,Debug,Copy)]
enum SectionLevel {
    H1, H2, H3, H4
}

#[derive(Clone,Debug)]
pub struct HeaderDecorator {
    title: String,
    level: SectionLevel
}

impl HeaderDecorator {
    pub fn H1<S : ToString>(title: S) -> Self {
        Self::new(title).with_level(SectionLevel::H1)
    }

    pub fn H2<S : ToString>(title: S) -> Self {
        Self::new(title).with_level(SectionLevel::H2)
    }

    pub fn H3<S : ToString>(title: S) -> Self {
        Self::new(title).with_level(SectionLevel::H3)
    }

    pub fn H4<S : ToString>(title: S) -> Self {
        Self::new(title).with_level(SectionLevel::H4)
    }

    fn new<S: ToString>(title: S) -> Self {
        HeaderDecorator {
            title: title.to_string(),
            level: SectionLevel::H2
        }
    }

    fn with_level(mut self, level: SectionLevel) -> Self {
        self.level = level;
        self
    }

    fn font_size_section_header(&self) -> f64 {
        self.font_size() * match self.level {
            SectionLevel::H1 => 2.0,
            SectionLevel::H2 => 1.4,
            SectionLevel::H3 => 1.2,
            _ => 1.0,
        }
    }

    fn background_color(&self) -> Color {
        Color::black().lighten_by(20u8)
    }

    fn font_color_section(&self) -> Color {
        Color::white().darken_by(20u8)
    }

}

impl Decorator for HeaderDecorator {
    fn draw<C: Canvas>(&self, canvas: &mut C, offset_y: f64) -> f64 {
        debug!("Appending section header with label '{}' at offset {}", self.title, offset_y);
        //let g = canvas.layer();
        
        let bg_width  = canvas.image_width() as f64;
        let bg_height = (2.0 * self.font_padding() + self.font_size()) as f64;

        canvas.draw_rect(
            0f64,
            offset_y as f64,
            bg_width,
            bg_height,
            Some(self.background_color()),
        );
        

        let text_x = self.font_padding() as f64;
        let text_y = (offset_y as f64 + self.font_padding() + self.font_size()) as f64;
        canvas.draw_text(
            &self.title,
            text_x,
            text_y,
            self.font_size(),
            false,
            true,
            Some(self.font_color_section()),
        );
        
        2.0 * self.font_padding() + self.font_size()
    }
}