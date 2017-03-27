use std::fmt;
use std::ops;
use std::slice;
use std::cmp::{Ord, Ordering};
use sequence::{Sequence, SequenceElement};
use dna::{DnaNucleotide, DnaCodon, DnaSequence};

#[derive(Clone,Debug)]
pub enum Aminoacid {
    A,
    R,
    N,
    D,
    C,
    E,
    Q,
    G,
    H,
    I,
    L,
    K,
    M,
    F,
    P,
    S,
    T,
    W,
    Y,
    V,
    Unknown,
    Stop,
}
#[derive(Clone,Debug)]
pub enum Aminoacid3 {
    Ala,
    Arg,
    Asn,
    Asp,
    Cys,
    Glu,
    Gln,
    Gly,
    His,
    Ile,
    Leu,
    Lys,
    Met,
    Phe,
    Pro,
    Ser,
    Thr,
    Trp,
    Tyr,
    Val,
    Unknown,
    Stop,
}

impl SequenceElement for Aminoacid {}

impl fmt::Display for Aminoacid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

impl PartialEq for Aminoacid {
    fn eq(&self, other: &Self) -> bool {
        (char::from(self)) == (char::from(other))
    }
}

impl Eq for Aminoacid {}

impl PartialOrd for Aminoacid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((char::from(self).cmp(&char::from(other))))
    }
}

impl Ord for Aminoacid {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl From<char> for Aminoacid {
    fn from(c: char) -> Aminoacid {
        match c {
            'A' => Aminoacid::A,
            'R' => Aminoacid::R,
            'N' => Aminoacid::N,
            'D' => Aminoacid::D,
            'C' => Aminoacid::C,
            'E' => Aminoacid::E,
            'Q' => Aminoacid::Q,
            'G' => Aminoacid::G,
            'H' => Aminoacid::H,
            'I' => Aminoacid::I,
            'L' => Aminoacid::L,
            'K' => Aminoacid::K,
            'M' => Aminoacid::M,
            'F' => Aminoacid::F,
            'P' => Aminoacid::P,
            'S' => Aminoacid::S,
            'T' => Aminoacid::T,
            'W' => Aminoacid::W,
            'Y' => Aminoacid::Y,
            'V' => Aminoacid::V,
            '*' => Aminoacid::Stop,
            _ => Aminoacid::Unknown,
        }
    }
}

impl From<Aminoacid> for char {
    fn from(n: Aminoacid) -> char {
        match n {
            Aminoacid::A => 'A',
            Aminoacid::R => 'R',
            Aminoacid::N => 'N',
            Aminoacid::D => 'D',
            Aminoacid::C => 'C',
            Aminoacid::E => 'G',
            Aminoacid::Q => 'Q',
            Aminoacid::G => 'G',
            Aminoacid::H => 'H',
            Aminoacid::I => 'I',
            Aminoacid::L => 'L',
            Aminoacid::K => 'K',
            Aminoacid::M => 'M',
            Aminoacid::F => 'F',
            Aminoacid::P => 'P',
            Aminoacid::S => 'S',
            Aminoacid::T => 'T',
            Aminoacid::W => 'W',
            Aminoacid::Y => 'Y',
            Aminoacid::V => 'V',
            Aminoacid::Unknown => 'X',
            Aminoacid::Stop => '*',
        }
    }
}


impl<'a> From<&'a Aminoacid> for char {
    fn from(n: &'a Aminoacid) -> char {
        match *n {
            Aminoacid::A => 'A',
            Aminoacid::R => 'R',
            Aminoacid::N => 'N',
            Aminoacid::D => 'D',
            Aminoacid::C => 'C',
            Aminoacid::E => 'E',
            Aminoacid::Q => 'Q',
            Aminoacid::G => 'G',
            Aminoacid::H => 'H',
            Aminoacid::I => 'I',
            Aminoacid::L => 'L',
            Aminoacid::K => 'K',
            Aminoacid::M => 'M',
            Aminoacid::F => 'F',
            Aminoacid::P => 'P',
            Aminoacid::S => 'S',
            Aminoacid::T => 'T',
            Aminoacid::W => 'W',
            Aminoacid::Y => 'Y',
            Aminoacid::V => 'V',
            Aminoacid::Unknown => 'X',
            Aminoacid::Stop => '*',
        }
    }
}


impl From<Aminoacid3> for Aminoacid {
    fn from(c: Aminoacid3) -> Aminoacid {
        match c {
            Aminoacid3::Ala => Aminoacid::A,
            Aminoacid3::Arg => Aminoacid::R,
            Aminoacid3::Asn => Aminoacid::N,
            Aminoacid3::Asp => Aminoacid::D,
            Aminoacid3::Cys => Aminoacid::C,
            Aminoacid3::Glu => Aminoacid::E,
            Aminoacid3::Gln => Aminoacid::Q,
            Aminoacid3::Gly => Aminoacid::G,
            Aminoacid3::His => Aminoacid::H,
            Aminoacid3::Ile => Aminoacid::I,
            Aminoacid3::Leu => Aminoacid::L,
            Aminoacid3::Lys => Aminoacid::K,
            Aminoacid3::Met => Aminoacid::M,
            Aminoacid3::Phe => Aminoacid::F,
            Aminoacid3::Pro => Aminoacid::P,
            Aminoacid3::Ser => Aminoacid::S,
            Aminoacid3::Thr => Aminoacid::T,
            Aminoacid3::Trp => Aminoacid::W,
            Aminoacid3::Tyr => Aminoacid::Y,
            Aminoacid3::Val => Aminoacid::V,
            Aminoacid3::Unknown => Aminoacid::Unknown,
            Aminoacid3::Stop => Aminoacid::Stop,
        }
    }
}
impl fmt::Display for Aminoacid3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Aminoacid3::Ala => write!(f, "Ala"),
            Aminoacid3::Arg => write!(f, "Arg"),
            Aminoacid3::Asn => write!(f, "Asn"),
            Aminoacid3::Asp => write!(f, "Asp"),
            Aminoacid3::Cys => write!(f, "Cys"),
            Aminoacid3::Glu => write!(f, "Glu"),
            Aminoacid3::Gln => write!(f, "Gln"),
            Aminoacid3::Gly => write!(f, "Gly"),
            Aminoacid3::His => write!(f, "His"),
            Aminoacid3::Ile => write!(f, "Ile"),
            Aminoacid3::Leu => write!(f, "Leu"),
            Aminoacid3::Lys => write!(f, "Lys"),
            Aminoacid3::Met => write!(f, "Met"),
            Aminoacid3::Phe => write!(f, "Phe"),
            Aminoacid3::Pro => write!(f, "Pro"),
            Aminoacid3::Ser => write!(f, "Ser"),
            Aminoacid3::Thr => write!(f, "Thr"),
            Aminoacid3::Trp => write!(f, "Trp"),
            Aminoacid3::Tyr => write!(f, "Tyr"),
            Aminoacid3::Val => write!(f, "Val"),
            Aminoacid3::Stop => write!(f, " * "),
            _ => write!(f, " ? "),
        }
    }
}

