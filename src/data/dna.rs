use std::cmp::{Ord,Ordering};
use data::sequence::{SequenceElement,Sequence};
use data::rna::RnaNucleotide;


#[derive(Clone,Debug)]
pub enum DnaNucleotide {
    A, C, G, T, N
}

impl PartialEq for DnaNucleotide {
    fn eq(&self, other: &Self) -> bool {
        return (char::from(self)) == (char::from(other));
    }
}

impl Eq for DnaNucleotide { }

impl PartialOrd for DnaNucleotide {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some( (char::from(self).cmp(& char::from(other))) );
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

impl SequenceElement for DnaNucleotide {}


pub trait DnaSequence : Sequence<DnaNucleotide> {
    
    fn transcribe(&self) -> Vec<RnaNucleotide> {
        self.as_vec().iter().map(|n| (*n).clone().into() ).collect()
    }
}


#[cfg(test)]
mod tests {

    use data::dna::DnaNucleotide;
    
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
}

