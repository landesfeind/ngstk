use sketch::Canvas;
use sketch::Style;

mod sectionheader;
pub use self::sectionheader::SectionHeaderDecorator;
mod sequence;
pub use self::sequence::SequenceDecorator;

pub trait Decorator {
    fn with_style(self, style: Style) -> Self;

    fn style(&self) -> Style;

    fn draw<C: Canvas>(&self, canvas: &mut C, offset_y: u64) -> u64;
}
