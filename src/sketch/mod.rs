pub mod svg;
pub mod color;
mod scale;

use sequence::*;
use region::RegionIdentifier;
use template::Template;

const FONT_SIZE: i32 = 12;

pub trait GraphicsOutput<I : RegionIdentifier, E : SequenceElement, S: Sequence<E>, T: Template<I, E, S>> {
    fn new(reference: T) -> Self;
}

