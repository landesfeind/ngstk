use std::fmt;
use std::cmp::{Ord,Ordering};
use std::ops;
use std::slice;
use data::sequence::{SequenceElement,Sequence};
use data::dna::{DnaNucleotide,DnaSequence};

#[derive(Clone,Debug)]
pub enum RnaNucleotide {
    A, C, G, U, N
}

impl SequenceElement for RnaNucleotide {}

impl fmt::Display for RnaNucleotide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

impl PartialEq for RnaNucleotide {
    fn eq(&self, other: &Self) -> bool {
        (char::from(self)) == (char::from(other))
    }
}

impl Eq for RnaNucleotide { }

impl PartialOrd for RnaNucleotide {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some( (char::from(self).cmp(& char::from(other))) )
    }
}

impl Ord for RnaNucleotide {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

impl From<char> for RnaNucleotide {
    fn from(c: char) -> RnaNucleotide {
        match c {
            'a' => RnaNucleotide::A,
            'A' => RnaNucleotide::A,
            'c' => RnaNucleotide::C,
            'C' => RnaNucleotide::C,
            'g' => RnaNucleotide::G,
            'G' => RnaNucleotide::G,
            'u' => RnaNucleotide::U,
            'U' => RnaNucleotide::U,
            _   => RnaNucleotide::N
        }
    }
}

impl From<RnaNucleotide> for char {
    fn from(n: RnaNucleotide) -> char {
        match n {
            RnaNucleotide::A => 'A',
            RnaNucleotide::C => 'C',
            RnaNucleotide::G => 'G',
            RnaNucleotide::U => 'U',
            _ => 'N'
        }
    }
}

impl From<u8> for RnaNucleotide {
    fn from(n: u8) -> RnaNucleotide {
        match n {
            1 => RnaNucleotide::A,
            2 => RnaNucleotide::C,
            3 => RnaNucleotide::G,
            4 => RnaNucleotide::U,
            _ => RnaNucleotide::N
        }
    }
}

impl From<RnaNucleotide> for u8 {
    fn from(n: RnaNucleotide) -> u8 {
        match n {
            RnaNucleotide::A => 1,
            RnaNucleotide::C => 2,
            RnaNucleotide::G => 3,
            RnaNucleotide::U => 4,
            _ => 0
        }
    }
}

impl<'a> From<&'a RnaNucleotide> for u8 {
    fn from(n: &'a RnaNucleotide) -> u8 {
        match n.clone() {
            RnaNucleotide::A => 1,
            RnaNucleotide::C => 2,
            RnaNucleotide::G => 3,
            RnaNucleotide::U => 4,
            _ => 0
        }
    }
}

impl<'a> From<&'a RnaNucleotide> for char {
    fn from(n: &RnaNucleotide) -> char {
        match *n {
            RnaNucleotide::A => 'A',
            RnaNucleotide::C => 'C',
            RnaNucleotide::G => 'G',
            RnaNucleotide::U => 'U',
            _ => 'N'
        }
    }
}


impl From<DnaNucleotide> for RnaNucleotide {
    fn from(n: DnaNucleotide) -> RnaNucleotide {
        match n {
            DnaNucleotide::A => RnaNucleotide::U,
            DnaNucleotide::C => RnaNucleotide::G,
            DnaNucleotide::G => RnaNucleotide::C,
            DnaNucleotide::T => RnaNucleotide::A,
            DnaNucleotide::N => RnaNucleotide::N
        }
    }
}

impl<'a> From<&'a DnaNucleotide> for RnaNucleotide {
    fn from(n: &DnaNucleotide) -> RnaNucleotide {
        match *n {
            DnaNucleotide::A => RnaNucleotide::U,
            DnaNucleotide::C => RnaNucleotide::G,
            DnaNucleotide::G => RnaNucleotide::C,
            DnaNucleotide::T => RnaNucleotide::A,
            DnaNucleotide::N => RnaNucleotide::N
        }
    }
}


#[derive(Clone,Debug)]
pub struct RnaSequence {
    nucleotides: Vec<RnaNucleotide>
}

impl Sequence<RnaNucleotide> for RnaSequence {
    fn new_empty() -> RnaSequence {
        RnaSequence { nucleotides: Vec::new() }
    }
    fn length(&self) -> usize {
        self.nucleotides.len()
    }
    fn iter(&self) -> slice::Iter<RnaNucleotide> {
        self.nucleotides.iter()
    }
}

impl PartialOrd for RnaSequence {
    fn partial_cmp(&self, other: &RnaSequence) -> Option<Ordering> {
        self.nucleotides.partial_cmp( &Vec::from(other.clone()) )
    }
}
impl Ord for RnaSequence {
    fn cmp(&self, other: &RnaSequence) -> Ordering {
        self.partial_cmp( other ).unwrap()
    }
}
impl PartialEq for RnaSequence {
    fn eq(&self, other: &RnaSequence) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for RnaSequence { }
impl ops::Index<usize> for RnaSequence {
    type Output = RnaNucleotide;

