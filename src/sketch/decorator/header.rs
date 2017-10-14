use sketch::Canvas;
use sketch::decorator::Decorator;

pub struct SectionHeaderDecorator {
    title: String
}

impl SectionHeaderDecorator {
    pub fn new<S: ToString>(title: S) -> Self {
        SectionHeaderDecorator {
            title: title.to_string()
        }
    }

}

impl Decorator for SectionHeaderDecorator {
    fn draw<C: Canvas>(&self, canvas: &mut C, offset_y: u64) -> u64 {
        debug!("Appending section header with label '{}' at offset {}", self.title, offset_y);
        //let g = canvas.layer();
        
        let style = canvas.style();

        let bg_width  = style.image_width() as f64;
        let bg_height = (2 * style.font_padding() + style.font_size()) as f64;

        canvas.draw_rect(
            0f64,
            offset_y as f64,
            bg_width,
            bg_height,
            Some(style.background_color_section()),
        );
        

        let text_x = style.font_padding() as f64;
        let text_y = (offset_y + style.font_padding() + style.font_size()) as f64;
        canvas.draw_text(
            &self.title,
            text_x,
            text_y,
            style.font_size(),
            false,
            true,
            Some(style.font_color_section()),
        );
        
        2 * style.font_padding() + style.font_size()
    }
}