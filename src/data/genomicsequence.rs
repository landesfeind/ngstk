
use data::sequence::DNASequence;

/// A genomic sequence represents a region on the DNA of an analyte. 
/// This range may 
pub struct GenomicSequence {
    name: String,
    sequence: DNASequence,
    offset: u64,
}

impl GenomicSequence {

    pub fn new(name: &str, seq: DNASequence, offset: u64) -> Self {
        assert!( ! seq.is_empty() );
        return GenomicSequence { name: name.to_string(), sequence: seq, offset: offset };
    }

}

