
use data::readsegment::ReadSegment;
use data::sequence::Sequence;
use data::sequence::DnaSequence;
use data::dnanucleotide::DNANucleotide;

///
/// A read is a nucleotide sequence generated
/// from a sequencing technology (Sanger, next-generation sequencing, ...).
///
#[derive(Clone,Debug)]
pub struct Read {
    segments: Vec<ReadSegment>,
    position: Option<usize>,
    mapping_quality: Option<i32>,
    is_forward: Option<bool>
}

impl Read {

    /// Appends a ReadSegment to this read.
    pub fn append_segment(&mut self, segment: ReadSegment) {
        self.segments.push( segment );
    }

    pub fn segments(&self) -> &Vec<ReadSegment> {
        &self.segments
    }

    /// Returns the full sequence of the read which is the
    /// concatenation of all read segments.
    pub fn sequence(&self) -> Vec<DNANucleotide> {
        self.segments.iter().fold(Vec::new(), |mut a,s|{ a.append(&mut s.sequence().clone()); a })
    }

    /// Returns the length of the full read sequence
    pub fn length(&self) -> usize {
        self.segments.iter().fold(0, |s,r| s + r.length() )
    }

    /// Returns `true` if the read is aligned to a genome. This is identical with having 
    /// a position assigned.
    pub fn is_aligned(&self) -> bool {
        return self.position.is_some() 
            && self.segments.iter().any( |rs| rs.is_aligned() );
    }

    /// Returns the read alignment position if the read `is_aligned()`.
    pub fn position(&self) -> Option<usize> {
        return self.position;
    }

    /// Returns the maximum end position which is the alignment start 
    /// plus the maximum of all read segment offsets plus lengths.
    pub fn position_end(&self) -> Option<usize> {
        if self.position.is_none() {
            return None
        }

        let end = self.position.unwrap() 
            + self.segments.iter().map(|s| s.offset() + s.length()).max().unwrap();
        return Some(end);
    }
     
    /// Set the alignment position of the read.
    pub fn set_position(&mut self, p: usize) {
        self.position = Some(p);
    }

    /// Returns `true` if the read is aligned onto the forward strand.
    pub fn is_forward(&self) -> Option<bool> {
        return self.is_forward;
    }
}

impl From<Vec<ReadSegment>> for Read {
    fn from(read_segs: Vec<ReadSegment>) -> Self {
        return Read { segments: read_segs, mapping_quality: None, position: None, is_forward: None }
    }
}

impl From<ReadSegment> for Read {
    fn from(rs: ReadSegment) -> Self {
        return Read::from( vec![ rs ] );
    }
}

impl From<Vec<DNANucleotide>> for Read {
    fn from(seq: Vec<DNANucleotide>) -> Self {
        return Read::from( ReadSegment::from(seq) );
    }
}

#[cfg(test)]
mod tests {
    
    use data::dnanucleotide::DNANucleotide;
    use data::sequence::DnaSequence;
    use data::readsegment::ReadSegment;
    use data::read::Read;

    #[test]
    fn test_1(){
        let seq : Vec<DNANucleotide> = DnaSequence::from("acgt");
        let read = Read::from( ReadSegment::from( seq.clone() ) );
        assert_eq!(read.sequence(), seq);
        assert_eq!(read.sequence().len(), 4);
    }

    #[test]
    fn test_2(){
        let seq1 : Vec<DNANucleotide> = DnaSequence::from("acgt");
        let seq2 : Vec<DNANucleotide> = DnaSequence::from("tgca");
        let mut seq3 = seq1.clone();
        seq3.append( &mut seq2.clone() );

        let mut read = Read::from( ReadSegment::from( seq1.clone() ) );
        read.append_segment( ReadSegment::from( seq2.clone() ) );
        assert_eq!(read.sequence().len(), 8);
        assert_eq!(read.sequence(), seq3);
    }
}
