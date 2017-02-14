pub use data::genomicrange::*;
pub use data::dna::*;

/// A genomic regions combines a genomic range with the actual DNA sequence. 
///
/// **Important:** Genomic coordinates offseting with 0
#[derive(Clone,Debug)]
pub struct GenomicRegion {
    refname: String,
    offset: usize,
    sequence: DnaSequence
}

impl GenomicRegion {

    /// Create a new genomic region. 
    pub fn new(refname: &str, offset: usize, seq: DnaSequence) -> Self {
        GenomicRegion { refname: refname.to_string(), offset: offset, sequence: seq }
    }
 
    /// Returns the genomic region name 
    pub fn refname(&self) -> &str {
        self.refname.as_ref()
    }

    /// Returns the genomic offset position (indexing starts with 0, inclusive)
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Returns the genomic end position (indexing starts with 0, inclusive)
    pub fn end(&self) -> usize {
        self.offset() + self.length()
    }

    /// Returns the length of the genomic region
    pub fn length(&self) -> usize {
        self.sequence.length()
    }

    /// Returns true of the sequence is empty, i.e., has a length of zero.
    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }

    /// Returns a reference to the actual sequence
    pub fn sequence(&self) -> &DnaSequence {
        &self.sequence
    }

    /// Extracts a sub-region of this genomic region. The sub-sequence starts
    /// at `offset() + offset` and will contain `length` nucleotides. If the
    /// requested region is out of range, `None` is returned.
    pub fn subregion(&self, offset: usize, length: usize) -> Option<GenomicRegion> {
        match self.subsequence(offset, length) {
            Some(seq) => Some(GenomicRegion::new(self.refname(), self.offset() + offset, seq)),
            None => None
        }
    }

    /// Extract the pure sub sequence of from the region
    ///
    /// The `offset` must be given relative to `self.offset()`. If the parameters result in
    /// the putative extraction of region outside of  the region that is covered by this
    /// GenomicRegion slice, then `None` is returned.
    pub fn subsequence(&self, offset: usize, length: usize) -> Option<DnaSequence> {
        match offset + length <= self.sequence.length() {
            false => None,
            true  => Some(DnaSequence::from( self.sequence[ (offset) .. (offset+length) ].to_vec() ))
        }
    }
}

#[cfg(test)]
mod tests {
    
    use data::dna::*;
    use data::genomicregion::GenomicRegion;

    #[test]
    fn test_subsequence(){
        let gr = GenomicRegion::new("unknown", 0usize, DnaSequence::from("ACGTTGCA"));

        assert_eq!( gr.subsequence(0,1), Some(DnaSequence::from("A")) );
        assert_eq!( gr.subsequence(1,0), Some(DnaSequence::from("")) );
        assert_eq!( gr.subsequence(1,2), Some(DnaSequence::from("CG")) );
        assert_eq!( gr.subsequence(4,4), Some(DnaSequence::from("TGCA")) );
        assert_eq!( gr.subsequence(4,5), None);
        assert_eq!( gr.subsequence(0,9), None);
    }
}
