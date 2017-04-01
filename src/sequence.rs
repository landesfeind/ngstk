use std::fmt;
use std::ops::Index;
use std::ops::Range;
use std::cmp::Ord;
use std::slice;

/// An element that can be part of a sequence.
pub trait SequenceElement: Ord + Eq + Clone + fmt::Debug + fmt::Display + Sized + From<char> {}

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
    + Default
    + fmt::Debug {

    fn from_string(seq: &str) -> Self;

    fn to_string(&self) -> String {
        self.iterator().map(|n| format!("{}", n)).collect()
    }

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

    /// Converts the sequence by cloning the
    /// the elements and collecting into a vector.
    fn as_vec(&self) -> Vec<E> {
        self.iterator().map(|n| (*n).clone()).collect()
    }

    fn subsequence(&self, offset: usize, length: usize) -> Self {
        let subs : Vec<E> = self.iterator().skip(offset).take(length).map( |x| (*x).clone() ).collect();
        Self::from(subs)
    }

    fn reverse(&self) -> Self {
        let subs : Vec<E> = self.iterator().rev().map( |x| (*x).clone() ).collect();
        Self::from(subs)
    }
}

impl<E : SequenceElement> Sequence<E> for Vec<E> {

    fn from_string(s: &str) -> Self {
        s.chars().map( |x| E::from(x) ).collect()
    }

    fn length(&self) -> usize {
        self.len()
    }

    fn iterator(&self) -> slice::Iter<E> {
        self.iter()
    }
}
