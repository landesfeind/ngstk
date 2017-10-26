extern crate rust_htslib;
use self::rust_htslib::bam;
use self::rust_htslib::bam::Read;
use self::rust_htslib::bam::record::Cigar;
use alignment::*;
use model::*;
use sequence::dna::*;
use std::cmp;
use std::path::Path;

pub struct IndexedBamReader {}

impl IndexedBamReader {
    /// Try to open the bam file and search for a specific region
    pub fn open<P: AsRef<Path>>(path: &P) -> bam::IndexedReader {
        match bam::IndexedReader::from_path(path) {
            Err(e) => {
                panic!(
                    "Can not open IndexedBamReader for '{:?}': {}",
                    path.as_ref(),
                    e
                )
            }
            Ok(bam) => bam,
        }
    }

    /// Open a BAM file and seek for a specific region
    pub fn open_region<P: AsRef<Path>, R: Region>(path: &P, region: &R) -> bam::IndexedReader {
        let mut bam = IndexedBamReader::open(path);

        let tid = match bam.header.tid(region.template().as_bytes()) {
            None => panic!("Can not find template '{}' in BAM", region.template()),
            Some(tid) => tid,
        };
        debug!("Found region with tid={}", tid);

        // Find start of region to extract
        let from = cmp::min(
                    region.offset() as u32,
                    bam.header.target_len(tid).unwrap_or(
                        region.offset() as u32,
                    ),
                );

        // Find end of region to extract
        let to = cmp::min(
                    region.end() as u32,
                    bam.header.target_len(tid).unwrap_or(
                        region.end() as u32,
                    ),
                );

        debug!("Extracting reads in range from {} to {}", from, to);
        match bam.seek(tid, from, to) {
            Err(e) => panic!("Can not seek the position '{}:{}-{}': {}", region.template(), region.offset(), region.length(), e),
            Ok(_) => bam,
        }
    }

    /// Parse a BAM file in a specific region and
    pub fn load_alignments<P: AsRef<Path>, R:Region>(
        region: &R,
        reference: DnaSequence,
        bam_path: &P,
    ) -> Option<Vec<Alignment<DnaNucleotide, DnaSequence>>> {
        let mut aligns = Vec::new();
        let bam = Self::open_region(bam_path, region);

        for r in bam.records() {
            let record = r.expect("Can not get record");
            // Important: SAM/BAM lists sequence in correct order while
            // NgsTK alignments are 'unreversed'
            let seq: Vec<DnaNucleotide> = match record.is_reverse() {
                true => {
                    record
                        .seq()
                        .as_bytes()
                        .iter()
                        .rev()
                        .map(|b| DnaNucleotide::from(*b as char))
                        .collect()
                }
                false => {
                    record
                        .seq()
                        .as_bytes()
                        .iter()
                        .map(|b| DnaNucleotide::from(*b as char))
                        .collect()
                }
            };
            let mut alignment = Alignment::new(Some(reference.clone()), DnaSequence::from(seq));


            let mut template_pos = record.pos() as usize;
            let mut sequence_pos = 0usize;

            for c in record.cigar() {
                match c {
                    Cigar::Match(l) | Cigar::Equal(l) | Cigar::Diff(l) => {
                        alignment.add_segment_aligned(
                            sequence_pos,
                            l as usize,
                            template_pos,
                            l as usize,
                            record.is_reverse(),
                        );
                        sequence_pos += l as usize;
                        template_pos += l as usize;
                    }
                    Cigar::Ins(l) => {
                        alignment.add_segment_aligned(
                            sequence_pos,
                            l as usize,
                            template_pos,
                            0usize,
                            record.is_reverse(),
                        );
                        sequence_pos += l as usize;
                    }
                    Cigar::Del(l) => {
                        alignment.add_segment_aligned(
                            sequence_pos,
                            0usize,
                            template_pos,
                            l as usize,
                            record.is_reverse(),
                        );
                        template_pos += l as usize;
                    }
                    Cigar::RefSkip(l) => {
                        template_pos += l as usize;
                    }
                    Cigar::SoftClip(l) => {
                        alignment.add_segment_unaligned(sequence_pos, l as usize);
                        sequence_pos += l as usize;
                    }
                    Cigar::HardClip(_) => {
                        alignment.add_segment_unaligned(sequence_pos, 0usize);
                    }
                    Cigar::Pad(_) => {}
                    _ => panic!("Unknown CIGAR: {:?}", c),
                }
            }

            aligns.push(alignment.canonicalize());
        }

        return Some(aligns);
    }
}


#[cfg(test)]
mod tests {
    use io::bam::IndexedBamReader;
    use io::bam::rust_htslib::bam::Read;
    //use io::bam::rust_htslib::bam::record::Record;
    use model::*;
    use std::usize;

    #[test]
    pub fn test_bam_open() {
        IndexedBamReader::open(&"testdata/toy.bam");
    }

    #[test]
    pub fn test_bam_open_region() {
        let region = SimpleRegion::new("ref".to_string(), 0, usize::MAX);
        let bam = IndexedBamReader::open_region(&"testdata/toy.bam", &region);

        let mut count = 0;
        for r in bam.records() {
            match r.is_ok() {
                true => count = count + 1,
                false => {}
            }
        }
        assert_eq!(count, 6, "Records on region '{}'", region)
    }

    #[test]
    pub fn test_bam_open_region_range() {
        let region = SimpleRegion::new("ref", 8, 2);
        let bam = IndexedBamReader::open_region(&"testdata/toy.bam", &region);

        let mut count = 0;
        for r in bam.records() {
            match r.is_ok() {
                true => count = count + 1,
                false => {}
            }
        }
        assert_eq!(count, 3, "Records on region '{}'", region)
    }

}
