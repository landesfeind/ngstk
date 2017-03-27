use std::fmt;
use std::ops::Index;
use std::ops::Range;
use std::cmp::Ord;
use std::slice;

/// An element that can be part of a sequence.
pub trait SequenceElement: Ord + Eq + Clone + fmt::Debug + fmt::Display + Sized {}

/// A sequence is a consecutive sequence of sequence elements like nucleotides or amino acids
pub trait Sequence<E: SequenceElement> : Clone
    + Index<usize>
    + Index<Range<usize>>
    + From<Vec<E>>
    + Into<Vec<E>>
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Sized
    + fmt::Debug {

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
    fn iter(&self) -> slice::Iter<E>;

    /// Converts the sequence by cloning the
    /// the elements and collecting into a vector.
    fn as_vec(&self) -> Vec<E> {
        self.iter().map(|n| (*n).clone()).collect()
    }

    fn subsequence(&self, offset: usize, length: usize) -> Self {
        let subs : Vec<E> = self.iter().skip(offset).take(length).map( |x| (*x).clone() ).collect();
        Self::from(subs)
    }

    fn reverse(&self) -> Self {
        let subs : Vec<E> = self.iter().rev().map( |x| (*x).clone() ).collect();
        Self::from(subs)
    }
}


