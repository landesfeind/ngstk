use dna::*;
use region::RegionIdentifier;
use readsegment::*;

///
/// A read is a nucleotide sequence generated
/// from a sequencing technology (Sanger, next-generation sequencing, ...).
///
#[derive(Clone,Debug)]
pub struct Read<I : RegionIdentifier> {
    segments: Vec<ReadSegment<I>>,
    mapping_quality: Option<f64>,
}

impl<I : RegionIdentifier> Read<I> {
    /// Appends a ReadSegment to this read.
    pub fn append_segment(&mut self, segment: ReadSegment<I>) {
        self.segments.push(segment);
    }

    pub fn segments(&self) -> &Vec<ReadSegment<I>> {
        &self.segments
    }

    /// Returns the full sequence of the read which is the
    /// concatenation of all read segments.
    pub fn sequence(&self) -> DnaSequence {
        self.segments.iter().fold(DnaSequence::new(), |a, s| a + s.aligned_sequence())
    }

    /// Returns the length of the full read sequence
    pub fn length(&self) -> usize {
        self.segments.iter().fold(0, |s, r| s + r.aligned_sequence().length())
    }

    /// Returns `true` if any segment is aligned to a template sequence.
    pub fn is_aligned(&self) -> bool {
        self.segments.iter().any(|rs| rs.is_aligned())
    }
}

impl<I : RegionIdentifier> From<Vec<ReadSegment<I>>> for Read<I> {
    fn from(read_segs: Vec<ReadSegment<I>>) -> Self {
        Read {
            segments: read_segs,
            mapping_quality: None,
        }
    }
}

impl<I: RegionIdentifier> From<ReadSegment<I>> for Read<I> {
    fn from(rs: ReadSegment<I>) -> Self {
        Read::from(vec![rs])
    }
}

#[cfg(test)]
mod tests {

    use dna::DnaSequence;
    use readsegment::ReadSegment;
    use read::Read;

    //    #[test]
    //    fn test_1(){
    //        let seq : DnaSequence = DnaSequence::from("acgt");
    //        let read = Read::from( ReadSegment::from( seq.clone() ) );
    //        assert_eq!(read.sequence(), seq);
    //        assert_eq!(read.length(), 4);
    //    }
    //
    //    #[test]
    //    fn test_2(){
    //        let seq1 : DnaSequence = DnaSequence::from("acgt");
    //        let seq2 : DnaSequence = DnaSequence::from("tgca");
    //        let seq3 = seq1.clone() + seq2.clone();
    //
    //        let mut read = Read::from( ReadSegment::from( seq1.clone() ) );
    //        read.append_segment( ReadSegment::from( seq2.clone() ) );
    //        assert_eq!(read.length(), 8);
    //        assert_eq!(read.sequence(), seq3);
    //    }
}
