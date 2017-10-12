extern crate svgdom;
pub use self::svgdom::*;

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

    fn append(&self, document: &mut svgdom::Document, offset_y: u64) -> (svgdom::Node, u64) {
        debug!("Appending section header with label '{}' at offset {}", self.title, offset_y);
        let g = document.create_element(ElementId::G);
        let bg = Self::add_rect(
            document,
            0f64,
            offset_y as f64,
            self.style().image_width() as f64,
            (self.style().font_size() + (2 * self.style().font_padding())) as f64,
            Some(self.style().background_color_section()),
        );
        
        let text = Self::add_text(
            document,
            &self.title,
            self.style().font_padding() as f64,
            (offset_y + self.style().font_padding() + self.style().font_size()) as f64,
            self.style().font_size(),
            false,
            true,
            Some(self.style().font_color_section()),
        );
        
        g.append(&bg);
        g.append(&text);

        (g, self.style().font_size() + 2 * self.style().font_padding())    
    }
}