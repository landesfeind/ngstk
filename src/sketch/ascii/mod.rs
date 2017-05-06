use sketch::GraphicsOutput;
use sequence::*;
use alignment::*;
use std::marker::PhantomData;

pub struct AsciiOutput<E : SequenceElement> {
    template_offset: usize,
    template_length: Option<usize>,
    _marker: PhantomData<E>
}

impl<E : SequenceElement>  GraphicsOutput<E> for AsciiOutput<E> {

    fn new(offset: usize, length: usize) -> Self {
        AsciiOutput {
            template_offset: offset,
            template_length: Some(length),
            _marker: PhantomData
        }
    }

    fn template_offset(&self) -> usize         { self.template_offset }
    fn template_length(&self) -> Option<usize> { self.template_length }

    fn append_section(&mut self, title: &str) {
        println!("> {}", title);
    }

    fn append_sequence_with_offset<S: Sequence<E>>(&mut self, sequence: &S, num_elements: usize){
        if num_elements > self.template_offset() {
            for _ in 0 .. (self.template_offset() - num_elements) {
                print!(" ");
            }
        }
        let mut sequence_to_draw = sequence.clone();
        if num_elements < self.template_offset() {
            sequence_to_draw = sequence_to_draw.subsequence(self.template_offset() - num_elements, sequence.length())
        }
        println!("{}", sequence_to_draw);
    }

    fn append_alignment<S: Sequence<E>>(&mut self, alignment: &Alignment<E, S>){
        for seg in alignment.segments().iter().filter(|s| s.is_aligned()) {
            let toff = seg.template_offset().expect("Not aligned");
            for _ in 0 .. toff - self.template_offset() {
                print!(" ");
            }

            if seg.is_deletion() {
                for _ in 0 .. seg.template_length().expect("Not aligned") {
                    print!("-")
                }
            } else {
                print!("{}", seg.sequence_slice());
                if seg.is_reverse() {
                    print!("<");
                }
                else {
                    print!(">");
                }
            
            }
            println!("");
        }
    }
}