impl From<Aminoacid> for Aminoacid3 {
    fn from(n: Aminoacid) -> Aminoacid3 {
        match n {
            Aminoacid::A => Aminoacid3::Ala,
            Aminoacid::R => Aminoacid3::Arg,
            Aminoacid::N => Aminoacid3::Asn,
            Aminoacid::D => Aminoacid3::Asp,
            Aminoacid::C => Aminoacid3::Cys,
            Aminoacid::E => Aminoacid3::Glu,
            Aminoacid::Q => Aminoacid3::Gln,
            Aminoacid::G => Aminoacid3::Gly,
            Aminoacid::H => Aminoacid3::His,
            Aminoacid::I => Aminoacid3::Ile,
            Aminoacid::L => Aminoacid3::Leu,
            Aminoacid::K => Aminoacid3::Lys,
            Aminoacid::M => Aminoacid3::Met,
            Aminoacid::F => Aminoacid3::Phe,
            Aminoacid::P => Aminoacid3::Pro,
            Aminoacid::S => Aminoacid3::Ser,
            Aminoacid::T => Aminoacid3::Thr,
            Aminoacid::W => Aminoacid3::Trp,
            Aminoacid::Y => Aminoacid3::Tyr,
            Aminoacid::V => Aminoacid3::Val,
            Aminoacid::Unknown => Aminoacid3::Unknown,
            Aminoacid::Stop => Aminoacid3::Stop,
        }
    }
}

