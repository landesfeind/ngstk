use data::dna::DnaSequence;
use data::sequence::Sequence;

/// A segment is a DNASequence with 
#[derive(Clone,Debug)]
pub struct ReadSegment {
    /// The nucleotides of this read segment
    sequence: DnaSequence, 
    /// The corresponding 
    qualities: Option<Vec<i32>>,
    /// The offset with respect to the Read position
    offset: usize,
    /// Set to true if the read segment is mapped to the genome
    is_aligned: bool
}

impl ReadSegment {

    pub fn sequence(&self) -> &DnaSequence {
        &self.sequence
    }

    pub fn length(&self) -> usize {
        self.sequence.length()
    }

    /// Returns the 
    pub fn qualities(&self) -> &Option<Vec<i32>> {
        &self.qualities
    }

    pub fn set_qualities(&mut self, quals: &Vec<i32>) {
        assert_eq!(self.length(), quals.len() as usize);
        self.qualities = Some(quals.clone());
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset
    }

    pub fn is_aligned(&self) -> bool {
        self.is_aligned
    }
}

impl From<DnaSequence> for ReadSegment {
    fn from(seq: DnaSequence) -> ReadSegment {
        ReadSegment { 
            sequence: seq,
            qualities: None,
            offset: 0,
            is_aligned: false
        }
    }
}

impl<'a> From<&'a DnaSequence> for ReadSegment {
    fn from(seq: &DnaSequence) -> ReadSegment {
        ReadSegment::from( seq.clone() )
    }
}

#[cfg(test)]
mod tests {

    use data::readsegment::ReadSegment;
    use data::dna::DnaSequence;
    use data::sequence::Sequence;

    #[test]
    fn test_from_dnasequence(){
        let seq = DnaSequence::from("ACGTTGCAACGT");
        let rs  = ReadSegment::from(&seq);

        assert_eq!(rs.length(), seq.length());
        assert_eq!(rs.sequence().clone(), seq);
        assert_eq!(rs.offset(), 0usize);
    }

}
