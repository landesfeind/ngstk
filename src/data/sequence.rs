use std::fmt;
use std::ops::Index;
use std::cmp::Ord;

/// An element that can be part of a sequence.
pub trait SequenceElement:  Ord + Eq + Clone + fmt::Debug + From<char> + Into<char> + Sized {}

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

//#[cfg(test)]
//mod tests {
//    
//    use data::sequence::*;
//
//    #[test]
//    fn test_from_vec(){
//        let seq = vec![1u8,2u8,3u8,4u8];
//
//        assert_eq!(seq.to_string(), "1234");
//        assert_eq!(seq.length(), 4);
//        assert_eq!(seq.as_vec().clone(), seq);
//    }
//
//    #[test]
//    fn test_from_string(){
//        let seq = vec![1u8,2u8,3u8,4u8];
//
//        assert_eq!(seq.to_string(), "ACGT");
//        assert_eq!(seq.length(), 4);
//    }
//
//    #[test]
//    fn test_subsequence(){
//        let seq = vec![1u8,2u8,3u8,4u8];
//
//        assert!(seq.subsequence(0,0).is_some());
//        assert!(seq.subsequence(0,1).is_some());
//        assert!(seq.subsequence(1,1).is_some());
//        assert!(seq.subsequence(1,2).is_some());
//        assert!(seq.subsequence(2,1).is_some());
//        assert!(seq.subsequence(0,4).is_some());
//        assert!(seq.subsequence(3,1).is_some());
//
//        assert!(seq.subsequence(3,2).is_none());
//        assert!(seq.subsequence(4,1).is_none());
//
//        assert_eq!(seq.subsequence(0,0).unwrap().to_string(), "");
//        assert_eq!(seq.subsequence(0,1).unwrap().to_string(), "A");
//        assert_eq!(seq.subsequence(1,1).unwrap().to_string(), "C");
//        assert_eq!(seq.subsequence(1,2).unwrap().to_string(), "CG");
//        assert_eq!(seq.subsequence(3,1).unwrap().to_string(), "T");
//    
//    }
//}
