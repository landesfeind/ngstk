use std::cmp::{Ord,Ordering};

#[derive(Clone,Debug)]
pub enum DNANucleotide {
    A, C, G, T, N
}

impl PartialEq for DNANucleotide {
    fn eq(&self, other: &Self) -> bool {
        return (char::from(self)) == (char::from(other));
    }
}

impl Eq for DNANucleotide { }

impl PartialOrd for DNANucleotide {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some( (char::from(self).cmp(& char::from(other))) );
    }
}

impl Ord for DNANucleotide {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.partial_cmp(other).unwrap();
    }
}


impl From<char> for DNANucleotide {
    fn from(c: char) -> DNANucleotide {
        match c {
            'a' => DNANucleotide::A,
            'A' => DNANucleotide::A,
            'c' => DNANucleotide::C,
            'C' => DNANucleotide::C,
            'g' => DNANucleotide::G,
            'G' => DNANucleotide::G,
            't' => DNANucleotide::T,
            'T' => DNANucleotide::T,
            _   => DNANucleotide::N
        }
    }
}

impl From<DNANucleotide> for char {
    fn from(n: DNANucleotide) -> char {
        match n {
            DNANucleotide::A => 'A',
            DNANucleotide::C => 'C',
            DNANucleotide::G => 'G',
            DNANucleotide::T => 'T',
            _ => 'N'
        }
    }
}

impl From<u8> for DNANucleotide {
    fn from(n: u8) -> DNANucleotide {
        match n {
            1 => DNANucleotide::A,
            2 => DNANucleotide::C,
            3 => DNANucleotide::G,
            4 => DNANucleotide::T,
            _ => DNANucleotide::N
        }
    }
}

impl From<DNANucleotide> for u8 {
    fn from(n: DNANucleotide) -> u8 {
        match n {
            DNANucleotide::A => 1,
            DNANucleotide::C => 2,
            DNANucleotide::G => 3,
            DNANucleotide::T => 4,
            _ => 0
        }
    }
}

impl<'a> From<&'a DNANucleotide> for char {
    fn from(n: &DNANucleotide) -> char {
        match *n {
            DNANucleotide::A => 'A',
            DNANucleotide::C => 'C',
            DNANucleotide::G => 'G',
            DNANucleotide::T => 'T',
            _ => 'N'
        }
    }
}


#[cfg(test)]
mod tests {

    use nucleotide::DNANucleotide;
    
    #[test]
    fn test_a() {
        assert_eq!( DNANucleotide::from('a'), DNANucleotide::A);
        assert_eq!( DNANucleotide::from('A'), DNANucleotide::A);
        assert_eq!( char::from(DNANucleotide::A), 'A');
    }
    
    #[test]
    fn test_c() {
        assert_eq!( DNANucleotide::from('c'), DNANucleotide::C);
        assert_eq!( DNANucleotide::from('C'), DNANucleotide::C);
        assert_eq!( char::from(DNANucleotide::C), 'C');
    }

    #[test]
    fn test_g() {
        assert_eq!( DNANucleotide::from('g'), DNANucleotide::G);
        assert_eq!( DNANucleotide::from('G'), DNANucleotide::G);
        assert_eq!( char::from(DNANucleotide::G), 'G');
    }

    #[test]
    fn test_t() {
        assert_eq!( DNANucleotide::from('t'), DNANucleotide::T);
        assert_eq!( DNANucleotide::from('T'), DNANucleotide::T);
        assert_eq!( char::from(DNANucleotide::T), 'T');
    }

    #[test]
    fn test_n() {
        assert_eq!( DNANucleotide::from('n'), DNANucleotide::N);
        assert_eq!( DNANucleotide::from('N'), DNANucleotide::N);
        assert_eq!( char::from(DNANucleotide::N), 'N');
    }

    #[test]
    fn test_others() {
        assert_eq!( DNANucleotide::from('u'), DNANucleotide::N);
        assert_eq!( DNANucleotide::from('U'), DNANucleotide::N);
        assert_eq!( DNANucleotide::from('B'), DNANucleotide::N);
        assert_eq!( DNANucleotide::from('H'), DNANucleotide::N);
    }
}




