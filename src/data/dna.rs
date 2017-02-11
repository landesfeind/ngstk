use std::cmp::{Ord,Ordering};
use std::ops;
use std::fmt;
use std::slice;
use std::iter;
use data::sequence::{SequenceElement,Sequence};
use data::rna::RnaNucleotide;


#[derive(Clone,Debug)]
pub enum DnaNucleotide {
    A, C, G, T, N
}

impl DnaNucleotide {
    pub fn complement(&self) -> DnaNucleotide {
        match *self {
            DnaNucleotide::A => DnaNucleotide::T,
            DnaNucleotide::C => DnaNucleotide::G,
            DnaNucleotide::G => DnaNucleotide::C,
            DnaNucleotide::T => DnaNucleotide::A,
            _ => DnaNucleotide::N
        }
    }
}

impl SequenceElement for DnaNucleotide {}

impl fmt::Display for DnaNucleotide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

impl PartialEq for DnaNucleotide {
    fn eq(&self, other: &Self) -> bool {
        (char::from(self)) == (char::from(other))
    }
}

impl Eq for DnaNucleotide { }

impl PartialOrd for DnaNucleotide {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some( (char::from(self).cmp(& char::from(other))) )
    }
}

impl Ord for DnaNucleotide {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

impl From<char> for DnaNucleotide {
    fn from(c: char) -> DnaNucleotide {
        match c {
            'a' => DnaNucleotide::A,
            'A' => DnaNucleotide::A,
            'c' => DnaNucleotide::C,
            'C' => DnaNucleotide::C,
            'g' => DnaNucleotide::G,
            'G' => DnaNucleotide::G,
            't' => DnaNucleotide::T,
            'T' => DnaNucleotide::T,
            _   => DnaNucleotide::N
        }
    }
}

impl From<DnaNucleotide> for char {
    fn from(n: DnaNucleotide) -> char {
        match n {
            DnaNucleotide::A => 'A',
            DnaNucleotide::C => 'C',
            DnaNucleotide::G => 'G',
            DnaNucleotide::T => 'T',
            _ => 'N'
        }
    }
}

impl From<u8> for DnaNucleotide {
    fn from(n: u8) -> DnaNucleotide {
        match n {
            1 => DnaNucleotide::A,
            2 => DnaNucleotide::C,
            3 => DnaNucleotide::G,
            4 => DnaNucleotide::T,
            _ => DnaNucleotide::N
        }
    }
}

impl From<DnaNucleotide> for u8 {
    fn from(n: DnaNucleotide) -> u8 {
        match n {
            DnaNucleotide::A => 1,
            DnaNucleotide::C => 2,
            DnaNucleotide::G => 3,
            DnaNucleotide::T => 4,
            _ => 0
        }
    }
}

impl<'a> From<&'a DnaNucleotide> for u8 {
    fn from(n: &'a DnaNucleotide) -> u8 {
        match n.clone() {
            DnaNucleotide::A => 1,
            DnaNucleotide::C => 2,
            DnaNucleotide::G => 3,
            DnaNucleotide::T => 4,
            _ => 0
        }
    }
}

impl<'a> From<&'a DnaNucleotide> for char {
    fn from(n: &DnaNucleotide) -> char {
        match *n {
            DnaNucleotide::A => 'A',
            DnaNucleotide::C => 'C',
            DnaNucleotide::G => 'G',
            DnaNucleotide::T => 'T',
            _ => 'N'
        }
    }
}


#[derive(Clone,Debug)]
pub struct DnaCodon (pub DnaNucleotide, pub DnaNucleotide, pub DnaNucleotide);


impl From<(DnaNucleotide, DnaNucleotide, DnaNucleotide)> for DnaCodon {
    fn from(c: (DnaNucleotide, DnaNucleotide, DnaNucleotide)) -> DnaCodon {
        DnaCodon(c.0, c.1,c.2)
    }
}


#[derive(Clone,Debug)]
pub struct DnaSequence {
    nucleotides: Vec<DnaNucleotide>
}

impl DnaSequence {

    /// Returns the reverse strand sequence
    pub fn reverse_strand(&self) -> DnaSequence {
        let r : Vec<DnaNucleotide> = self.iter().map( |n| n.complement() ).collect();
        DnaSequence::from( r )
    }

    /// Returns the reverse strand sequence in forward direction
    pub fn complement(&self) -> DnaSequence {
        let mut r : Vec<DnaNucleotide> = self.iter().map( |n| n.complement() ).collect();
        r.reverse();
        DnaSequence::from( r )
    }

    /// Returns an iterator on the codons. This is identical
    /// to `frame(0)`.
    pub fn codons(&self) -> Vec<DnaCodon> {
        self.frame(0usize)
    }

