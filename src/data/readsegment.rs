use data::dna::DnaSequence;
use data::sequence::Sequence;

/// A segment is a DNASequence with 
#[derive(Clone,Debug)]
pub struct ReadSegment {
    /// The nucleotides of this read segment
    sequence: DnaSequence, 
    /// The corresponding 
    qualities: Option<Vec<i32>>,
    /// The offset with respect to the read position
    offset: Option<usize>,
    /// The length of the reference that is covered by this read segment
    ref_length: Option<usize>
}

impl ReadSegment {

    pub fn sequence(&self) -> &DnaSequence {
        &self.sequence
    }

    pub fn length(&self) -> usize {
        self.sequence.length()
    }

    pub fn reference_length(&self) -> usize {
        match self.ref_length {
            Some(r) => r,
            None => 0usize
        }
    }

    /// Returns the 
    pub fn qualities(&self) -> &Option<Vec<i32>> {
        &self.qualities
    }

    pub fn set_qualities(&mut self, quals: &Vec<i32>) {
        assert_eq!(self.length(), quals.len() as usize);
        self.qualities = Some(quals.clone());
    }

}

impl From<DnaSequence> for ReadSegment {
    fn from(seq: DnaSequence) -> ReadSegment {
        ReadSegment { 
            sequence: seq,
            qualities: None,
            offset: None,
            ref_length: None
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
        assert_eq!(rs.offset(), None);
    }

    #[test]
    fn test_offset_setter(){
        let seq = DnaSequence::from("ACGTTGCAACGT");
        let mut rs  = ReadSegment::from(&seq);

        assert_eq!(rs.is_aligned(), false);
        assert_eq!(rs.offset(), None);
        rs.set_offset(100usize);
        assert_eq!(rs.is_aligned(), true);
        assert_eq!(rs.offset(), Some(100usize));
    }

}
