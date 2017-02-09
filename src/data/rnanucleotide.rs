use std::cmp::{Ord,Ordering};

#[derive(Clone,Debug)]
pub enum RNANucleotide {
    A, C, G, U, N
}

impl PartialEq for RNANucleotide {
    fn eq(&self, other: &Self) -> bool {
        return (char::from(self)) == (char::from(other));
    }
}

impl Eq for RNANucleotide { }

impl PartialOrd for RNANucleotide {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some( (char::from(self).cmp(& char::from(other))) );
    }
}

impl Ord for RNANucleotide {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

impl From<char> for RNANucleotide {
    fn from(c: char) -> RNANucleotide {
        match c {
            'a' => RNANucleotide::A,
            'A' => RNANucleotide::A,
            'c' => RNANucleotide::C,
            'C' => RNANucleotide::C,
            'g' => RNANucleotide::G,
            'G' => RNANucleotide::G,
            'u' => RNANucleotide::U,
            'U' => RNANucleotide::U,
            _   => RNANucleotide::N
        }
    }
}

impl From<RNANucleotide> for char {
    fn from(n: RNANucleotide) -> char {
        match n {
            RNANucleotide::A => 'A',
            RNANucleotide::C => 'C',
            RNANucleotide::G => 'G',
            RNANucleotide::U => 'U',
            _ => 'N'
        }
    }
}

impl From<u8> for RNANucleotide {
    fn from(n: u8) -> RNANucleotide {
        match n {
            1 => RNANucleotide::A,
            2 => RNANucleotide::C,
            3 => RNANucleotide::G,
            4 => RNANucleotide::U,
            _ => RNANucleotide::N
        }
    }
}

impl From<RNANucleotide> for u8 {
    fn from(n: RNANucleotide) -> u8 {
        match n {
            RNANucleotide::A => 1,
            RNANucleotide::C => 2,
            RNANucleotide::G => 3,
            RNANucleotide::U => 4,
            _ => 0
        }
    }
}

impl<'a> From<&'a RNANucleotide> for u8 {
    fn from(n: &'a RNANucleotide) -> u8 {
        match n.clone() {
            RNANucleotide::A => 1,
            RNANucleotide::C => 2,
            RNANucleotide::G => 3,
            RNANucleotide::U => 4,
            _ => 0
        }
    }
}

impl<'a> From<&'a RNANucleotide> for char {
    fn from(n: &RNANucleotide) -> char {
        match *n {
            RNANucleotide::A => 'A',
            RNANucleotide::C => 'C',
            RNANucleotide::G => 'G',
            RNANucleotide::U => 'U',
            _ => 'N'
        }
    }
}

#[cfg(test)]
mod tests {

    use data::rnanucleotide::RNANucleotide;
    
    #[test]
    fn test_a() {
        assert_eq!( RNANucleotide::from('a'), RNANucleotide::A);
        assert_eq!( RNANucleotide::from('A'), RNANucleotide::A);
        assert_eq!( char::from(RNANucleotide::A), 'A');
    }
    
    #[test]
    fn test_c() {
        assert_eq!( RNANucleotide::from('c'), RNANucleotide::C);
        assert_eq!( RNANucleotide::from('C'), RNANucleotide::C);
        assert_eq!( char::from(RNANucleotide::C), 'C');
    }

    #[test]
    fn test_g() {
        assert_eq!( RNANucleotide::from('g'), RNANucleotide::G);
        assert_eq!( RNANucleotide::from('G'), RNANucleotide::G);
        assert_eq!( char::from(RNANucleotide::G), 'G');
    }

    #[test]
    fn test_t() {
        assert_eq!( RNANucleotide::from('u'), RNANucleotide::U);
        assert_eq!( RNANucleotide::from('U'), RNANucleotide::U);
        assert_eq!( char::from(RNANucleotide::U), 'U');
    }

    #[test]
    fn test_n() {
        assert_eq!( RNANucleotide::from('n'), RNANucleotide::N);
        assert_eq!( RNANucleotide::from('N'), RNANucleotide::N);
        assert_eq!( char::from(RNANucleotide::N), 'N');
    }

    #[test]
    fn test_others() {
        assert_eq!( RNANucleotide::from('B'), RNANucleotide::N);
        assert_eq!( RNANucleotide::from('H'), RNANucleotide::N);
    }
}