    /// Generates the codons that represents the frame starting
    /// at `offset`. If the sequence is not a multiple of 3, the 
    /// last codon will be filled with `DnaNucleotide::N`.
    pub fn frame(&self, offset: usize) -> Vec<DnaCodon> {
        self.nucleotides[offset .. self.nucleotides.len()].chunks(3usize).map(|e| match e.len() {
            1 => DnaCodon(e[0].clone(), DnaNucleotide::N, DnaNucleotide::N),
            2 => DnaCodon(e[0].clone(), e[1].clone(), DnaNucleotide::N),
            3 => DnaCodon(e[0].clone(), e[1].clone(), e[2].clone()),
            _ => DnaCodon(DnaNucleotide::N, DnaNucleotide::N, DnaNucleotide::N),
        }).collect()
    }
}

impl Sequence<DnaNucleotide> for DnaSequence {
    fn new_empty() -> DnaSequence {
        DnaSequence { nucleotides: Vec::new() }
    }
    fn length(&self) -> usize {
        self.nucleotides.len()
    }

    fn iter(&self) -> slice::Iter<DnaNucleotide> {
        self.nucleotides.iter()
    }
}

impl PartialOrd for DnaSequence {
    fn partial_cmp(&self, other: &DnaSequence) -> Option<Ordering> {
        self.nucleotides.partial_cmp( &other.nucleotides )
    }
}
impl Ord for DnaSequence {
    fn cmp(&self, other: &DnaSequence) -> Ordering {
        self.partial_cmp( other ).unwrap()
    }
}
impl PartialEq for DnaSequence {
    fn eq(&self, other: &DnaSequence) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for DnaSequence { }
impl ops::Index<usize> for DnaSequence {
    type Output = DnaNucleotide;

    fn index(&self, i:usize) -> &DnaNucleotide {
        &self.nucleotides[i]
    }
}
impl ops::Index<ops::Range<usize>> for DnaSequence {
    type Output = [DnaNucleotide];

    fn index(&self, i: ops::Range<usize>) -> &[DnaNucleotide] {
        &self.nucleotides[i]
    }
}
impl From<Vec<DnaNucleotide>> for DnaSequence {
    fn from(n: Vec<DnaNucleotide>) -> DnaSequence {
        DnaSequence { nucleotides: n }
    }
}

impl From<DnaSequence> for Vec<DnaNucleotide> {
    fn from(seq: DnaSequence) -> Vec<DnaNucleotide> {
        seq.nucleotides
    }
}

impl<'a> From<&'a str> for DnaSequence {
    fn from(s: &str) -> DnaSequence {
        let v : Vec<DnaNucleotide> = s.chars().map(|c| DnaNucleotide::from(c)).collect();
        DnaSequence::from(v)
    }
}

impl fmt::Display for DnaSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s : String = self.iter().map(|n| char::from(n) ).collect();
        write!(f, "{}", s)
    }
}


#[cfg(test)]
mod tests {
    use data::dna::DnaNucleotide;
    use data::dna::DnaSequence;
    
    #[test]
    fn test_a() {
        assert_eq!( DnaNucleotide::from('a'), DnaNucleotide::A);
        assert_eq!( DnaNucleotide::from('A'), DnaNucleotide::A);
        assert_eq!( char::from(DnaNucleotide::A), 'A');
    }
    
    #[test]
    fn test_c() {
        assert_eq!( DnaNucleotide::from('c'), DnaNucleotide::C);
        assert_eq!( DnaNucleotide::from('C'), DnaNucleotide::C);
        assert_eq!( char::from(DnaNucleotide::C), 'C');
    }

    #[test]
    fn test_g() {
        assert_eq!( DnaNucleotide::from('g'), DnaNucleotide::G);
        assert_eq!( DnaNucleotide::from('G'), DnaNucleotide::G);
        assert_eq!( char::from(DnaNucleotide::G), 'G');
     }

    #[test]
    fn test_t() {
        assert_eq!( DnaNucleotide::from('t'), DnaNucleotide::T);
        assert_eq!( DnaNucleotide::from('T'), DnaNucleotide::T);
        assert_eq!( char::from(DnaNucleotide::T), 'T');
    }

    #[test]
    fn test_n() {
        assert_eq!( DnaNucleotide::from('n'), DnaNucleotide::N);
        assert_eq!( DnaNucleotide::from('N'), DnaNucleotide::N);
        assert_eq!( char::from(DnaNucleotide::N), 'N');
    }

    #[test]
    fn test_others() {
        assert_eq!( DnaNucleotide::from('u'), DnaNucleotide::N);
        assert_eq!( DnaNucleotide::from('U'), DnaNucleotide::N);
        assert_eq!( DnaNucleotide::from('B'), DnaNucleotide::N);
        assert_eq!( DnaNucleotide::from('H'), DnaNucleotide::N);
    }


    #[test]
    fn test_dnsequence_to_string(){
        let seq : DnaSequence =  DnaSequence::from("AGTACGGCAAGT");
        assert_eq!(seq.to_string(), "AGTACGGCAAGT");
    }
    #[test]
    fn test_dnsequence_to_reverse_strand(){
        let seq : DnaSequence =  DnaSequence::from("AGTACGGCAAGT");
        assert_eq!(seq.reverse_strand().to_string(), "TCATGCCGTTCA");
    }
    #[test]
    fn test_dnsequence_to_complement(){
        let seq : DnaSequence =  DnaSequence::from("AGTACGGCAAGT");
        assert_eq!(seq.complement().to_string(), "ACTTGCCGTACT");
    }

}

