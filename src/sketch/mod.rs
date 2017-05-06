//pub mod svg;
//pub mod color;
//mod scale;
pub mod ascii;

use sequence::*;
use alignment::*;

/// A graphics output generates a graphic for a given alignment 
pub trait GraphicsOutput<E : SequenceElement> {

    fn new() -> Self;

    fn new_with_region(offset: usize, length: usize) -> Self;

    fn template_offset(&self) -> usize;
    fn template_length(&self) -> Option<usize>;

    /// Add a section to the graphic
    fn append_section(&mut self, title: &str);

    /// Appends a raw sequence
    fn append_sequence<S: Sequence<E>>(&mut self, sequence: &S){
        let offset = self.template_offset();
        self.append_sequence_with_offset(sequence, offset)
    }

    /// Append a sequence at a given offset. The offset is given by the number of
    /// `SequenceElement`s that prequel the sequence. The offset is an absolute value,
    /// i.e., not relative to template offset!
    fn append_sequence_with_offset<S: Sequence<E>>(&mut self, sequence: &S, offset: usize);

    fn append_alignment<S: Sequence<E>>(&mut self, alignment: &Alignment<E,S>);
}

