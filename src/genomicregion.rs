pub use dna::*;
pub use template::*;

use std::ops;
use std::slice;
use std::cmp::Ordering;

/// A genomic regions combines a genomic range with an actual DNA sequence.
///
/// **Important:** Genomic coordinates offseting with 0
#[derive(Clone,Debug)]
pub struct GenomicRegion<I : RegionIdentifier> {
    refname: I,
    offset: usize,
    sequence: DnaSequence,
}

impl<I : RegionIdentifier> GenomicRegion<I> {
    /// Create a new genomic region.
    pub fn new(refname: I, offset: usize, seq: DnaSequence) -> Self {
        GenomicRegion {
            refname: refname,
            offset: offset,
            sequence: seq,
        }
    }
}

impl RegionIdentifier for String {}

impl<I : RegionIdentifier> Template<I, DnaNucleotide, DnaSequence> for GenomicRegion<I> { 
    fn sequence(&self) -> DnaSequence {
        self.sequence.clone()
    }
}

impl<I : RegionIdentifier> Region<I, DnaNucleotide> for GenomicRegion<I> {

    fn reference(&self) -> I {
        self.refname.clone()
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn end(&self) -> usize {
        self.offset + self.sequence.length() + 1
    }

}


#[cfg(test)]
mod tests {

    use dna::*;
    use template::*;
    use genomicregion::GenomicRegion;

    #[test]
    fn test_subsequence() {
        let gr = GenomicRegion::new("unknown", 0usize, DnaSequence::from("ACGTTGCA"));

        assert_eq!(gr.subsequence(0, 1), DnaSequence::from("A"));
        assert_eq!(gr.subsequence(1, 0), DnaSequence::from(""));
        assert_eq!(gr.subsequence(1, 2), DnaSequence::from("CG"));
        assert_eq!(gr.subsequence(4, 4), DnaSequence::from("TGCA"));
    }
}
