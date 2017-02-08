use std::cmp::{Ord,Ordering};

#[derive(Clone,Debug)]
pub enum Aminoacid  { A, R, N, D, C, E, Q, G, H, I, L, K, M, F, P, S, T, W, Y, V, Unknown, Stop }
pub enum Aminoacid3 { Ala, Arg, Asn, Asp, Cys, Glu, Gln, Gly, His, Ile, Leu, Lys, Met, Phe, Pro, Ser, Thr, Trp, Tyr, Val, Unknown, Stop }

//Ala 	A
//Arg 	R
//Asn 	N
//Asp 	D
//Cys 	C
//Glu   E
//Gln 	Q
//Gly 	G
//His 	H
//Ile 	I
//Leu 	L
//Lys 	K
//Met 	M
//Phe 	F
//Pro 	P
//Ser 	S
//Thr 	T
//Trp 	W
//Tyr 	Y
//Val 	V

impl PartialEq for Aminoacid {
    fn eq(&self, other: &Self) -> bool {
        return (char::from(self)) == (char::from(other));
    }
}

impl Eq for Aminoacid { }

impl PartialOrd for Aminoacid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some( (char::from(self).cmp(& char::from(other))) );
    }
}

impl Ord for Aminoacid {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.partial_cmp(other).unwrap();
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
                _   => Aminoacid::Unknown
        }
    }
}

impl From<Aminoacid> for char {
    fn from(n: Aminoacid) -> char {
        match n {
                Aminoacid::A    => 'A',
                Aminoacid::R    => 'R',
                Aminoacid::N    => 'N',
                Aminoacid::D    => 'D',
                Aminoacid::C    => 'C',
                Aminoacid::E    => 'G',
                Aminoacid::Q    => 'Q',
                Aminoacid::G    => 'G',
                Aminoacid::H    => 'H',
                Aminoacid::I    => 'I',
                Aminoacid::L    => 'L',
                Aminoacid::K    => 'K',
                Aminoacid::M    => 'M',
                Aminoacid::F    => 'F',
                Aminoacid::P    => 'P',
                Aminoacid::S    => 'S',
                Aminoacid::T    => 'T',
                Aminoacid::W    => 'W',
                Aminoacid::Y    => 'Y',
                Aminoacid::V    => 'V',
                Aminoacid::Unknown    => 'X',
                Aminoacid::Stop => '*',
        }
    }
}


impl<'a> From<&'a Aminoacid> for char {
    fn from(n: &'a Aminoacid) -> char {
        match *n {
                Aminoacid::A    => 'A',
                Aminoacid::R    => 'R',
                Aminoacid::N    => 'N',
                Aminoacid::D    => 'D',
                Aminoacid::C    => 'C',
                Aminoacid::E    => 'E',
                Aminoacid::Q    => 'Q',
                Aminoacid::G    => 'G',
                Aminoacid::H    => 'H',
                Aminoacid::I    => 'I',
                Aminoacid::L    => 'L',
                Aminoacid::K    => 'K',
                Aminoacid::M    => 'M',
                Aminoacid::F    => 'F',
                Aminoacid::P    => 'P',
                Aminoacid::S    => 'S',
                Aminoacid::T    => 'T',
                Aminoacid::W    => 'W',
                Aminoacid::Y    => 'Y',
                Aminoacid::V    => 'V',
                Aminoacid::Unknown => 'X',
                Aminoacid::Stop => '*',
        }
    }
}


impl From<Aminoacid3> for Aminoacid {
    fn from(c: Aminoacid3) -> Aminoacid {
        match c {
                Aminoacid3::Ala  => Aminoacid::A,
                Aminoacid3::Arg  => Aminoacid::R,
                Aminoacid3::Asn  => Aminoacid::N,
                Aminoacid3::Asp  => Aminoacid::D,
                Aminoacid3::Cys  => Aminoacid::C,
                Aminoacid3::Glu  => Aminoacid::E,
                Aminoacid3::Gln  => Aminoacid::Q,
                Aminoacid3::Gly  => Aminoacid::G,
                Aminoacid3::His  => Aminoacid::H,
                Aminoacid3::Ile  => Aminoacid::I,
                Aminoacid3::Leu  => Aminoacid::L,
                Aminoacid3::Lys  => Aminoacid::K,
                Aminoacid3::Met  => Aminoacid::M,
                Aminoacid3::Phe  => Aminoacid::F,
                Aminoacid3::Pro  => Aminoacid::P,
                Aminoacid3::Ser  => Aminoacid::S,
                Aminoacid3::Thr  => Aminoacid::T,
                Aminoacid3::Trp  => Aminoacid::W,
                Aminoacid3::Tyr  => Aminoacid::Y,
                Aminoacid3::Val  => Aminoacid::V,
                Aminoacid3::Unknown => Aminoacid::Unknown,
                Aminoacid3::Stop => Aminoacid::Stop,
        }
    }
}

impl From<Aminoacid> for Aminoacid3 {
    fn from(n: Aminoacid) -> Aminoacid3 {
        match n {
                Aminoacid::A       => Aminoacid3::Ala,
                Aminoacid::R       => Aminoacid3::Arg,
                Aminoacid::N       => Aminoacid3::Asn,
                Aminoacid::D       => Aminoacid3::Asp,
                Aminoacid::C       => Aminoacid3::Cys,
                Aminoacid::E       => Aminoacid3::Glu,
                Aminoacid::Q       => Aminoacid3::Gln,
                Aminoacid::G       => Aminoacid3::Gly,
                Aminoacid::H       => Aminoacid3::His,
                Aminoacid::I       => Aminoacid3::Ile,
                Aminoacid::L       => Aminoacid3::Leu,
                Aminoacid::K       => Aminoacid3::Lys,
                Aminoacid::M       => Aminoacid3::Met,
                Aminoacid::F       => Aminoacid3::Phe,
                Aminoacid::P       => Aminoacid3::Pro,
                Aminoacid::S       => Aminoacid3::Ser,
                Aminoacid::T       => Aminoacid3::Thr,
                Aminoacid::W       => Aminoacid3::Trp,
                Aminoacid::Y       => Aminoacid3::Tyr,
                Aminoacid::V       => Aminoacid3::Val,
                Aminoacid::Unknown => Aminoacid3::Unknown,
                Aminoacid::Stop    => Aminoacid3::Stop,
        }
    }
}