impl<'a> From<&'a DnaCodon> for Aminoacid {
    fn from(c: &DnaCodon) -> Aminoacid {
        match *c {
            DnaCodon(DnaNucleotide::G, DnaNucleotide::C, _) => Aminoacid::A,
            DnaCodon(DnaNucleotide::C, DnaNucleotide::G, _) => Aminoacid::R,
            DnaCodon(DnaNucleotide::A, DnaNucleotide::G, DnaNucleotide::A) => Aminoacid::R,
            DnaCodon(DnaNucleotide::A, DnaNucleotide::G, DnaNucleotide::G) => Aminoacid::R,
            DnaCodon(DnaNucleotide::A, DnaNucleotide::A, DnaNucleotide::T) => Aminoacid::N,
            DnaCodon(DnaNucleotide::A, DnaNucleotide::A, DnaNucleotide::C) => Aminoacid::N,
            DnaCodon(DnaNucleotide::G, DnaNucleotide::A, DnaNucleotide::T) => Aminoacid::D,
            DnaCodon(DnaNucleotide::G, DnaNucleotide::A, DnaNucleotide::C) => Aminoacid::D,
            DnaCodon(DnaNucleotide::T, DnaNucleotide::G, DnaNucleotide::T) => Aminoacid::C,
            DnaCodon(DnaNucleotide::T, DnaNucleotide::G, DnaNucleotide::C) => Aminoacid::C,
            DnaCodon(DnaNucleotide::C, DnaNucleotide::A, DnaNucleotide::A) => Aminoacid::Q,
            DnaCodon(DnaNucleotide::C, DnaNucleotide::A, DnaNucleotide::G) => Aminoacid::Q,
            DnaCodon(DnaNucleotide::G, DnaNucleotide::A, DnaNucleotide::A) => Aminoacid::E,
            DnaCodon(DnaNucleotide::G, DnaNucleotide::A, DnaNucleotide::G) => Aminoacid::E,
            DnaCodon(DnaNucleotide::G, DnaNucleotide::G, _) => Aminoacid::G,
            DnaCodon(DnaNucleotide::C, DnaNucleotide::A, DnaNucleotide::T) => Aminoacid::H,
            DnaCodon(DnaNucleotide::C, DnaNucleotide::A, DnaNucleotide::C) => Aminoacid::H,
            DnaCodon(DnaNucleotide::A, DnaNucleotide::T, DnaNucleotide::T) => Aminoacid::I,
            DnaCodon(DnaNucleotide::A, DnaNucleotide::T, DnaNucleotide::C) => Aminoacid::I,
            DnaCodon(DnaNucleotide::A, DnaNucleotide::T, DnaNucleotide::A) => Aminoacid::I,
            DnaCodon(DnaNucleotide::C, DnaNucleotide::T, _) => Aminoacid::L,
            DnaCodon(DnaNucleotide::T, DnaNucleotide::T, DnaNucleotide::A) => Aminoacid::L,
            DnaCodon(DnaNucleotide::T, DnaNucleotide::T, DnaNucleotide::G) => Aminoacid::L,
            DnaCodon(DnaNucleotide::A, DnaNucleotide::A, DnaNucleotide::A) => Aminoacid::K,
            DnaCodon(DnaNucleotide::A, DnaNucleotide::A, DnaNucleotide::G) => Aminoacid::K,
            DnaCodon(DnaNucleotide::A, DnaNucleotide::T, DnaNucleotide::G) => Aminoacid::M,
            DnaCodon(DnaNucleotide::T, DnaNucleotide::T, DnaNucleotide::T) => Aminoacid::F,
            DnaCodon(DnaNucleotide::T, DnaNucleotide::T, DnaNucleotide::C) => Aminoacid::F,
            DnaCodon(DnaNucleotide::C, DnaNucleotide::C, _) => Aminoacid::P,
            DnaCodon(DnaNucleotide::T, DnaNucleotide::C, _) => Aminoacid::S,
            DnaCodon(DnaNucleotide::A, DnaNucleotide::G, DnaNucleotide::T) => Aminoacid::S,
            DnaCodon(DnaNucleotide::A, DnaNucleotide::G, DnaNucleotide::C) => Aminoacid::S,
            DnaCodon(DnaNucleotide::A, DnaNucleotide::C, _) => Aminoacid::T,
            DnaCodon(DnaNucleotide::T, DnaNucleotide::G, DnaNucleotide::G) => Aminoacid::W,
            DnaCodon(DnaNucleotide::T, DnaNucleotide::A, DnaNucleotide::T) => Aminoacid::Y,
            DnaCodon(DnaNucleotide::T, DnaNucleotide::A, DnaNucleotide::C) => Aminoacid::Y,
            DnaCodon(DnaNucleotide::G, DnaNucleotide::T, _) => Aminoacid::V,
            DnaCodon(DnaNucleotide::T, DnaNucleotide::A, DnaNucleotide::A) => Aminoacid::Stop,
            DnaCodon(DnaNucleotide::T, DnaNucleotide::A, DnaNucleotide::G) => Aminoacid::Stop,
            DnaCodon(DnaNucleotide::T, DnaNucleotide::G, DnaNucleotide::A) => Aminoacid::Stop,
            _ => Aminoacid::Unknown,
        }
    }
}
impl From<DnaCodon> for Aminoacid {
    fn from(c: DnaCodon) -> Aminoacid {
        Aminoacid::from(&c)
    }
}

