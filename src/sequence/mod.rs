use std::fmt;

pub mod dna;
pub mod rna;
pub mod aminoacid;

pub use self::dna::DnaNucleotide;
pub use self::dna::DnaSequence;
pub use self::rna::RnaNucleotide;
pub use self::rna::RnaSequence;
pub use self::aminoacid::Aminoacid;
pub use self::aminoacid::Peptide;


/// An element that can be part of a sequence.
pub trait SequenceElement
    : Ord + Eq + Clone + fmt::Debug + fmt::Display + Sized + From<char> + Into<char>
    {
}

/// A sequence is a consecutive sequence of sequence elements like nucleotides or amino acids
pub trait Sequence<E: SequenceElement>
    : Clone
    + fmt::Debug
    + fmt::Display {
    type SubsequenceType : Sequence<E>;

    /// Returns the length of the DNA sequence
    /// which is the number of nucleotides in it.
    fn length(&self) -> usize;

    /// Returns `true` if the sequence does
    /// not contain a single nucleotide.
    fn is_empty(&self) -> bool {
        self.length() == 0
    }

    /// Returns a copy of the sequence as vector
    /// of the sequence elements.
    fn vec(&self) -> Vec<E>;

    /// Extracts the subsequence with a given offset and length.
    fn subsequence(&self, offset: usize, length: usize) -> Self::SubsequenceType;

}
