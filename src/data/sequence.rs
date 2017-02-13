use std::fmt;
use std::ops::Index;
use std::ops::Range;
use std::ops::Add;
use std::cmp::Ord;
use std::slice;

/// An element that can be part of a sequence.
pub trait SequenceElement:  Ord + Eq + Clone + fmt::Debug + fmt::Display + Sized {}

/// A sequence is a consecutive sequence of sequence elements like nucleotides or amino acids
pub trait Sequence<T: SequenceElement> : Clone + Index<usize> + Index<Range<usize>> + From<Vec<T>> + Into<Vec<T>> + PartialEq + Eq + PartialOrd + Ord + Sized + fmt::Debug + Add {

    /// Constructs a new empty sequence
    fn new_empty() -> Self;

    /// Returns the length of the DNA sequence which is the number of nucleotides in it.
    fn length(&self) -> usize;

    /// Returns `true` if the sequence does not contain a single nucleotide.
    fn is_empty(&self) -> bool {
        self.length() == 0
    }

    fn iter(&self) -> slice::Iter<T>;

    fn as_vec(&self) -> Vec<T> {
        self.iter().map(|x| (*x).clone()).collect()
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
