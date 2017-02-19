use data::sequence::*;

/// Templates implement sequences that represent the (a portion) of
/// some reference sequence.
pub trait Template<E: SequenceElement, S: Sequence<E>> {

    /// Returns the name of the template sequence
    fn name(&self) -> &str;

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

    fn sequence(&self) -> &S;

    fn subsequence(&self, offset: usize, length: usize) -> Option<S> {
        self.sequence().subsequence(offset, length)
    }   
}


/// A template alignment represents a single part of an alignment like a match, an insertion, or an
/// deletion.
pub trait TemplateAlignment<E: SequenceElement, S: Sequence<E>, T: Template<E,S>> {

    /// Returns the template against which this trait is aligned against
    fn template(&self) -> &T;

    /// The offset of the alignment relative to the start of the template.
    fn template_offset(&self) -> usize;

    /// Returns the number of elements that
    fn template_alignment_length(&self) -> usize;

    /// Returns the pure (sub-)sequence from the alignment that is part of
    /// the alignment
    fn template_sequence(&self) -> S {
        self.template().subsequence(self.template_offset(), self.template_alignment_length()).unwrap()
    }

    /// Returns the template against which this trait is aligned against
    fn aligned_sequence(&self) -> &S;

    fn is_aligned(&self) -> bool;

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

