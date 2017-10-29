

pub use sequence::{SequenceElement, Sequence};
use std::cmp::{Ord, Ordering};
use std::fmt;
pub use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum DnaNucleotide {
    A,
    C,
    G,
    T,
    N,
}

impl DnaNucleotide {
    pub fn complement(&self) -> DnaNucleotide {
        match *self {
            DnaNucleotide::A => DnaNucleotide::T,
            DnaNucleotide::C => DnaNucleotide::G,
            DnaNucleotide::G => DnaNucleotide::C,
            DnaNucleotide::T => DnaNucleotide::A,
            _ => DnaNucleotide::N,
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

impl Eq for DnaNucleotide {}

impl PartialOrd for DnaNucleotide {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((char::from(self).cmp(&char::from(other))))
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
            _ => DnaNucleotide::N,
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
            _ => 'N',
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
            _ => DnaNucleotide::N,
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
            _ => 0,
        }
    }
}

impl<'a> From<&'a DnaNucleotide> for u8 {
    fn from(n: &'a DnaNucleotide) -> u8 {
        match *n {
            DnaNucleotide::A => 1,
            DnaNucleotide::C => 2,
            DnaNucleotide::G => 3,
            DnaNucleotide::T => 4,
            _ => 0,
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
            _ => 'N',
        }
    }
}


#[derive(Clone, Debug)]
pub struct DnaCodon(pub DnaNucleotide, pub DnaNucleotide, pub DnaNucleotide);

impl From<(DnaNucleotide, DnaNucleotide, DnaNucleotide)> for DnaCodon {
    fn from(c: (DnaNucleotide, DnaNucleotide, DnaNucleotide)) -> DnaCodon {
        DnaCodon(c.0, c.1, c.2)
    }
}

impl<'a> From<&'a [DnaNucleotide]> for DnaCodon {
    fn from(e: &[DnaNucleotide]) -> DnaCodon {
        match e.len() {
            0 => DnaCodon(DnaNucleotide::N, DnaNucleotide::N, DnaNucleotide::N),
            1 => DnaCodon(e[0].clone(), DnaNucleotide::N, DnaNucleotide::N),
            2 => DnaCodon(e[0].clone(), e[1].clone(), DnaNucleotide::N),
            _ => DnaCodon(e[0].clone(), e[1].clone(), e[2].clone()),
        }
    }
}

impl<'a> From<&'a Vec<DnaNucleotide>> for DnaCodon {
    fn from(e: &'a Vec<DnaNucleotide>) -> DnaCodon {
        match e.len() {
            0 => DnaCodon(DnaNucleotide::N, DnaNucleotide::N, DnaNucleotide::N),
            1 => DnaCodon(e[0].clone(), DnaNucleotide::N, DnaNucleotide::N),
            2 => DnaCodon(e[0].clone(), e[1].clone(), DnaNucleotide::N),
            _ => DnaCodon(e[0].clone(), e[1].clone(), e[2].clone()),
        }
    }
}


#[derive(Clone, Debug)]
pub struct DnaSequence {
    elements: Vec<DnaNucleotide>,
}

impl Sequence<DnaNucleotide> for DnaSequence {
    type SubsequenceType = DnaSequence;
    
    fn length(&self) -> usize {
        self.elements.len()
    }

    fn vec(&self) -> Vec<DnaNucleotide> {
        self.elements.clone()
    }

    fn subsequence(&self, offset:usize, length: usize) -> DnaSequence {
        let v : Vec<DnaNucleotide> = self.elements.iter().skip(offset).take(length).cloned().collect();
        DnaSequence {
            elements: v
        }
    }
}

impl DnaSequence {
    /// Returns the complementary strand sequence in reversed direction (i.e., the actual sequence
    /// that is read by DNA or RNA polymerase).
    pub fn reverse_strand(&self) -> Self {
        let r: Vec<DnaNucleotide> = self.vec().iter().map(|n| n.complement()).rev().collect();
        Self::from(r)
    }

    /// Returns the complementary strand sequence in forward direction.
    pub fn complement(&self) -> Self {
        let r: Vec<DnaNucleotide> = self.vec().iter().map(|n| n.complement()).collect();
        Self::from(r)
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
        let v: Vec<DnaNucleotide> = self.vec().iter().skip(offset).map(|n| n.clone()).collect();
        v.chunks(3usize).map(|c| DnaCodon::from(c)).collect()
    }
}

impl PartialEq for DnaSequence {
    fn eq(&self, other: &DnaSequence) -> bool {
        self.elements.eq(&other.elements)
    }
}
impl Eq for DnaSequence {}

impl PartialOrd for DnaSequence {
    fn partial_cmp(&self, other: &DnaSequence) -> Option<Ordering> {
        self.elements.partial_cmp(&other.elements)
    }
}
impl Ord for DnaSequence {
    fn cmp(&self, other: &DnaSequence) -> Ordering {
        self.elements.cmp(&other.elements)
    }
}
impl Default for DnaSequence {
    fn default() -> DnaSequence {
        DnaSequence::from(Vec::new())
    }
}
impl FromStr for DnaSequence {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<DnaNucleotide> = s.chars()
            .filter(|n| !n.is_whitespace())
            .map(|n| DnaNucleotide::from(n))
            .collect();
        Ok(DnaSequence::from(v))
    }
}

