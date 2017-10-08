pub mod stream;
pub mod file;
pub mod index;
pub mod write;

pub use self::file::FastaFile;
pub use self::stream::FastaStream;
pub use self::index::IndexedFastaFile;
pub use self::write::FastaWriter;
use sequence::dna::DnaSequence;
use std::io::{Read, Seek};
use std::str::FromStr;


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


