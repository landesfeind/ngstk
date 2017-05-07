use std::fmt;
use std::ops;
use std::cmp;
use std::slice;

pub mod dna;
pub mod rna;
pub mod aminoacid;

/// An element that can be part of a sequence.
pub trait SequenceElement: Ord + Eq + Clone + fmt::Debug + fmt::Display + Sized + From<char> {}

/// A sequence is a consecutive sequence of sequence elements like nucleotides or amino acids
pub trait Sequence<E: SequenceElement> : Clone
    + From<Vec<E>>
    + Into<Vec<E>>
    + cmp::PartialEq
    + cmp::Eq
    + cmp::PartialOrd
    + cmp::Ord
    + Sized
    + Default
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

    fn as_vec(&self) -> Vec<E> {
        self.iterator().map(|e| e.clone()).collect()
    }

    /// Extracts the subsequence with a given offset and length
    fn subsequence(&self, offset: usize, length: usize) -> Self {
        let subs : Vec<E> = self.iterator().skip(offset).take(length).map( |x| (*x).clone() ).collect();
        Self::from(subs)
    }

    /// Returns the sequence in reverse order
    fn reverse(&self) -> Self {
        let subs : Vec<E> = self.iterator().rev().map( |x| (*x).clone() ).collect();
        Self::from(subs)
    }
}
