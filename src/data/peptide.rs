
use data::aminoacid::Aminoacid;

#[derive(Clone,Debug)]
struct Peptide {
    aminoacids: Vec<Aminoacid>
}

/// A DNA sequence is a consecutive sequence of DNA nucleotides without further information.
impl Peptide {

    /// Returns a new DNA sequences that does not comprise any nucleotide
    pub fn new_empty() -> Self {
        return Peptide { aminoacids: Vec::new() };
    }

    /// Returns the length of the DNA sequence which is the number of nucleotides in it.
    pub fn length(&self) -> usize {
        self.aminoacids.len()
    }

    /// Returns `true` if the sequence does not contain a single nucleotide.
    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }

    /// Returns the nucleotides of the sequence as a vector
    pub fn aminoacids(&self) -> &Vec<Aminoacid> {
        &self.aminoacids
    }

    pub fn subsequence(&self, from: usize, length: usize) -> Self {
        return Peptide { aminoacids: self.aminoacids.iter().skip(from).take(length).map(|n| n.clone() ).collect() }
    }
}


impl From<Vec<Aminoacid>> for Peptide {
    fn from(aas: Vec<Aminoacid>) -> Peptide {
        Peptide { aminoacids: aas }
    }
}

impl<'a> From<&'a Vec<Aminoacid>> for Peptide {
    fn from(aas: &'a Vec<Aminoacid>) -> Peptide {
        Peptide::from(aas.clone())
    }
}

impl<'a> From<&'a str> for Peptide {
    fn from(aastr: &'a str) -> Peptide {
        Peptide::from(aastr.chars().map(|c| Aminoacid::from(c) ).collect::<Vec<Aminoacid>>())
    }
}



