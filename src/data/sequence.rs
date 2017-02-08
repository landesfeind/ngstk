use std::fmt::{Display,Formatter,Result};
use std::ops::Add;
use data::nucleotide::DNANucleotide;

#[derive(Clone,Debug)]
pub struct DNASequence {
    seq: Vec<DNANucleotide>
}

/// A DNA sequence is a consecutive sequence of DNA nucleotides without further information.
impl DNASequence {

    /// Returns a new DNA sequences that does not comprise any nucleotide
    pub fn new_empty() -> Self {
        return DNASequence { seq: Vec::new() };
    }

    /// Returns the length of the DNA sequence which is the number of nucleotides in it.
    pub fn length(&self) -> u64 {
        self.seq.len() as u64
    }

    /// 
    pub fn is_empty(&self) -> bool {
        self.length() > 0
    }

    pub fn nucleotides(&self) -> &Vec<DNANucleotide> {
        &self.seq
    }
}

/// A trait that is implemented by all structs that comprise a DNA sequence
pub trait HasDnaSequence {

    fn dna_sequence(&self) -> &DNASequence;

    fn dna_nucleotides(&self) -> &Vec<DNANucleotide> {
        self.dna_sequence().nucleotides()
    }

    fn dna_length(&self) -> u64 {
        self.dna_sequence().length()
    }
}

impl HasDnaSequence for DNASequence {
    fn dna_sequence(&self) -> &DNASequence { self }
}

impl From<Vec<DNANucleotide>> for DNASequence {

    fn from(s: Vec<DNANucleotide>) -> DNASequence {
        return DNASequence { seq: s };
    }
}

impl<'a> From<&'a str> for DNASequence {

    fn from(s: &str) -> DNASequence {
        return DNASequence::from( s.chars().map( |n| DNANucleotide::from(n) ).collect::<Vec<DNANucleotide>>() );
    }
}

impl Add for DNASequence {
    type Output = DNASequence;

    fn add(self, other: DNASequence) -> DNASequence {
        let mut nucleotides = self.nucleotides().clone();
        nucleotides.append( &mut other.nucleotides().clone() );
        return DNASequence::from( nucleotides ); 
    }
}

impl Display for DNASequence {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let s: String = self.seq.iter().map(|n| char::from(n) ).collect();
        write!(f, "{}", s)
    }
}


#[cfg(test)]
mod tests {
    
    use data::nucleotide::DNANucleotide;
    use data::sequence::DNASequence;

    #[test]
    fn test_from_vec(){
        let seq_vec = vec![ DNANucleotide::A, DNANucleotide::C, DNANucleotide::G, DNANucleotide::T ];
        let seq = DNASequence::from(seq_vec);

        assert_eq!(seq.to_string(), "ACGT");
        assert_eq!(seq.length(), 4);
    }

    #[test]
    fn test_from_string(){
        let seq = DNASequence::from("acgt");
        assert_eq!(seq.to_string(), "ACGT");
        assert_eq!(seq.length(), 4);
    }
}
