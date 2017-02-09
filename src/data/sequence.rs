use std::fmt;
use std::ops::Add;
use std::ops::Index;
use std::cmp::{Ord,Ordering};

use data::dnanucleotide::DNANucleotide;
use data::rnanucleotide::RNANucleotide;
use data::aminoacid::Aminoacid;

/// An element that can be part of a sequence.
pub trait SequenceElement:  Ord + Eq + Clone + fmt::Debug + From<char> + Into<char> + Sized {}
impl SequenceElement for DNANucleotide {}
impl SequenceElement for RNANucleotide {}
impl SequenceElement for Aminoacid {}

/// A sequence is a consecutive sequence of sequence elements like nucleotides or amino acids
pub trait Sequence<T: SequenceElement> : Clone + Index<usize> + From<Vec<T>> + PartialEq + Eq + PartialOrd + Ord + Sized + fmt::Debug {

    /// Constructs a new empty sequence
    fn new_empty() -> Self;

    /// Returns the length of the DNA sequence which is the number of nucleotides in it.
    fn length(&self) -> usize;

    /// Returns `true` if the sequence does not contain a single nucleotide.
    fn is_empty(&self) -> bool {
        self.length() == 0
    }

    /// Returns the nucleotides of the sequence as a vector
    fn as_vec(&self) -> &Vec<T>;

    /// Returns a slice of the sequence or `None` if the coordinates
    /// are out of range.
    fn subsequence(&self, from: usize, length: usize) -> Option<Self> {
        if from + length > self.length() {
            None
        } else {
            let elems : Vec<T> = self.as_vec().iter().skip(from).take(length).map(|n| n.clone() ).collect();
            Some( elems.into() )
        }
    }


    fn from_string(s: &str) -> Self {
        let elems : Vec<T> = s.chars().map(|c| T::from(c) ).collect();
        elems.into()
    }

    fn to_string(&self) -> String {
        self.as_vec().iter().map(|n| n.clone().into() ).collect()
    }
}


pub trait DnaSequence: Sequence<DNANucleotide> {
    fn from(s: &str) -> Self {
        let elems : Vec<DNANucleotide> = s.chars().map(|c| DNANucleotide::from(c) ).collect();
        elems.into()
    }

}

pub trait RnaSequence: Sequence<RNANucleotide> {
    fn from(s: &str) -> Self {
        let elems : Vec<RNANucleotide> = s.chars().map(|c| RNANucleotide::from(c) ).collect();
        elems.into()
    }
}

pub trait Peptide: Sequence<Aminoacid> {

    fn from(s: &str) -> Self {
        let elems : Vec<Aminoacid> = s.chars().map(|c| Aminoacid::from(c) ).collect();
        elems.into()
    }

}

impl<T: SequenceElement> Sequence<T> for Vec<T> {
    fn new_empty() -> Self {
        Vec::new()
    }
    fn length(&self) -> usize {
        self.len()
    }
    fn as_vec(&self) -> &Vec<T> {
        &self
    }
}

impl DnaSequence for Vec<DNANucleotide> {}
impl RnaSequence for Vec<RNANucleotide> {}
impl Peptide for Vec<Aminoacid> {}

#[cfg(test)]
mod tests {
    
    use data::dnanucleotide::DNANucleotide;
    use data::rnanucleotide::RNANucleotide;
    use data::aminoacid::Aminoacid;
    use data::sequence::*;

    #[test]
    fn test_from_tring(){
        let dna : Vec<DNANucleotide> = DnaSequence::from("ACGT");
        let rna : Vec<RNANucleotide> = RnaSequence::from("ACGU");
        let peptide : Vec<Aminoacid> = Peptide::from("MAASGTSTTS");
    }

    #[test]
    fn test_to_tring(){
        let dna : Vec<DNANucleotide> = DnaSequence::from("ACGT");
        assert_eq!( dna.to_string(), "ACGT" );
        let rna : Vec<RNANucleotide> = RnaSequence::from("ACGU");
        assert_eq!( rna.to_string(), "ACGU" );
        let peptide : Vec<Aminoacid> = Peptide::from("MAASGTSTTS");
        assert_eq!( peptide.to_string(), "MAASGTSTTS" );
    }
    #[test]
    fn test_from_vec(){
        let seq = vec![ DNANucleotide::A, DNANucleotide::C, DNANucleotide::G, DNANucleotide::T ];

        assert_eq!(seq.to_string(), "ACGT");
        assert_eq!(seq.length(), 4);
        assert_eq!(seq.as_vec().clone(), seq);
    }

    #[test]
    fn test_from_string(){
        let seq = vec![ DNANucleotide::A, DNANucleotide::C, DNANucleotide::G, DNANucleotide::T ];

        assert_eq!(seq.to_string(), "ACGT");
        assert_eq!(seq.length(), 4);
    }

    #[test]
    fn test_subsequence(){
        let seq = vec![ DNANucleotide::A, DNANucleotide::C, DNANucleotide::G, DNANucleotide::T ];

        assert!(seq.subsequence(0,0).is_some());
        assert!(seq.subsequence(0,1).is_some());
        assert!(seq.subsequence(1,1).is_some());
        assert!(seq.subsequence(1,2).is_some());
        assert!(seq.subsequence(2,1).is_some());
        assert!(seq.subsequence(0,4).is_some());
        assert!(seq.subsequence(3,1).is_some());

        assert!(seq.subsequence(3,2).is_none());
        assert!(seq.subsequence(4,1).is_none());

        assert_eq!(seq.subsequence(0,0).unwrap().to_string(), "");
        assert_eq!(seq.subsequence(0,1).unwrap().to_string(), "A");
        assert_eq!(seq.subsequence(1,1).unwrap().to_string(), "C");
        assert_eq!(seq.subsequence(1,2).unwrap().to_string(), "CG");
        assert_eq!(seq.subsequence(3,1).unwrap().to_string(), "T");
    
    }
}
