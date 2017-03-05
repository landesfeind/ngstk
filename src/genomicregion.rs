use template::*;
use dna::*;

/// A genomic regions combines a genomic range with the actual DNA sequence.
///
/// **Important:** Genomic coordinates offseting with 0
#[derive(Clone,Debug)]
pub struct GenomicRegion {
    refname: String,
    offset: usize,
    sequence: DnaSequence,
}

impl GenomicRegion {
    /// Create a new genomic region.
    pub fn new(refname: &str, offset: usize, seq: DnaSequence) -> Self {
        GenomicRegion {
            refname: refname.to_string(),
            offset: offset,
            sequence: seq,
        }
    }
}

impl Template<DnaNucleotide, DnaSequence> for GenomicRegion {
    fn name(&self) -> &str {
        self.refname.as_ref()
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn sequence(&self) -> &DnaSequence {
        &self.sequence
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
