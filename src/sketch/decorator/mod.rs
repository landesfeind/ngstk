use sketch::Canvas;
use sketch::Color;
use region::Region;

mod header;
pub use self::header::HeaderDecorator;
mod dnasequence;
pub use self::dnasequence::DnaSequenceDecorator;
mod bed;
pub use self::bed::BedRecordDecorator;


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

    fn element_width<C: Canvas>(&self, canvas: &C, region: &Region) -> f64 {
        assert!(region.has_coordinates());
        canvas.image_width() as f64 / region.length().unwrap() as f64
    }

}
