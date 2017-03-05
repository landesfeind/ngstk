pub mod svg;
pub mod color;
mod scale;


use genomicregion::GenomicRegion;

const FONT_SIZE: i32 = 12;

pub trait GraphicsOutput {
    fn new(reference: GenomicRegion) -> Self;

}

