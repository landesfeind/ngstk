use sequence::*;

/// Templates implement sequences that represent the (a portion) of
/// some reference sequence.
pub trait Template<E: SequenceElement, S: Sequence<E>> {

    /// Returns the name of the template sequence
    fn name(&self) -> &str;

    /// Returns the length of the template sequence.
    fn length(&self) -> usize {
        self.sequence().length()
    }

    /// Returns the (optional) offset from which the template 
    /// starts. The indexing starts at zero.
    fn offset(&self) -> usize;

    /// Returns the position where the template ends
    fn end(&self) -> usize {
        self.offset() + self.length()
    }

    /// returns the sequence of the template (without offset and name functions)
    fn sequence(&self) -> &S;

    fn subsequence(&self, offset: usize, length: usize) -> Option<S> {
        self.sequence().subsequence(offset, length)
    }   
}


/// A template alignment represents a single part of an alignment like a match, an insertion, or an
/// deletion.
///
/// **Important:** implementations must ensure that the alignment is fully covered by the template
/// sequence.
pub trait TemplateAlignment<E: SequenceElement, S: Sequence<E>, T: Template<E,S>> {

    /// The absolute offset of the alignment with regard to the start of the
    /// template sequence (ignoring the template offset).
    /// Should return `None` if this object represents a sequence that is not aligned to the 
    /// reference (i.e., it is "unmapped").
    fn offset(&self) -> Option<usize>;

    /// Returns the template against which this trait is aligned against
    fn template(&self) -> &T;

    /// The offset of the template. Short for
    /// `template().offset()`
    fn template_offset(&self) -> usize {
        self.template().offset()
    }

    /// Returns the number of elements that
    fn template_alignment_length(&self) -> usize;

    /// Returns the pure (sub-)sequence from the alignment that is part of
    /// the alignment
    fn template_sequence(&self) -> S {
        self.template().subsequence(self.template_offset(), self.template_alignment_length()).unwrap()
    }

    /// The offset of the alignment relative to the start of the template.
    fn offset_relative(&self) -> Option<usize> {
        match self.offset() {
            Some(o) => Some(o - self.template_offset()),
            None => None
        }
    }

    /// Returns the template against which this trait is aligned against
    fn aligned_sequence(&self) -> &S;

    /// Returns `true` is this alignment is truly aligned to the template,
    /// i.e., there exists a defined offset
    fn is_aligned(&self) -> bool {
        self.offset().is_some() && (
            self.template_alignment_length() > 0 || self.aligned_sequence().length() > 0 )
    }

    /// Returns `true` if the template sequence and the aligned sequence match.
    /// For a match, the template sequence and the aligned sequence must be identical.
    fn is_match(&self) -> bool {
        self.is_aligned() && 
            self.template_sequence().as_vec() == self.aligned_sequence().as_vec()
    }

    /// Returns `true` if this alignment represents a mismach.
    /// A mismatch is characterized by equal length of template sequence and aligned sequence
    /// but different sequence elements.
    fn is_mismatch(&self) -> bool {
        self.is_aligned()
            && self.template_sequence().length() == self.aligned_sequence().length()
            && self.template_sequence().as_vec() != self.aligned_sequence().as_vec()
    }

    /// Returns `true` if this alignment represents an insertion.
    /// An insertion is characterized by a zero-length template sequence
    /// but a non-zero-length aligned sequence.
    fn is_insertion(&self) -> bool {
        self.is_aligned() 
            && self.template_alignment_length() == 0 
            && self.aligned_sequence().length() > 0
    }

    /// Returns `true` if this alignment represents a deletion.
    /// A deletion is characterized by a non-zero-length template sequence
    /// but a zero-length aligned sequence.
    fn is_deletion(&self) -> bool {
        self.is_aligned() 
            && self.template_alignment_length() > 0 
            && self.aligned_sequence().length() == 0
    }

    /// Returns true if this alignment represents a complex type. A 
    /// Complex type is characterized by non-zero-length template and non-zero-length aligned
    /// sequence. At the same time, the template and the aligned sequence must
    /// have different lengths.
    fn is_complex(&self) -> bool {
        self.is_aligned() 
            && self.template_alignment_length() > 0 
            && self.aligned_sequence().length() > 0
            && self.template_sequence().length() != self.aligned_sequence().length()
    }
}

