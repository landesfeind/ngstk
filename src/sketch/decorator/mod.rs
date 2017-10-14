use sketch::Canvas;
use sketch::Color;

mod header;
pub use self::header::HeaderDecorator;
mod dnasequence;
pub use self::dnasequence::DnaSequenceDecorator;

pub trait Decorator {
    fn draw<C: Canvas>(&self, canvas: &mut C, offset_y: f64) -> f64;

    fn font_size(&self) -> f64 {
    	12f64
    }

    fn font_padding(&self) -> f64 {
    	0.2 * self.font_size()
    }

    fn font_color(&self) -> Color {
    	Color::black().lighten_by(20u8)
    }
}
