
use data::dnasequence::{DNASequence, HasDnaSequence};

/// A genomic sequence represents a region on the DNA of an analyte. 
/// The genomic range may include the full genome (e.g., for prokaryotes), 
/// a complete chromosome, or a portion of the chromosome.
///
/// **Important:** Genomic coordinates starting with 0
pub struct GenomicRegion {
    name: String,
    sequence: DNASequence,
    offset: usize,
}

impl GenomicRegion {

    pub fn new(name: &str, seq: DNASequence) -> Self {
        Self::new_with_offset(name, seq, 0)
    }

    pub fn new_with_offset(name: &str, seq: DNASequence, offset: usize) -> Self {
        assert!( ! seq.is_empty() );
        return GenomicRegion { name: name.to_string(), sequence: seq, offset: offset};
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    /// 
    /// Extract a subsequence of the genomic region starting at *genomic position* `from` and 
    /// continuing for `length` nucleotides. GenomicRegion can not be empty and therefore
    /// `length` must be larger than `0`.
    ///
    pub fn subsequence(&self, from: usize, length: usize) -> Self {
        assert!( self.offset <= from );
        assert!( length > 0 );
        assert!( self.sequence.length() >= from - self.offset + length );
        Self::new_with_offset(self.name.as_str(), self.dna_subsequence(from - self.offset, length), from)
    }
}

impl HasDnaSequence for GenomicRegion {
    fn dna_sequence(&self) -> &DNASequence { &self.sequence }
}

#[cfg(test)]
mod tests {
    
    use data::dnasequence::{DNASequence, HasDnaSequence};
    use data::genomicregion::GenomicRegion;

    #[test]
    fn test_subsequence(){
        let gr = GenomicRegion::new("chr", DNASequence::from("ACGTTGCA"));

        assert_eq!(gr.subsequence(0,1).dna_sequence().clone(), DNASequence::from("A"));
        assert_eq!(gr.subsequence(1,1).dna_sequence().clone(), DNASequence::from("C"));
        assert_eq!(gr.subsequence(1,2).dna_sequence().clone(), DNASequence::from("CG"));
        assert_eq!(gr.subsequence(3,1).dna_sequence().clone(), DNASequence::from("T"));
        assert_eq!(gr.subsequence(3,3).dna_sequence().clone(), DNASequence::from("TTG"));
    }
}
