use data::dnanucleotide::DNANucleotide;
use data::sequence::DnaSequence;

/// A segment is a DNASequence with 
#[derive(Clone,Debug)]
pub struct ReadSegment {
    /// The nucleotides of this read segment
    nucleotides: Vec<DNANucleotide>, 
    /// The corresponding 
    qualities: Vec<i32>,
    /// The offset with respect to the Read position
    offset: usize,
    /// Set to true if the read segment is mapped to the genome
    is_aligned: bool
}

impl ReadSegment {

    pub fn sequence(&self) -> &Vec<DNANucleotide> {
        &self.nucleotides
    }

    pub fn length(&self) -> usize {
        self.nucleotides.len()
    }

    pub fn qualities(&self) -> &Vec<i32> {
        &self.qualities
    }

    pub fn set_qualities(&mut self, quals: &Vec<i32>) {
        assert_eq!(self.length(), quals.len() as usize);
        self.qualities = quals.clone();
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

impl From<Vec<DNANucleotide>> for ReadSegment {
    fn from(seq: Vec<DNANucleotide>) -> ReadSegment {
        let qs : Vec<i32> = seq.iter().map(|n| 0 as i32).collect();
        return ReadSegment { 
            nucleotides: seq,
            qualities: qs,
            offset: 0,
            is_aligned: false
        };
    }
}

impl From<ReadSegment> for Vec<DNANucleotide> {

    fn from(s: ReadSegment) -> Vec<DNANucleotide> {
       s.sequence().clone()
    }
}

#[cfg(test)]
mod tests {

    use data::readsegment::ReadSegment;
    use data::dnanucleotide::DNANucleotide;
    use data::sequence::*;




    #[test]
    fn test_conversion_from_vec(){
        let seq : Vec<DNANucleotide> = DnaSequence::from("ACGGTCAGCT");
    }

}
