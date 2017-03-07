pub mod svg;
pub mod color;
mod scale;

use sequence::*;
use template::Template;

const FONT_SIZE: i32 = 12;

pub trait GraphicsOutput<E : SequenceElement, S: Sequence<SequenceElement>, T: Template<String, E, S>> {
    fn new(reference: T) -> Self;
}