impl From<(DnaNucleotide, DnaNucleotide, DnaNucleotide)> for Aminoacid {
    fn from(c: (DnaNucleotide, DnaNucleotide, DnaNucleotide)) -> Aminoacid {
        Aminoacid::from(DnaCodon(c.0, c.1, c.2))
    }
}

#[derive(Clone,Debug)]
pub struct Peptide {
    nucleotides: Vec<Aminoacid>,
}

impl Peptide {
    pub fn new() -> Self {
        Peptide { nucleotides: Vec::new() }
    }
}

impl Sequence<Aminoacid> for Peptide {
    fn length(&self) -> usize {
        self.nucleotides.len()
    }

    fn iter(&self) -> slice::Iter<Aminoacid> {
        self.nucleotides.iter()
    }
}

impl Default for Peptide {
    fn default() -> Peptide {
        Peptide { nucleotides: Vec::new() }
    }
}
impl PartialOrd for Peptide {
    fn partial_cmp(&self, other: &Peptide) -> Option<Ordering> {
        self.nucleotides.partial_cmp(&other.nucleotides)
    }
}
impl Ord for Peptide {
    fn cmp(&self, other: &Peptide) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl PartialEq for Peptide {
    fn eq(&self, other: &Peptide) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Peptide {}
impl ops::Index<usize> for Peptide {
    type Output = Aminoacid;

    fn index(&self, i: usize) -> &Aminoacid {
        &self.nucleotides[i]
    }
}
impl ops::Index<ops::Range<usize>> for Peptide {
    type Output = [Aminoacid];

    fn index(&self, i: ops::Range<usize>) -> &[Aminoacid] {
        &self.nucleotides[i]
    }
}
impl ops::Add for Peptide {
    type Output = Peptide;
    fn add(self, other: Peptide) -> Peptide {
        let mut v = self.nucleotides.clone();
        v.extend(other.nucleotides);
        Peptide::from(v)
    }
}
impl<'a> ops::Add<&'a Peptide> for Peptide {
    type Output = Peptide;
    fn add(self, other: &Peptide) -> Peptide {
        let mut v = self.nucleotides.clone();
        v.extend(other.nucleotides.clone());
        Peptide::from(v)
    }
}



impl From<Vec<Aminoacid>> for Peptide {
    fn from(n: Vec<Aminoacid>) -> Peptide {
        Peptide { nucleotides: n }
    }
}

impl From<Peptide> for Vec<Aminoacid> {
    fn from(seq: Peptide) -> Vec<Aminoacid> {
        seq.nucleotides
    }
}

impl<'a> From<&'a DnaSequence> for Peptide {
    fn from(s: &DnaSequence) -> Peptide {
        let aas: Vec<Aminoacid> = s.codons()
            .iter()
            .map(|c| Aminoacid::from(c))
            .collect();
        Peptide::from(aas)
    }
}
impl From<DnaSequence> for Peptide {
    fn from(s: DnaSequence) -> Peptide {
        let aas: Vec<Aminoacid> = s.codons()
            .iter()
            .map(|c| Aminoacid::from(c))
            .collect();
        Peptide::from(aas)
    }
}

impl fmt::Display for Peptide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.iter().map(|n| char::from(n)).collect();
        write!(f, "{}", s)
    }
}