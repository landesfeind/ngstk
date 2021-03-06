use sequence::*;
use std::marker::PhantomData;

/// A template alignment represents a single part of an alignment like a match, an insertion, or an
/// deletion.
///
/// **Important:** implementations must ensure that the alignment is fully covered by the template
/// sequence.
#[derive(Clone, Debug)]
pub struct Alignment<E: SequenceElement, S: Sequence<E>> {
    template: Option<S>,
    sequence: S,
    segments: Vec<AlignmentSegment<E, S>>,
    _marker: PhantomData<E>,
}

impl<E: SequenceElement, S: Sequence<E>> Alignment<E, S> {
    /// Create a new alignment between two sequences
    pub fn new(template: Option<S>, sequence: S) -> Self {
        Alignment {
            template: template,
            sequence: sequence,
            segments: Vec::new(),
            _marker: PhantomData,
        }
    }

    /// Create a new aligned
    pub fn new_aligned(template: S, sequence: S) -> Self {
        Self::new(Some(template), sequence)
    }

    pub fn new_unaligned(sequence: S) -> Self {
        Self::new(None, sequence)
    }

    /// Add a new sequence alignment segment
    pub fn add_segment(
        &mut self,
        sequence_offset: usize,
        sequence_length: usize,
        template_offset: Option<usize>,
        template_length: Option<usize>,
        is_reverse: bool,
    ) {
        let s = match template_offset {
            Some(_) => {
                match template_length {
                    Some(_) => {
                        AlignmentSegment::new(
                            self.template.clone(),
                            template_offset,
                            template_length,
                            self.sequence.clone(),
                            sequence_offset,
                            sequence_length,
                            is_reverse,
                        )
                    }
                    None => {
                        AlignmentSegment::new(
                            self.template.clone(),
                            template_offset,
                            Some(0usize),
                            self.sequence.clone(),
                            sequence_offset,
                            sequence_length,
                            is_reverse,
                        )
                    }
                }
            }
            None => {
                AlignmentSegment::new(
                    None,
                    None,
                    None,
                    self.sequence.clone(),
                    sequence_offset,
                    sequence_length,
                    is_reverse,
                )
            }
        };

        self.segments.push(s);
    }

    pub fn add_segment_aligned(
        &mut self,
        sequence_offset: usize,
        sequence_length: usize,
        template_offset: usize,
        template_length: usize,
        is_reverse: bool,
    ) {
        self.add_segment(
            sequence_offset,
            sequence_length,
            Some(template_offset),
            Some(template_length),
            is_reverse,
        );
    }

    pub fn add_segment_unaligned(&mut self, sequence_offset: usize, sequence_length: usize) {
        self.add_segment(sequence_offset, sequence_length, None, None, false);
    }


    /// Returns the sequence that is aligned against the template
    pub fn sequence(&self) -> S {
        self.sequence.clone()
    }

    /// Returns the single segments
    pub fn segments(&self) -> Vec<AlignmentSegment<E, S>> {
        self.segments.clone()
    }


    pub fn canonicalize(&self) -> Self {
        let mut new_segs = self.segments.clone();
        let mut optimization_found = true;

        while optimization_found {
            optimization_found = false;
            let mut idx = 1usize;

            while idx < new_segs.len() {
                if new_segs[idx - 1].is_insertion() && new_segs[idx].is_insertion() &&
                    new_segs[idx - 1].sequence_offset() + new_segs[idx - 1].sequence_length() ==
                        new_segs[idx].sequence_offset() &&
                    new_segs[idx - 1].template_offset() == new_segs[idx].template_offset() &&
                    new_segs[idx - 1].is_reverse() == new_segs[idx].is_reverse()
                {
                    let ns = AlignmentSegment::new(
                        self.template.clone(),
                        new_segs[idx - 1].template_offset(),
                        Some(0usize),
                        self.sequence.clone(),
                        new_segs[idx - 1].sequence_offset(),
                        new_segs[idx - 1].sequence_length() + new_segs[idx].sequence_length(),
                        new_segs[idx].is_reverse(),
                    );
                    new_segs.remove(idx);
                    new_segs.remove(idx - 1);
                    new_segs.insert(idx - 1, ns);
                    optimization_found = true;
                } else {
                    idx += 1;
                }
            }
        }

        Alignment {
            template: self.template.clone(),
            sequence: self.sequence.clone(),
            segments: new_segs,
            _marker: PhantomData,
        }
    }
}



