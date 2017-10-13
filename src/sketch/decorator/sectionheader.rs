use sketch::Canvas;
use sketch::Style;
use sketch::decorator::Decorator;

pub struct SectionHeaderDecorator {
    style: Option<Style>,
    title: String
}

impl SectionHeaderDecorator {
    pub fn new<S: ToString>(title: S) -> Self {
        SectionHeaderDecorator {
            style: None, title: title.to_string()
        }
    }

}

impl Decorator for SectionHeaderDecorator {

    fn with_style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    fn style(&self) -> Style {
        match self.style.clone() {
            Some(s) => s,
            None => Style::default()
        }
    }

    fn draw<C: Canvas>(&self, canvas: &mut C, offset_y: u64) -> u64 {
        debug!("Appending section header with label '{}' at offset {}", self.title, offset_y);
        //let g = canvas.layer();
        canvas.draw_rect(
            0f64,
            offset_y as f64,
            self.style().image_width() as f64,
            (self.style().font_size() + (2 * self.style().font_padding())) as f64,
            Some(self.style().background_color_section()),
        );
        
        canvas.draw_text(
            &self.title,
            self.style().font_padding() as f64,
            (offset_y + self.style().font_padding() + self.style().font_size()) as f64,
            self.style().font_size(),
            false,
            true,
            Some(self.style().font_color_section()),
        );
        
        self.style().font_size() + 2 * self.style().font_padding()   
    }
}