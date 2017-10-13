use sketch::Canvas;
use sketch::Style;

mod sectionheader;
pub use self::sectionheader::SectionHeaderDecorator;

pub trait Decorator {
    fn with_style(self, style: Style) -> Self;

    fn style(&self) -> Style;

    fn append<C: Canvas>(&self, canvas: &mut C, offset_y: u64) -> u64;
}