#[derive(Clone, Debug)]
pub struct AlignmentSegment<E: SequenceElement, S: Sequence<E>> {
    template: Option<S>,
    template_offset: Option<usize>,
    template_length: Option<usize>,
    sequence: S,
    sequence_offset: usize,
    sequence_length: usize,
    is_reverse: bool,
    _marker: PhantomData<E>,
}

impl<E: SequenceElement, S: Sequence<E>> AlignmentSegment<E, S> {
    pub fn new(
        template: Option<S>,
        template_offset: Option<usize>,
        template_length: Option<usize>,
        sequence: S,
        sequence_offset: usize,
        sequence_length: usize,
        is_reverse: bool,
    ) -> Self {

        AlignmentSegment {
            template: template,
            template_offset: template_offset,
            template_length: template_length,
            sequence: sequence,
            sequence_offset: sequence_offset,
            sequence_length: sequence_length,
            is_reverse: is_reverse,
            _marker: PhantomData,
        }
    }

    /// This function must return `true` if the `sequence_slice()` of this segment
    /// must be reversed for the alignment.
    pub fn is_reverse(&self) -> bool {
        self.is_reverse
    }

    /// Returns the template against which the sequence is aligned
    pub fn template(&self) -> Option<S> {
        self.template.clone()
    }

    /// Returns the position at the template sequence where the alignment segment starts
    pub fn template_offset(&self) -> Option<usize> {
        self.template_offset
    }
    /// Returns the length of the alignem
    pub fn template_length(&self) -> Option<usize> {
        self.template_length
    }

    /// Returns the slice that is
    pub fn template_slice(&self) -> Option<S> {
        match self.is_aligned() {
            true => {
                Some(self.template().unwrap().subsequence(
                    self.template_offset().unwrap(),
                    self.template_length().unwrap(),
                ))
            }
            false => None,
        }
    }

    /// Returns the sequence that is aligned against the template
    pub fn sequence(&self) -> S {
        self.sequence.clone()
    }

    /// Returns the offset at which the alignment starts with respect to the start of the sequence.
    pub fn sequence_offset(&self) -> usize {
        self.sequence_offset
    }
    /// Returns the length of the sequence that is covered by this alignment.
    pub fn sequence_length(&self) -> usize {
        self.sequence_length
    }

    /// Returns the aligned sequence slice. If the alignment `is_reverse()`
    /// the returned sequence is already reversed.
    pub fn sequence_slice(&self) -> S {
        match self.is_reverse() {
            true => {
                self.sequence()
                    .subsequence(self.sequence_offset(), self.sequence_length())
                    .reverse()
            }
            false => {
                self.sequence().subsequence(
                    self.sequence_offset(),
                    self.sequence_length(),
                )
            }
        }
    }

    /// Returns `true` is this segment is truly aligned to the template,
    /// i.e., there exists a defined offset
    pub fn is_aligned(&self) -> bool {
        self.template().is_some() && self.template_offset().is_some() &&
            self.template_length().is_some()
    }

    /// Returns `true` if the template sequence and the aligned sequence match.
    /// For a match, the template sequence and the aligned sequence must be identical.
    pub fn is_match(&self) -> bool {
        self.is_aligned() && self.template_slice().unwrap() == self.sequence_slice()
    }

    /// Returns `true` if this alignment represents a mismach.
    /// A mismatch is characterized by equal length of template sequence and aligned sequence
    /// but different sequence elements.
    pub fn is_mismatch(&self) -> bool {
        self.is_aligned() && self.template_length().unwrap() == self.sequence_length() &&
            self.template_slice().unwrap() != self.sequence_slice()
    }

    /// Returns `true` if this alignment represents an insertion.
    /// An insertion is characterized by a zero-length template sequence
    /// but a non-zero-length aligned sequence.
    pub fn is_insertion(&self) -> bool {
        self.is_aligned() && self.template_length().unwrap() == 0 && self.sequence_length() > 0
    }

    /// Returns `true` if this alignment represents a deletion.
    /// A deletion is characterized by a non-zero-length template sequence
    /// but a zero-length aligned sequence.
    pub fn is_deletion(&self) -> bool {
        self.is_aligned() && self.template_length().unwrap() > 0 && self.sequence_length() == 0
    }

    /// Returns true if this alignment represents a complex type. A
    /// Complex type is characterized by non-zero-length template and non-zero-length aligned
    /// sequence. At the same time, the template and the aligned sequence must
    /// have different lengths.
    pub fn is_complex(&self) -> bool {
        self.is_aligned() && self.template_length().unwrap() > 0 && self.sequence_length() > 0 &&
            self.template_length().unwrap() != self.sequence_length()
    }
}
