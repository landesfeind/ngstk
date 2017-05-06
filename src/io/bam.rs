extern crate rust_htslib;
use self::rust_htslib::bam;
use self::rust_htslib::bam::Read;
use self::rust_htslib::bam::record::Cigar;
use region::Region;
use std::path::Path;
use std::cmp;

use sequence::dna::*;
use alignment::*;

pub struct IndexedBamReader {
}

impl IndexedBamReader {
    fn open_region<P: AsRef<Path>>(path: &P, region: &Region) -> Result<bam::IndexedReader,String> {
        match bam::IndexedReader::from_path(path) {
            Err(e) => Err(format!("Can not open: {}", e)),

            Ok(mut bam) => match bam.header.tid( region.name().as_bytes() ) {
                None => Err(format!("Can not find region {} in BAM", region)),
                
                Some(tid) => {
                    let from = cmp::min( region.offset()                    as u32, bam.header.target_len(tid).unwrap());
                    let to   = cmp::min((region.offset() + region.length()) as u32, bam.header.target_len(tid).unwrap());
                    match bam.seek(tid, from,to) {
                        Err(e) => return Err(format!("Can not seek the position '{}': {}", region, e)),

                        Ok(_) => Ok( bam )
                    }
                }
            }
        }
    }

    pub fn load_alignments<P: AsRef<Path>>(region: &Region, reference: DnaSequence, bam_path: &P) -> Option<Vec<Alignment<DnaNucleotide, DnaSequence>>> {
        let mut aligns = Vec::new();
        let bam = Self::open_region(bam_path, region).expect("Can not open BAM file");

        for r in bam.records() {
            let record = r.ok().expect("Error reading BAM file.");
            let seq : Vec<DnaNucleotide> = record.seq().as_bytes().iter().map(|b| DnaNucleotide::from(*b as char)).collect();
            let mut alignment = Alignment::new(Some(reference.clone()), DnaSequence::from(seq));

            let mut template_pos = record.pos() as usize;
            let mut sequence_pos = 0usize;

            for c in record.cigar() {
                match c {
                    Cigar::Match(l) => {
                        alignment.add_segment(sequence_pos, l as usize, template_pos, l as usize, record.is_reverse());
                        sequence_pos += l as usize;
                        template_pos += l as usize;
                    },
                    Cigar::Ins(l) => {
                        alignment.add_segment(sequence_pos, l as usize, template_pos, 0usize, record.is_reverse());
                        sequence_pos += l as usize;
                    },
                    Cigar::Del(l) => {
                        alignment.add_segment(sequence_pos, 0usize, template_pos, l as usize, record.is_reverse());
                        template_pos += l as usize;
                    },
                    _ => {}
                }
            }

            aligns.push(alignment);
        }

        return Some(aligns);
    }
}

