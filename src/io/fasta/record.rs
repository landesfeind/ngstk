use std::cmp::{Ord, Ordering};

use sequence::dna::{DnaNucleotide,DnaSequence};
use sequence::rna::{RnaNucleotide,RnaSequence};
use sequence::aminoacid::{Aminoacid,Peptide};

#[derive(Clone,Debug)]
pub struct FastaRecord {
    header: String,
    sequence: String
}

/// Struct for resembling records from a FASTA file. 
/// This is basically a struct containing two strings.
impl FastaRecord {

    /// Create a new record
    pub fn new<H: ToString, S: ToString>(header: H, sequence: S) -> FastaRecord {
        FastaRecord {
            header: header.to_string(),
            sequence: sequence.to_string().chars().filter(|c| ! c.is_whitespace() ).collect()
        }
    }

    /// Return the header part
    pub fn header(&self) -> String {
        self.header.clone()
    }

    /// Return the first non-whitespace part of the header
    pub fn name(&self) -> String {
        self.header.chars().take_while(|c| !c.is_whitespace() ).collect()
    }

    /// Returns the sequence part
    pub fn sequence(&self) -> String {
        self.sequence.clone()
    }

    /// Converts the sequence into a DNA sequence
    pub fn as_dna(&self) -> DnaSequence {
        DnaSequence::from(self)
    }

    /// Converts the sequence into a RNA sequence
    pub fn as_rna(&self) -> RnaSequence {
        RnaSequence::from(self)
    }

    /// Converts the sequence into a peptide
    pub fn as_peptide(&self) -> Peptide {
        Peptide::from(self)
    }

    pub fn subsequence(&self, offset: usize, length: usize) -> Self {
        Self::new(
            self.header.clone(),
            self.sequence.chars().skip(offset).take(length).collect::<String>()
        )    
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


impl PartialEq for FastaRecord {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for FastaRecord {}

impl PartialOrd for FastaRecord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let o = self.name().cmp(&other.name());
        match o {
            Ordering::Equal => Some(self.sequence().cmp(&other.sequence())),
            _ => Some(o)
        }
    }
}

impl Ord for FastaRecord {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

