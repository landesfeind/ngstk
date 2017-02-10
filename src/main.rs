
mod data;

use data::sequence::*;
use data::dna::*;

fn main() {
    let seq : Vec<DnaNucleotide> = Vec::from_string("ATGTGGTGCTGATGA");
    let tra = seq.rnanucleotides();
    let cod = seq.codons();
    let pep = seq.aminoacids();
    println!("{:?}", seq);
    println!("-> {:?}", tra);
    println!("-> {:?}", cod);
    println!("-> {:?}", pep);
}
