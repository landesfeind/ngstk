use std::fmt;
use std::ops;
use std::cmp;
use std::slice;

pub mod dna;
pub mod rna;
pub mod aminoacid;

/// An element that can be part of a sequence.
pub trait SequenceElement: Ord + Eq + Clone + fmt::Debug + fmt::Display + Sized + From<char> + Into<char> {}

/// A sequence is a consecutive sequence of sequence elements like nucleotides or amino acids
pub trait Sequence<E: SequenceElement> : Clone
    + cmp::PartialEq
    + cmp::Eq
    + cmp::PartialOrd
    + cmp::Ord
    + From<Vec<E>>
    + fmt::Debug 
    + fmt::Display {

    /// Returns the length of the DNA sequence
    /// which is the number of nucleotides in it.
    fn length(&self) -> usize;

    /// Returns `true` if the sequence does
    /// not contain a single nucleotide.
    fn is_empty(&self) -> bool {
        self.length() == 0
    }

    /// Generates an iterator running over
    /// the elements of the sequence
    fn iterator(&self) -> slice::Iter<E>;

    /// Returns a (cloned) vector of the sequence elements.
    fn as_vec(&self) -> Vec<E> {
        self.iterator().cloned().collect()
    }

    /// Extracts the subsequence with a given offset and length. The 
    /// offset is given relative to the start if this sequence! 
    ///
    /// *Important*: Ensure the offset are determined correctly!
    fn subsequence(&self, offset: usize, length: usize) -> Self {
        assert!(offset + length <=  length);
        let v : Vec<E> = self.iterator().skip(offset).take(length).cloned().collect();
        Self::from(v)
    }

    /// 
    fn reverse(&self) -> Self {
        let v : Vec<E> = self.iterator().rev().cloned().collect();
        Self::from(v)
    }
}

//use std::marker::PhantomData;
//#[derive(Clone,Debug)]
//pub struct SequenceSlice<E : SequenceElement, S: Sequence<E>> {
//    refseq: S,
//    offset: usize,
//    length: usize,
//    _marker_E: PhantomData<E>
//}
//
//impl<E : SequenceElement, S: Sequence<E>> SequenceSlice<E,S> {
//
//    /// Equals the default offset of `Sequence` but is included for clarity
//    fn offset_absolute(&self) -> usize {
//        self.offset()
//    }
//
//    /// The number of bases that are between the offset of the
//    /// reference sequence and the offset of this slice.
//    fn offset_relative(&self) -> usize {
//        self.offset - self.refseq.offset()
//    }
//}
//
//impl<E : SequenceElement, S: Sequence<E>> Sequence<E> for SequenceSlice<E,S> {
//
//    fn name(&self) -> Option<&str> { self.refseq.name() }
//    fn offset(&self) -> usize { self.offset }
//    fn length(&self) -> usize { self.length }
//
//    fn iterator(&self) -> slice::Iter<E> {
//        let v : Vec<E> = self.refseq.iterator().skip( self.offset_relative() ).cloned().collect();
//        v.iter()
//    }
//
//    fn subsequence(&self, offset: usize, length: usize) -> Self {
//        SequenceSlice {
//            refseq: self.refseq.clone(),
//            offset: self.offset + offset,
//            length: length,
//            _marker_E: PhantomData
//        }
//    }
//}
//
//impl<E : SequenceElement, S: Sequence<E>> PartialEq for SequenceSlice<E,S> {
//    fn eq(&self, other: &SequenceSlice<E,S>) -> bool {
//        self.as_vec().eq(&other.as_vec())
//    }
//}
//impl<E : SequenceElement, S: Sequence<E>> Eq for SequenceSlice<E,S> {}
//
//impl<E : SequenceElement, S: Sequence<E>> PartialOrd for SequenceSlice<E,S>  {
//    fn partial_cmp(&self, other: &SequenceSlice<E,S>) -> Option<cmp::Ordering> {
//        self.as_vec().partial_cmp( &other.as_vec() )
//    }
//}
//impl<E : SequenceElement, S: Sequence<E>> Ord for SequenceSlice<E,S>  {
//    fn cmp(&self, other: &SequenceSlice<E,S>) -> cmp::Ordering {
//        self.as_vec().cmp( &other.as_vec() )
//    }
//}
//impl<E : SequenceElement, S: Sequence<E>> fmt::Display for SequenceSlice<E,S> {
//    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
//        let s : String = self.iterator().map(|n| format!("{}", n) ).collect();
//        write!(f, "{}", s)
//    }
//}




