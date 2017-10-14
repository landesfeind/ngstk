use sketch::Canvas;

mod header;
pub use self::header::SectionHeaderDecorator;
mod sequence;
pub use self::sequence::SequenceDecorator;

pub trait Decorator {
    fn draw<C: Canvas>(&self, canvas: &mut C, offset_y: u64) -> u64;
}
