
use readsegment::ReadSegment;
use sequence::DNASequence;

///
/// A read is a nucleotide sequence generated
/// from a sequencing technology (Sanger, next-generation sequencing, ...).
///
#[derive(Clone,Debug)]
pub struct Read {
    segments: Vec<ReadSegment>,
    position: Option<u64>,
    mapping_quality: Option<i32>,
    is_forward: Option<bool>
}

impl Read {

    pub fn append_segment(&mut self, segment: ReadSegment) {
        self.segments.push( segment );
    }

    pub fn sequence(&self) -> DNASequence {
        return self.segments.iter().fold(DNASequence::empty(), |s,r| s + r.sequence() )
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

impl From<DNASequence> for Read {
    fn from(seq: DNASequence) -> Self {
        return Read::from( ReadSegment::from(seq) );
    }
}

#[cfg(test)]
mod tests {
    
    use sequence::DNASequence;
    use readsegment::ReadSegment;
    use read::Read;

    #[test]
    fn test_1(){
        let read = Read::from( ReadSegment::from( DNASequence::from("acgt") ) );
        assert_eq!(read.sequence().to_string(), "ACGT");
        assert_eq!(read.sequence().length(), 4);
    }

    #[test]
    fn test_2(){
        let mut read = Read::from( ReadSegment::from( DNASequence::from("acgt") ) );
        read.append_segment( ReadSegment::from( DNASequence::from("tgca") ) );
        assert_eq!(read.sequence().to_string(), "ACGTTGCA");
        assert_eq!(read.sequence().length(), 8);
    }
}
