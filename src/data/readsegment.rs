use data::sequence::DNASequence;

/// A segment is a DNASequence with 
#[derive(Clone,Debug)]
pub struct ReadSegment {
    /// The nucleotides of this read segment
    nucleotides: DNASequence, 
    /// The corresponding 
    qualities: Vec<i32>,
    /// The offset with respect to the Read position
    offset: u64,
    /// Set to true if the read segment is mapped to the genome
    is_aligned: bool
}

impl ReadSegment {
    pub fn sequence(&self) -> DNASequence {
        return self.nucleotides.clone();
    }

    pub fn length(&self) -> u64 {
        return self.nucleotides.length();
    }

    pub fn qualities(&self) -> Vec<i32> {
        return self.qualities.clone();
    }

    pub fn set_qualities(&mut self, quals: &Vec<i32>) {
        assert_eq!(self.length(), quals.len() as u64);
        self.qualities = quals.clone();
    }

    pub fn offset(&self) -> u64 {
        return self.offset;
    }

    pub fn set_offset(&mut self, offset: u64) {
        self.offset = offset;
    }

    pub fn is_aligned(&self) -> bool {
        return self.is_aligned;
    }
}

impl From<DNASequence> for ReadSegment {
    fn from(seq: DNASequence) -> ReadSegment {
        let qs : Vec<i32> = seq.nucleotides().iter().map(|n| 0 as i32).collect();
        return ReadSegment { 
            nucleotides: seq,
            qualities: qs,
            offset: 0,
            is_aligned: false
        };
    }
}

