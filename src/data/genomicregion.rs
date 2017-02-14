use std::ops;

pub use data::genomicrange::*;
pub use data::dna::*;

/// A genomic regions combines a genomic range with the actual DNA sequence. 
///
/// **Important:** Genomic coordinates starting with 0
#[derive(Clone,Debug)]
pub struct GenomicRegion {
    range: GenomicRange,
    sequence: DnaSequence
}

impl GenomicRegion {
    /// Create a new genomic region. 
    ///
    /// # Panics
    ///
    /// Fails if length of `range` is not equal to the length of the `seq`.
    ///
    pub fn new(range: GenomicRange, seq: DnaSequence) -> Self {
        assert_eq!( range.length(), seq.length() );
        GenomicRegion { range: range, sequence: seq }
    }
 
    pub fn refname(&self) -> &str {
        self.range.refname()
    }
    pub fn start(&self) -> usize {
        self.range.start()
    }
    pub fn end(&self) -> usize {
        self.range.end()
    }
    pub fn length(&self) -> usize {
        self.end() - self.start() + 1
    }
    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }
    pub fn sequence(&self) -> &DnaSequence {
        &self.sequence
    }
}

impl ops::BitAnd<ops::Range<usize>> for GenomicRegion {
    type Output = Self;

    fn bitand(self, rhs: ops::Range<usize>) -> Self::Output {
        let r = self.range.clone() & rhs;
        let s = DnaSequence::from( self.sequence[ (r.start() - self.range.start()) .. (r.end() - self.range.start()) ].to_vec() );
        GenomicRegion::new(r, s)
    }
}

impl From<DnaSequence> for GenomicRegion {
    
    fn from(seq: DnaSequence) -> GenomicRegion {
        GenomicRegion::new(GenomicRange::new("unknown", 0, seq.length()), seq)
    }
    
}

#[cfg(test)]
mod tests {
    
    use data::dna::*;
    use data::genomicregion::GenomicRegion;

    #[test]
    fn test_subsequence(){
        let gr = GenomicRegion::from(DnaSequence::from("ACGTTGCA"));

        assert_eq!( (gr.clone() & (0..1)).sequence().clone(), DnaSequence::from("A") );
        assert_eq!( (gr.clone() & (1..1)).sequence().clone(), DnaSequence::from("") );
        assert_eq!( (gr.clone() & (1..2)).sequence().clone(), DnaSequence::from("C") );
        assert_eq!( (gr.clone() & (4..8)).sequence().clone(), DnaSequence::from("TGCA") );
    }
}
