
mod data;

use data::sequence::*;
use data::dna::*;
use data::rna::*;
use data::aminoacid::*;

fn main() {
    let seq : DnaSequence = DnaSequence::from("ATGTGGTGCTGATGA");
    let tra = RnaSequence::from(&seq);
    let pep = Peptide::from(&seq);
    println!("<{}", seq.reverse_strand());
    println!(">{}", seq);
    println!("-> {}", tra);
    println!("-> {}", pep);
}
