use std::fmt::{Display,Formatter,Result};
use std::ops::Add;
use std::cmp::{Ord,Ordering};
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
    pub fn length(&self) -> usize {
        self.seq.len()
    }

    /// Returns `true` if the sequence does not contain a single nucleotide.
    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }

    /// Returns the nucleotides of the sequence as a vector
    pub fn nucleotides(&self) -> &Vec<DNANucleotide> {
        &self.seq
    }

    pub fn subsequence(&self, from: usize, length: usize) -> Self {
        return DNASequence { seq: self.seq.iter().skip(from).take(length).map(|n| n.clone() ).collect() }
    }
}

/// A trait that is implemented by all structs that comprise a DNA sequence
pub trait HasDnaSequence {

    fn dna_sequence(&self) -> &DNASequence;

    fn dna_nucleotides(&self) -> &Vec<DNANucleotide> { self.dna_sequence().nucleotides()  }

    fn dna_length(&self) -> usize { self.dna_sequence().length()  }

    fn dna_subsequence(&self, from: usize, length: usize) -> DNASequence { self.dna_sequence().subsequence(from, length) }
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

impl PartialOrd for DNASequence {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.nucleotides().partial_cmp( other.nucleotides() )
    }
}

impl Ord for DNASequence {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for DNASequence {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for DNASequence { }


impl Display for DNASequence {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let s: String = self.seq.iter().map(|n| char::from(n) ).collect();
        write!(f, "{}", s)
    }
}


#[cfg(test)]
mod tests {
    
    use data::nucleotide::DNANucleotide;
    use data::dnasequence::DNASequence;

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

    #[test]
    fn test_subsequence(){
        let seq = DNASequence::from("acgt");

        assert_eq!(seq.subsequence(0,0), DNASequence::new_empty());
        assert_eq!(seq.subsequence(0,1), DNASequence::from("A"));
        assert_eq!(seq.subsequence(1,1), DNASequence::from("C"));
        assert_eq!(seq.subsequence(1,2), DNASequence::from("CG"));
        assert_eq!(seq.subsequence(3,1), DNASequence::from("T"));
        assert_eq!(seq.subsequence(4,1), DNASequence::new_empty());
    
    }
}
