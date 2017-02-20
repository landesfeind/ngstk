use data::sequence::*;
use data::template::*;
use data::dna::*;
use data::genomicregion::*;

#[derive(Clone,Debug)]
pub struct ReadSegment {
    template: GenomicRegion,
    /// The nucleotides of this read segment
    sequence: DnaSequence, 
    /// The corresponding 
    qualities: Option<Vec<i32>>,
    /// The absolute offset starting at base 0 of the genomic region
    offset: Option<usize>,
    /// The length of the reference that is covered by this read segment
    ref_length: Option<usize>,
    /// Indicates if the alignment direction is forward or reverse. 
    is_forward: bool
}

impl ReadSegment {

    /// Creates a new unmapped read segment
    pub fn new(t: GenomicRegion, seq: DnaSequence) -> ReadSegment {
        ReadSegment { 
            template: t,
            sequence: seq,
            qualities: None,
            offset: None,
            ref_length: None,
            is_forward: true
        }
    }

    /// Creates a new aligned read segment
    ///
    ///  * t: the genomic region against which the alignment was performed
    ///  * seq: the sequence of the segment
    ///  * offset: the absolute offset with respect to base 0 of the genomic region
    ///  * template_length: the length of the template length that is covered by this alignment
    ///
    pub fn new_aligned(t: GenomicRegion, seq: DnaSequence, offset: usize, template_length: usize) -> ReadSegment {
        assert!( offset + template_length < t.length() );
        ReadSegment { 
            template: t,
            sequence: seq,
            qualities: None,
            offset: Some(offset),
            ref_length: Some(template_length),
            is_forward: true
        }
    }


    /// Returns the optional per-base qualitities of the sequence.
    /// The length of the returned vector equals the length of the 
    /// aligned sequence.
    pub fn qualities(&self) -> &Option<Vec<i32>> {
        &self.qualities
    }

    /// Set the per-base sequencing qualities of the aligned sequence.
    /// 
    /// # Panics
    /// 
    /// If the length of the `quals` vector does not equal the length of the
    /// sequence given in the constructor.
    pub fn set_qualities(&mut self, quals: Vec<i32>) {
        assert_eq!(self.sequence.length(), quals.len() as usize);
        self.qualities = Some(quals);
    }
}

impl TemplateAlignment<DnaNucleotide, DnaSequence, GenomicRegion> for ReadSegment {
    fn offset(&self) -> Option<usize> {
        self.offset
    }
    fn template(&self) -> &GenomicRegion {
        &self.template
    }
    fn template_alignment_length(&self) -> usize {
        match self.ref_length {
            Some(rl) => rl,
            None => 0usize
        }
    }
    fn aligned_sequence(&self) -> &DnaSequence {
        &self.sequence
    }
}

