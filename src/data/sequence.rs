use std::fmt::{Display,Formatter,Result,Debug};
use std::ops::Add;
use std::cmp::{Ord,Ordering};
use std::iter::FromIterator;

pub trait SequenceElement:  Ord + Eq + Clone + Debug + From<char> + Into<char> {}


#[derive(Clone,Debug)]
pub struct Sequence<T: SequenceElement> {
    seq: Vec<T>
}

/// A DNA sequence is a consecutive sequence of DNA nucleotides without further information.
impl<T: SequenceElement> Sequence<T> {

    /// Returns a new DNA sequences that does not comprise any nucleotide
    pub fn new_empty() -> Self {
        return Sequence { seq: Vec::new() };
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
    pub fn elements(&self) -> &Vec<T> {
        &self.seq
    }

    pub fn element(&self, idx: usize) -> Option<&T> {
        self.seq.get(idx)
    }

    pub fn subsequence(&self, from: usize, length: usize) -> Option<Self> {
        if from + length >= self.length() {
            None
        } else {
            Some( Sequence { seq: self.seq.iter().skip(from).take(length).map(|n| n.clone() ).collect() } )
        }
    }

    pub fn to_string(&self) -> String {
        self.seq.iter().map(|n| n.clone().into() ).collect()
    }
}

impl<T: SequenceElement> PartialEq for Sequence<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<T: SequenceElement> Eq for Sequence<T> {
}

impl<T: SequenceElement> From<Vec<T>> for Sequence<T> {

    fn from(s: Vec<T>) -> Sequence<T> {
        Sequence { seq: s }
    }
}

impl<'a, T: SequenceElement> From<&'a Vec<T>> for Sequence<T> {

    fn from(s: &'a Vec<T>) -> Sequence<T> {
        Sequence::from( s.clone() )
    }
}

impl<T: SequenceElement> Add for Sequence<T> {
    type Output = Sequence<T>;

    fn add(self, other: Sequence<T>) -> Sequence<T> {
        let mut elems = self.elements().clone();
        elems.append( &mut other.elements().clone() );
        return Sequence::from( elems ); 
    }
}

impl<T: SequenceElement> PartialOrd for Sequence<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.elements().partial_cmp( other.elements() )
    }
}

impl<T: SequenceElement> Ord for Sequence<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T: SequenceElement> Display for Sequence<T> {

    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}


//#[cfg(test)]
//mod tests {
//    
//    use data::nucleotide::DNANucleotide;
//    use data::dnasequence::DNASequence;
//
//    #[test]
//    fn test_from_vec(){
//        let seq_vec = vec![ DNANucleotide::A, DNANucleotide::C, DNANucleotide::G, DNANucleotide::T ];
//        let seq = DNASequence::from(seq_vec);
//
//        assert_eq!(seq.to_string(), "ACGT");
//        assert_eq!(seq.length(), 4);
//    }
//
//    #[test]
//    fn test_from_string(){
//        let seq = DNASequence::from("acgt");
//        assert_eq!(seq.to_string(), "ACGT");
//        assert_eq!(seq.length(), 4);
//    }
//
//    #[test]
//    fn test_subsequence(){
//        let seq = DNASequence::from("acgt");
//
//        assert_eq!(seq.subsequence(0,0), DNASequence::new_empty());
//        assert_eq!(seq.subsequence(0,1), DNASequence::from("A"));
//        assert_eq!(seq.subsequence(1,1), DNASequence::from("C"));
//        assert_eq!(seq.subsequence(1,2), DNASequence::from("CG"));
//        assert_eq!(seq.subsequence(3,1), DNASequence::from("T"));
//        assert_eq!(seq.subsequence(4,1), DNASequence::new_empty());
//    
//    }
//}