impl From<Vec<DnaNucleotide>> for DnaSequence {
    fn from(v: Vec<DnaNucleotide>) -> DnaSequence {
        DnaSequence { elements: v }
    }
}
impl fmt::Display for DnaSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let s: String = self.vec().iter().map(|n| char::from(n)).collect();
        write!(f, "{}", s)
    }
}
impl From<DnaSequence> for Vec<DnaNucleotide> {
    fn from(seq: DnaSequence) -> Vec<DnaNucleotide> {
        seq.vec().iter().map(|n| n.clone()).collect()
    }
}
impl<'a> From<&'a DnaSequence> for Vec<DnaNucleotide> {
    fn from(seq: &'a DnaSequence) -> Vec<DnaNucleotide> {
        seq.vec().iter().map(|n| n.clone()).collect()
    }
}


#[cfg(test)]
mod tests {
    use self::dna::DnaNucleotide;
    use self::dna::DnaSequence;
    use sequence::*;
    use std::str::FromStr;

    #[test]
    fn test_a() {
        assert_eq!(DnaNucleotide::from('a'), DnaNucleotide::A);
        assert_eq!(DnaNucleotide::from('A'), DnaNucleotide::A);
        assert_eq!(char::from(DnaNucleotide::A), 'A');
    }

    #[test]
    fn test_c() {
        assert_eq!(DnaNucleotide::from('c'), DnaNucleotide::C);
        assert_eq!(DnaNucleotide::from('C'), DnaNucleotide::C);
        assert_eq!(char::from(DnaNucleotide::C), 'C');
    }

    #[test]
    fn test_g() {
        assert_eq!(DnaNucleotide::from('g'), DnaNucleotide::G);
        assert_eq!(DnaNucleotide::from('G'), DnaNucleotide::G);
        assert_eq!(char::from(DnaNucleotide::G), 'G');
    }

    #[test]
    fn test_t() {
        assert_eq!(DnaNucleotide::from('t'), DnaNucleotide::T);
        assert_eq!(DnaNucleotide::from('T'), DnaNucleotide::T);
        assert_eq!(char::from(DnaNucleotide::T), 'T');
    }

    #[test]
    fn test_n() {
        assert_eq!(DnaNucleotide::from('n'), DnaNucleotide::N);
        assert_eq!(DnaNucleotide::from('N'), DnaNucleotide::N);
        assert_eq!(char::from(DnaNucleotide::N), 'N');
    }

    #[test]
    fn test_others() {
        assert_eq!(DnaNucleotide::from('u'), DnaNucleotide::N);
        assert_eq!(DnaNucleotide::from('U'), DnaNucleotide::N);
        assert_eq!(DnaNucleotide::from('B'), DnaNucleotide::N);
        assert_eq!(DnaNucleotide::from('H'), DnaNucleotide::N);
    }


    #[test]
    fn test_dnsequence_to_string() {
        let seq: DnaSequence =
            DnaSequence::from_str(&"AGTACGGCAAGT").expect("Can not parse DNA sequence string");
        assert_eq!(seq.to_string(), "AGTACGGCAAGT");
    }
    #[test]
    fn test_dnsequence_to_reverse_strand() {
        let seq: DnaSequence =
            DnaSequence::from_str(&"AGTACGGCAAGT").expect("Can not parse DNA sequence string");
        assert_eq!(seq.complement().to_string(), "TCATGCCGTTCA");
    }
    #[test]
    fn test_dnsequence_to_complement() {
        let seq: DnaSequence =
            DnaSequence::from_str(&"AGTACGGCAAGT").expect("Can not parse DNA sequence string");
        assert_eq!(seq.reverse_strand().to_string(), "ACTTGCCGTACT");
    }

    /*    #[test]
    fn test_dnasequence_add() {
        let s1 = DnaSequence::from_str(&"ACGT").expect("Can not parse DNA sequence string");
        let s2 = DnaSequence::from_str(&"TGCA").expect("Can not parse DNA sequence string");
        // let s3 = s1 + s2;
        // assert_eq!(s3.to_string(), "ACGTTGCA");
    }*/

    #[test]
    fn test_dna_subsequence() {
        let s1 = DnaSequence::from_str(&"ACGT").expect("Can not parse DNA sequence string");
        assert_eq!(s1.subsequence(0, 1), DnaSequence::from_str(&"A").unwrap());
        assert_eq!(s1.subsequence(0, 2), DnaSequence::from_str(&"AC").unwrap());
        assert_eq!(s1.subsequence(1, 1), DnaSequence::from_str(&"C").unwrap());
        assert_eq!(s1.subsequence(1, 2), DnaSequence::from_str(&"CG").unwrap());
        assert_eq!(s1.subsequence(3, 1), DnaSequence::from_str(&"T").unwrap());
    }
}
