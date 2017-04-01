use std::marker::PhantomData;
use sequence::*;

/// A template alignment represents a single part of an alignment like a match, an insertion, or an
/// deletion.
///
/// **Important:** implementations must ensure that the alignment is fully covered by the template
/// sequence.
pub trait Alignment<E: SequenceElement, S: Sequence<E>, AS : AlignmentSegment<E, S>> {

    /// Returns the sequence that is aligned against the template
    fn sequence(&self) -> S;

    /// Returns the template against which the sequence is aligned
    fn template(&self) -> S;

    /// Returns the single segments 
    fn segments(&self) -> Vec<AS>;
}

/// An aligned segment represents either a matching alignment, a mismatching alignment, 
/// or a mismatching alignment
pub trait AlignmentSegment<E: SequenceElement, S: Sequence<E>> {

    /// This function must return `true` if the `sequence_slice()` of this segment
    /// must be reversed for the alignment.
    fn is_reverse(&self) -> bool;

    /// Returns the template against which the sequence is aligned
    fn template(&self) -> Option<S>;

    /// Returns the template against which this segment
    /// is aligned against
    fn template_offset(&self) -> Option<usize>;
    fn template_length(&self) -> Option<usize>;

    fn template_slice(&self) -> Option<S> {
        match self.is_aligned() {
            true => Some( self.template().unwrap().subsequence(self.template_offset().unwrap(), self.template_length().unwrap()) ),
            false => None
        }
    }
    
    /// Returns the sequence that is aligned against the template
    fn sequence(&self) -> S;
    
    /// Returns the length of the template that is covered by this alignment.
    fn sequence_offset(&self) -> usize;
    fn sequence_length(&self) -> usize;

    /// Returns the aligned sequence slice. If the alignment `is_reverse()`
    /// the returned sequence is already reversed.
    fn sequence_slice(&self) -> S {
        match self.is_reverse() {
            true  => self.sequence().subsequence(self.sequence_offset(), self.sequence_length()).reverse(),
            false => self.sequence().subsequence(self.sequence_offset(), self.sequence_length())
        }
    }

    /// Returns `true` is this segment is truly aligned to the template,
    /// i.e., there exists a defined offset
    fn is_aligned(&self) -> bool {
        self.template().is_some() && self.template_offset().is_some() && self.template_length().is_some()
    }

    /// Returns `true` if the template sequence and the aligned sequence match.
    /// For a match, the template sequence and the aligned sequence must be identical.
    fn is_match(&self) -> bool {
        self.is_aligned() && self.template_slice().unwrap() == self.sequence_slice()
    }

    /// Returns `true` if this alignment represents a mismach.
    /// A mismatch is characterized by equal length of template sequence and aligned sequence
    /// but different sequence elements.
    fn is_mismatch(&self) -> bool {
        self.is_aligned()
            && self.template_length().unwrap() == self.sequence_length()
            && self.template_slice().unwrap() != self.sequence_slice()
    }

    /// Returns `true` if this alignment represents an insertion.
    /// An insertion is characterized by a zero-length template sequence
    /// but a non-zero-length aligned sequence.
    fn is_insertion(&self) -> bool {
        self.is_aligned() 
            && self.template_length().unwrap() == 0 
            && self.sequence_length() > 0
    }

    /// Returns `true` if this alignment represents a deletion.
    /// A deletion is characterized by a non-zero-length template sequence
    /// but a zero-length aligned sequence.
    fn is_deletion(&self) -> bool {
        self.is_aligned() 
            && self.template_length().unwrap() > 0 
            && self.sequence_length() == 0
    }

    /// Returns true if this alignment represents a complex type. A
    /// Complex type is characterized by non-zero-length template and non-zero-length aligned
    /// sequence. At the same time, the template and the aligned sequence must
    /// have different lengths.
    fn is_complex(&self) -> bool {
        self.is_aligned() 
            && self.template_length().unwrap() > 0 
            && self.sequence_length() > 0 
            && self.template_length().unwrap() != self.sequence_length()
    }
}


#[derive(Clone,Debug)]
pub struct DefaultAlignmentSegment<E: SequenceElement, S: Sequence<E>> {
    template: Option<S>,
    template_offset: Option<usize>,
    template_length: Option<usize>,
    sequence: S,
    sequence_offset: usize,
    sequence_length: usize,
    is_reverse: bool,
    _marker: PhantomData<E>
}
impl<E: SequenceElement, S: Sequence<E>> DefaultAlignmentSegment<E,S> {
    pub fn new(
        sequence: S, sequence_offset:usize, sequence_length: usize,
        template: Option<S>, template_offset: Option<usize>, template_length: Option<usize>,
        is_reverse: bool) -> Self {
        DefaultAlignmentSegment {
            template: template,
            template_offset: template_offset,
            template_length: template_length,
            sequence: sequence,
            sequence_offset: sequence_offset,
            sequence_length: sequence_length,
            is_reverse: is_reverse,
            _marker: PhantomData
        }
    }
}
impl<E: SequenceElement, S: Sequence<E>> AlignmentSegment<E, S> for DefaultAlignmentSegment<E,S> {
    /// This function must return `true` if the `sequence_slice()` of this segment
    /// must be reversed for the alignment.
    fn is_reverse(&self) -> bool {
        self.is_reverse
    }

    /// Returns the template against which the sequence is aligned
    fn template(&self) -> Option<S> {
        self.template.clone()
    }

    /// Returns the template against which this segment
    /// is aligned against
    fn template_offset(&self) -> Option<usize> {
        self.template_offset
    }

    fn template_length(&self) -> Option<usize> {
        self.template_length
    }
    
    /// Returns the sequence that is aligned against the template
    fn sequence(&self) -> S {
        self.sequence.clone()
    }
    
    /// Returns the length of the template that is covered by this alignment.
    fn sequence_offset(&self) -> usize {
        self.sequence_offset
    }
    fn sequence_length(&self) -> usize {
        self.sequence_length
    }
}

#[derive(Clone,Debug)]
pub struct DefaultAlignment<E: SequenceElement, S: Sequence<E>> {
    template: S,
    sequence: S,
    segments: Vec<DefaultAlignmentSegment<E,S>>
}

impl<E: SequenceElement, S: Sequence<E>> DefaultAlignment<E,S> {
    pub fn new(template: S, sequence: S) -> Self {
        DefaultAlignment {
            template: template,
            sequence: sequence,
            segments: Vec::new()
        }
    }

    pub fn add_segment(&mut self, sequence_offset: usize, sequence_length: usize, template_offset: usize, template_length: usize, is_reverse: bool){
        self.segments.push(
            DefaultAlignmentSegment::new(
                self.sequence.clone(), sequence_offset, sequence_length,
                Some(self.template.clone()), Some(template_offset), Some(template_length),
                is_reverse
            )
        )
    }
}
impl<E: SequenceElement, S: Sequence<E>> Alignment<E,S, DefaultAlignmentSegment<E, S>> for DefaultAlignment<E,S> {

    /// Returns the sequence that is aligned against the template
    fn sequence(&self) -> S {
        self.sequence.clone()
    }

    /// Returns the template against which the sequence is aligned
    fn template(&self) -> S {
        self.template.clone()
    }

    /// Returns the single segments 
    fn segments(&self) -> Vec<DefaultAlignmentSegment<E,S>> {
        self.segments.clone()
    }
}
