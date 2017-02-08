use std::collections::HashMap;
use data::nucleotide::DNANucleotide;
use data::readsegment::ReadSegment;
use data::read::Read;

#[derive(Clone,Debug)]
pub struct PositionPileup (u64, u64, u64, u64, u64);

#[derive(Clone,Debug)]
pub struct Pileup {
    /// Stores the data mapping a position toward
    /// a tuple of nucleotide counts. The indices of the
    /// tuple correspond to the 
    positions: HashMap<u64, PositionPileup>
}

impl Pileup {

    /// Returns the pileup counts at position `p`
    pub fn pileup(&self, p: u64) -> PositionPileup {
        match self.positions.get(&p) {
            Some(p) => p.clone(),
            None => PositionPileup (0u64, 0u64, 0u64, 0u64, 0u64)
        }
    }
    /// Internal method to add a nucleotide to the pileup at a specific position
    fn add_pileup(&mut self, p: u64, n: DNANucleotide){
        let np = u8::from(n);
        if self.positions.contains_key(&p) {
            let counts = self.positions.get_mut(&p).unwrap();
            counts[np] += 1;
        }
        else {
            let counts = PositionPileup (0u64, 0u64, 0u64, 0u64, 0u64);
            counts[np] = 1;
            self.positions.insert(p, counts);
        }
    }

    /// Returns the nucleotide count of a specific nucleotide `n` at position `p`.
    pub fn nucleotide_count(&self, p: u64, n: DNANucleotide) -> u64 {
        let i = n as u8;
        let pp = self.pileup(p)[ i ];
    }

    /// Add a read to the pileup
    pub fn add_read(&mut self, r: &Read) {
        if r.is_aligned() {
            let p = r.position().unwrap();
            for rs in r.segments().iter().filter( |rs| rs.is_aligned() ) {
                let mut o = 0;
                for n in rs.sequence().nucleotides() {
                    self.add_pileup(p+o, n);
                    o += 1;
                }
            }
        }
    }
}