    fn index(&self, i:usize) -> &RnaNucleotide {
        &self.nucleotides[i]
    }
}
impl ops::Index<ops::Range<usize>> for RnaSequence {
    type Output = [RnaNucleotide];

    fn index(&self, i: ops::Range<usize>) -> &[RnaNucleotide] {
        &self.nucleotides[i]
    }
}
impl ops::Add for RnaSequence {
    type Output = RnaSequence;
    fn add(self, other: RnaSequence) -> RnaSequence {
        let mut v = self.nucleotides.clone();
        v.extend(other.nucleotides);
        RnaSequence::from(v)
    }
}
impl<'a> ops::Add<&'a RnaSequence> for RnaSequence {
    type Output = RnaSequence;
    fn add(self, other: &RnaSequence) -> RnaSequence {
        let mut v = self.nucleotides.clone();
        v.extend(other.nucleotides.clone());
        RnaSequence::from(v)
    }
}



impl From<Vec<RnaNucleotide>> for RnaSequence {
    fn from(n: Vec<RnaNucleotide>) -> RnaSequence {
        RnaSequence { nucleotides: n }
    }
}

impl From<RnaSequence> for Vec<RnaNucleotide> {
    fn from(seq: RnaSequence) -> Vec<RnaNucleotide> {
        seq.nucleotides
    }
}

impl<'a> From<&'a DnaSequence> for RnaSequence {
    fn from(s: &DnaSequence) -> RnaSequence {
        let n : Vec<RnaNucleotide> = s.iter().map(|n| RnaNucleotide::from(n) ).collect();
        RnaSequence::from(n)
    }
}

impl fmt::Display for RnaSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s : String = self.iter().map(|n| char::from(n) ).collect();
        write!(f, "{}", s)
    }
}



#[cfg(test)]
mod tests {

    use data::rna::RnaNucleotide;
    
    #[test]
    fn test_a() {
        assert_eq!( RnaNucleotide::from('a'), RnaNucleotide::A);
        assert_eq!( RnaNucleotide::from('A'), RnaNucleotide::A);
        assert_eq!( char::from(RnaNucleotide::A), 'A');
    }
    
    #[test]
    fn test_c() {
        assert_eq!( RnaNucleotide::from('c'), RnaNucleotide::C);
        assert_eq!( RnaNucleotide::from('C'), RnaNucleotide::C);
        assert_eq!( char::from(RnaNucleotide::C), 'C');
    }

    #[test]
    fn test_g() {
        assert_eq!( RnaNucleotide::from('g'), RnaNucleotide::G);
        assert_eq!( RnaNucleotide::from('G'), RnaNucleotide::G);
        assert_eq!( char::from(RnaNucleotide::G), 'G');
    }

    #[test]
    fn test_t() {
        assert_eq!( RnaNucleotide::from('u'), RnaNucleotide::U);
        assert_eq!( RnaNucleotide::from('U'), RnaNucleotide::U);
        assert_eq!( char::from(RnaNucleotide::U), 'U');
    }

    #[test]
    fn test_n() {
        assert_eq!( RnaNucleotide::from('n'), RnaNucleotide::N);
        assert_eq!( RnaNucleotide::from('N'), RnaNucleotide::N);
        assert_eq!( char::from(RnaNucleotide::N), 'N');
    }

    #[test]
    fn test_others() {
        assert_eq!( RnaNucleotide::from('B'), RnaNucleotide::N);
        assert_eq!( RnaNucleotide::from('H'), RnaNucleotide::N);
    }
}

