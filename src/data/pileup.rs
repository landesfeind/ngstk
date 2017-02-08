use std::cmp::{PartialEq,Eq};
use std::collections::HashMap;
use data::nucleotide::DNANucleotide;
use data::readsegment::ReadSegment;
use data::read::Read;

#[derive(Clone,Debug)]
pub struct Pileup {
    /// Stores the data mapping a position toward
    /// a tuple of nucleotide counts. The keys are 
    positions: HashMap<u64, u64>
}

impl Pileup {
    fn get_key(&self, p: u64, n: DNANucleotide) -> u64 {
        p * 10 + (n as u8) as u64
    }

    /// Internal method to add a nucleotide to the pileup at a specific position
    fn add_pileup(&mut self, p: u64, n: DNANucleotide){
        let pn = self.get_key(p, n);

        let counts : u64 = match self.positions.get(&pn) {
            Some(c) => *c,
            None => 0
        };
        self.positions.insert(pn, counts + 1u64);
    }

    /// Returns the nucleotide count of a specific nucleotide `n` at position `p`.
    pub fn nucleotide_count(&self, p: u64, n: DNANucleotide) -> u64 {
        match self.positions.get(&self.get_key(p, n)) {
            Some(c) => *c,
            None => 0
        }
    }

    /// Add a read to the pileup
    pub fn add_read(&mut self, r: &Read) {
        if r.is_aligned() {
            let p = r.position().unwrap();
            for rs in r.segments().iter().filter( |rs| rs.is_aligned() ) {
                let mut o = 0;
                for n in rs.sequence().nucleotides().clone() {
                    self.add_pileup(p+o, n);
                    o += 1;
                }
            }
        }
    }
}

