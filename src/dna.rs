use std::cmp::{Ord, Ordering};
use std::ops;
use std::fmt;
use std::slice;

use sequence::{SequenceElement, Sequence};

#[derive(Clone,Debug)]
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
        match n.clone() {
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


#[derive(Clone,Debug)]
pub struct DnaCodon(pub DnaNucleotide, pub DnaNucleotide, pub DnaNucleotide);


impl From<(DnaNucleotide, DnaNucleotide, DnaNucleotide)> for DnaCodon {
    fn from(c: (DnaNucleotide, DnaNucleotide, DnaNucleotide)) -> DnaCodon {
        DnaCodon(c.0, c.1, c.2)
    }
}

impl<'a> From<&'a [DnaNucleotide]> for DnaCodon {
    fn from(e: &[DnaNucleotide]) -> DnaCodon {
        match e.len() {
            1 => DnaCodon(e[0].clone(), DnaNucleotide::N, DnaNucleotide::N),
            2 => DnaCodon(e[0].clone(), e[1].clone(), DnaNucleotide::N),
            3 => DnaCodon(e[0].clone(), e[1].clone(), e[2].clone()),
            _ => DnaCodon(DnaNucleotide::N, DnaNucleotide::N, DnaNucleotide::N),
        }
    }
}

#[derive(Clone,Debug)]
pub struct DnaSequence {
    nucleotides: Vec<DnaNucleotide>,
}

impl DnaSequence {
    pub fn new() -> Self {
        DnaSequence { nucleotides: Vec::new() }
    }

    /// Returns the reverse strand sequence
    pub fn reverse_strand(&self) -> DnaSequence {
        let r: Vec<DnaNucleotide> = self.iter().map(|n| n.complement()).collect();
        DnaSequence::from(r)
    }

    /// Returns the reverse strand sequence in forward direction
    pub fn complement(&self) -> DnaSequence {
        let mut r: Vec<DnaNucleotide> = self.iter().map(|n| n.complement()).collect();
        r.reverse();
        DnaSequence::from(r)
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
        self.nucleotides[offset..self.nucleotides.len()]
            .chunks(3usize)
            .map(|e| DnaCodon::from(e))
            .collect()
    }
    /// Generates all codons that can be generated from the DnaSequence. The
    /// codons will be generated from the first three DnaNucleotide (position 1, 2, and 3),
    /// then DnaNucleotides at position 2, 3, and 4, and so on.
    /// If the DnaSequence is not a multiple of 3, the
    /// last codon will be filled with `DnaNucleotide::N`.
    pub fn all_codons_in_all_frames(&self) -> Vec<DnaCodon> {
        self.nucleotides.windows(3usize).map(|e| DnaCodon::from(e)).collect()
    }
}

impl Sequence<DnaNucleotide> for DnaSequence {
    fn length(&self) -> usize {
        self.nucleotides.len()
    }

    fn iter(&self) -> slice::Iter<DnaNucleotide> {
        self.nucleotides.iter()
    }

    fn slice(&self, offset: usize, length: usize) -> Self {
        let v: Vec<DnaNucleotide> =
            self.iter().skip(offset).take(length).map(|n| (*n).clone()).collect();
        DnaSequence::from(v)
    }
}

impl PartialOrd for DnaSequence {
    fn partial_cmp(&self, other: &DnaSequence) -> Option<Ordering> {
        self.nucleotides.partial_cmp(&other.nucleotides)
    }
}
impl Ord for DnaSequence {
    fn cmp(&self, other: &DnaSequence) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl PartialEq for DnaSequence {
    fn eq(&self, other: &DnaSequence) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for DnaSequence {}
impl ops::Index<usize> for DnaSequence {
    type Output = DnaNucleotide;

    fn index(&self, i: usize) -> &DnaNucleotide {
        &self.nucleotides[i]
    }
}
impl ops::Index<ops::Range<usize>> for DnaSequence {
    type Output = [DnaNucleotide];

    fn index(&self, i: ops::Range<usize>) -> &[DnaNucleotide] {
        &self.nucleotides[i]
    }
}
impl ops::Add for DnaSequence {
    type Output = DnaSequence;
    fn add(self, other: DnaSequence) -> DnaSequence {
        let mut v = self.nucleotides.clone();
        v.extend(other.nucleotides);
        DnaSequence::from(v)
    }
}
impl<'a> ops::Add<&'a DnaSequence> for DnaSequence {
    type Output = DnaSequence;
    fn add(self, other: &DnaSequence) -> DnaSequence {
        let mut v = self.nucleotides.clone();
        v.extend(other.nucleotides.clone());
        DnaSequence::from(v)
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
        let v: Vec<DnaNucleotide> = s.chars().map(|c| DnaNucleotide::from(c)).collect();
        DnaSequence::from(v)
    }
}

impl fmt::Display for DnaSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.iter().map(|n| char::from(n)).collect();
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use dna::DnaNucleotide;
    use dna::DnaSequence;

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
        let seq: DnaSequence = DnaSequence::from("AGTACGGCAAGT");
        assert_eq!(seq.to_string(), "AGTACGGCAAGT");
    }
    #[test]
    fn test_dnsequence_to_reverse_strand() {
        let seq: DnaSequence = DnaSequence::from("AGTACGGCAAGT");
        assert_eq!(seq.reverse_strand().to_string(), "TCATGCCGTTCA");
    }
    #[test]
    fn test_dnsequence_to_complement() {
        let seq: DnaSequence = DnaSequence::from("AGTACGGCAAGT");
        assert_eq!(seq.complement().to_string(), "ACTTGCCGTACT");
    }

    #[test]
    fn test_dnasequence_add() {
        let s1 = DnaSequence::from("ACGT");
        let s2 = DnaSequence::from("TGCA");
        let s3 = s1 + s2;
        assert_eq!(s3.to_string(), "ACGTTGCA");

    }
}
