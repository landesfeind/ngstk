

use sequence::dna::*;
use std::cmp::{Ord, Ordering};
use std::fmt;
pub use std::str::FromStr;

#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
impl From<Aminoacid> for u8 {
    fn from(n: Aminoacid) -> u8 {
        char::from(n) as u8
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


#[derive(Clone, Debug)]
pub struct Peptide {
    elements: Vec<Aminoacid>,
}

impl Peptide {}


impl Sequence<Aminoacid> for Peptide {
    type SubsequenceType = Peptide;
    
    fn length(&self) -> usize {
        self.elements.len()
    }

    fn vec(&self) -> Vec<Aminoacid> {
        self.elements.clone()
    }

    fn subsequence(&self, offset:usize, length: usize) -> Peptide {
        let v : Vec<Aminoacid> = self.elements.iter().cloned().skip(offset).take(length).collect();
        Peptide::from(v)
    }
}

impl PartialEq for Peptide {
    fn eq(&self, other: &Peptide) -> bool {
        self.elements.eq(&other.elements)
    }
}
impl Eq for Peptide {}

impl PartialOrd for Peptide {
    fn partial_cmp(&self, other: &Peptide) -> Option<Ordering> {
        self.elements.partial_cmp(&other.elements)
    }
}
impl Ord for Peptide {
    fn cmp(&self, other: &Peptide) -> Ordering {
        self.elements.cmp(&other.elements)
    }
}
impl Default for Peptide {
    fn default() -> Peptide {
        Peptide { elements: Vec::new() }
    }
}
impl FromStr for Peptide {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<Aminoacid> = s.chars()
            .filter(|n| *n != '\t' && *n != '\n' && *n != ' ')
            .map(|n| Aminoacid::from(n))
            .collect();
        Ok(Peptide::from(v))
    }
}
impl From<Vec<Aminoacid>> for Peptide {
    fn from(v: Vec<Aminoacid>) -> Peptide {
        Peptide { elements: v }
    }
}
impl fmt::Display for Peptide {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let s: String = self.vec().iter().map(|n| char::from(n)).collect();
        write!(f, "{}", s)
    }
}
impl From<Peptide> for Vec<Aminoacid> {
    fn from(seq: Peptide) -> Vec<Aminoacid> {
        seq.vec().iter().map(|n| n.clone()).collect()
    }
}
impl<'a> From<&'a Peptide> for Vec<Aminoacid> {
    fn from(seq: &'a Peptide) -> Vec<Aminoacid> {
        seq.vec().iter().map(|n| n.clone()).collect()
    }
}
impl From<DnaSequence> for Peptide {
    fn from(d: DnaSequence) -> Peptide {
        Peptide::from(&d)
    }
}
impl<'a> From<&'a DnaSequence> for Peptide {
    fn from(d: &'a DnaSequence) -> Peptide {
        let v: Vec<Aminoacid> = d.codons().iter().map(|n| Aminoacid::from(n)).collect();
        Peptide::from(v)
    }
}
impl From<Vec<DnaCodon>> for Peptide {
    fn from(d: Vec<DnaCodon>) -> Peptide {
        Peptide::from(&d)
    }
}
impl<'a> From<&'a Vec<DnaCodon>> for Peptide {
    fn from(d: &'a Vec<DnaCodon>) -> Peptide {
        let v: Vec<Aminoacid> = d.iter().map(|c| Aminoacid::from(c)).collect();
        Peptide::from(v)
    }
}
