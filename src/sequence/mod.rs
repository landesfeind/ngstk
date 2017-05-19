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

    /// Returns an iterator of over the sequence.
    fn iterator(&self) -> slice::Iter<E>;

    /// Returns a copy of the sequence as vector
    /// of the sequence elements.
    fn as_vec(&self) -> Vec<E> {
        self.iterator().cloned().collect()
    }

    /// Extracts the subsequence with a given offset and length.  
    fn subsequence(&self, offset: usize, length: usize) -> Self {
        assert!(offset + length <= self.length(), "The requested range [{} .. {}] is out of range for a sequence of length {}", offset, offset+length, self.length());
        let v : Vec<E> = self.iterator().skip(offset).take(length).cloned().collect();
        Self::from(v)
    }

    /// Returns the a copy of the reversed sequence.
    fn reverse(&self) -> Self {
        let v : Vec<E> = self.iterator().rev().cloned().collect();
        Self::from(v)
    }
}

