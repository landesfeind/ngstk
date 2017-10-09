pub mod stream;
pub mod file;
pub mod index;
pub mod write;

pub use self::file::FastaFile;
pub use self::index::IndexedFastaFile;
pub use self::stream::FastaStream;
pub use self::write::FastaWriter;
use sequence::dna::{DnaNucleotide,DnaSequence};
use sequence::rna::{RnaNucleotide,RnaSequence};
use sequence::aminoacid::{Aminoacid,Peptide};
use std::io::{Read, Seek};
use std::str::FromStr;
use std::iter::Iterator;

#[derive(Clone,Debug)]
pub struct FastaRecord {
    header: String,
    sequence: String
}

impl FastaRecord {

    pub fn new<H: ToString, S: ToString>(header: H, sequence: S) -> FastaRecord {
        FastaRecord {
            header: header.to_string(),
            sequence: sequence.to_string().chars().filter(|c| ! c.is_whitespace() ).collect()
        }
    }

    /// Return the complete header
    pub fn header(&self) -> String {
        self.header.clone()
    }

    /// Return the first non-whitespace part of the header
    pub fn name(&self) -> String {
        self.header.chars().take_while(|c| !c.is_whitespace() ).collect()
    }

    // Returns the sequence
    pub fn sequence(&self) -> String {
        self.sequence.clone()
    }

    /// Converts the 
    pub fn as_dna(&self) -> DnaSequence {
        DnaSequence::from(self)
    }

    pub fn as_rna(&self) -> RnaSequence {
        RnaSequence::from(self)
    }

    pub fn as_peptide(&self) -> Peptide {
        Peptide::from(self)
    }
}

impl<'a> From<&'a FastaRecord> for DnaSequence {
    fn from(r: &FastaRecord) -> DnaSequence {
        let nucs : Vec<DnaNucleotide> = r.sequence().chars().map(|c| DnaNucleotide::from(c)).collect();
        DnaSequence::from(nucs)
    }
}

impl<'a> From<&'a FastaRecord> for RnaSequence {
    fn from(r: &FastaRecord) -> RnaSequence {
        let nucs : Vec<RnaNucleotide> = r.sequence().chars().map(|c| RnaNucleotide::from(c)).collect();
        RnaSequence::from(nucs)
    }
}

impl<'a> From<&'a FastaRecord> for Peptide {
    fn from(r: &FastaRecord) -> Peptide {
        let nucs : Vec<Aminoacid> = r.sequence().chars().map(|c| Aminoacid::from(c)).collect();
        Peptide::from(nucs)
    }
}


/// A trait implementing a reader for FASTA files.
pub trait FastaReader {
    /// Searches for a specific sequence
    fn search(&mut self, name: &str) -> Option<String>;

    /// Search for a specific sequence-region and extracts the subsequence
    fn search_region(&mut self, name: &str, offset: usize, length: usize) -> Option<String>;


    /// Helper methods that searches for a specific sequence
    /// and parses it into a DnaSequence
    fn search_dna(&mut self, name: &str) -> Option<DnaSequence> {
        match self.search(name) {
            None => None, // name was not found
            Some(s) => {
                match DnaSequence::from_str(s.as_ref()) {
                    Ok(dna_seq) => Some(dna_seq),
                    _ => None,
                }
            }
        }
    }

    /// Helper method that searches for a specific sequence-region
    /// and parses it into a DnaSequence
    fn search_dna_region(
        &mut self,
        name: &str,
        offset: usize,
        length: usize,
    ) -> Option<DnaSequence> {
        match self.search_region(name, offset, length) {
            None => None, // name was not found
            Some(s) => {
                match DnaSequence::from_str(s.as_ref()) {
                    Ok(dna_seq) => Some(dna_seq),
                    _ => None,
                }
            }
        }
    }
}

pub fn read_stream<R: Read>(input: R) -> FastaStream<R> {
    FastaStream::from(input)
}

pub fn read_file<R: Read + Seek>(input: R) -> FastaFile<R> {
    FastaFile::from(input)
}
