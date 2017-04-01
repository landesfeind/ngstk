use std::fmt;
use std::cmp::{Ord, Ordering};
use std::ops;
use std::slice;
use sequence::{SequenceElement, Sequence};
use dna::{DnaNucleotide, DnaSequence};

#[derive(Clone,Debug)]
pub enum RnaNucleotide {
    A,
    C,
    G,
    U,
    N,
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

impl Eq for RnaNucleotide {}

impl PartialOrd for RnaNucleotide {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((char::from(self).cmp(&char::from(other))))
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
            _ => RnaNucleotide::N,
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
            _ => 'N',
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
            _ => RnaNucleotide::N,
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
            _ => 0,
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
            _ => 0,
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
            _ => 'N',
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
            DnaNucleotide::N => RnaNucleotide::N,
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
            DnaNucleotide::N => RnaNucleotide::N,
        }
    }
}


pub trait RnaSequence : Sequence<RnaNucleotide> {}

impl RnaSequence for Vec<RnaNucleotide> {}

#[cfg(test)]
mod tests {

    use rna::RnaNucleotide;

    #[test]
    fn test_a() {
        assert_eq!(RnaNucleotide::from('a'), RnaNucleotide::A);
        assert_eq!(RnaNucleotide::from('A'), RnaNucleotide::A);
        assert_eq!(char::from(RnaNucleotide::A), 'A');
    }

    #[test]
    fn test_c() {
        assert_eq!(RnaNucleotide::from('c'), RnaNucleotide::C);
        assert_eq!(RnaNucleotide::from('C'), RnaNucleotide::C);
        assert_eq!(char::from(RnaNucleotide::C), 'C');
    }

    #[test]
    fn test_g() {
        assert_eq!(RnaNucleotide::from('g'), RnaNucleotide::G);
        assert_eq!(RnaNucleotide::from('G'), RnaNucleotide::G);
        assert_eq!(char::from(RnaNucleotide::G), 'G');
    }

    #[test]
    fn test_t() {
        assert_eq!(RnaNucleotide::from('u'), RnaNucleotide::U);
        assert_eq!(RnaNucleotide::from('U'), RnaNucleotide::U);
        assert_eq!(char::from(RnaNucleotide::U), 'U');
    }

    #[test]
    fn test_n() {
        assert_eq!(RnaNucleotide::from('n'), RnaNucleotide::N);
        assert_eq!(RnaNucleotide::from('N'), RnaNucleotide::N);
        assert_eq!(char::from(RnaNucleotide::N), 'N');
    }

    #[test]
    fn test_others() {
        assert_eq!(RnaNucleotide::from('B'), RnaNucleotide::N);
        assert_eq!(RnaNucleotide::from('H'), RnaNucleotide::N);
    }
}
