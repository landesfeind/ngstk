mod stream;

pub use self::stream::StreamFastaReader;
use sequence::dna::DnaSequence;
use std::iter::Iterator;
use std::str::FromStr;

/// A trait implementing a reader for FASTA files.
pub trait FastaReader: Iterator {
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
