
use io::fasta::FastaRecord;
use sequence::dna::DnaSequence;

/// A trait implementing a reader for FASTA files.
pub trait FastaReader {
    /// Searches for a specific sequence
    fn search<P: ToString>(&mut self, name: P) -> Option<FastaRecord>;

    /// Searches for a specific sequence
    fn search_as_sequence<P: ToString>(&mut self, name: P) -> Option<String> {
        match self.search(name) {
            Some(record) => Some(record.sequence()),
            None => None,
        }
    }

    /// Search for a specific sequence-region and extracts the subsequence
    fn search_region<P: ToString>(&mut self, name: P, offset: usize, length: usize) -> Option<FastaRecord>;

    /// Searches for a specific sequence
    fn search_region_as_sequence<P: ToString>(
        &mut self,
        name: P,
        offset: usize,
        length: usize,
    ) -> Option<String> {
        match self.search_region(name, offset, length) {
            Some(record) => Some(record.sequence()),
            None => None,
        }
    }
    
    /// Search for a specific sequence
    /// and parses it into a DnaSequence
    fn search_as_dna<P: ToString>(&mut self, name: P) -> Option<DnaSequence> {
        match self.search(name) {
            None => None, // name was not found
            Some(r) => Some(r.as_dna())
        }
    }

    /// Search for a specific sequence-region
    /// and parses it into a DnaSequence
    fn search_region_as_dna<P: ToString>(
        &mut self,
        name: P,
        offset: usize,
        length: usize,
    ) -> Option<DnaSequence> {
        match self.search_region(name, offset, length) {
            None => None, // name was not found
            Some(r) => Some(r.as_dna())
        }
    }
}
