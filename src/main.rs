
mod data;

use data::sequence::*;
use data::dna::*;
use data::rna::*;
use data::aminoacid::*;

fn main() {
    let seq : DnaSequence = DnaSequence::from("ATGTGGTGCTGATG");
    let tra = RnaSequence::from(&seq);
    let pep = Peptide::from(&seq);
    println!("<{}", seq.reverse_strand());
    println!(">{}", seq);
    println!("Frame1: {:?}", seq.frame(0usize));
    println!("Frame2: {:?}", seq.frame(1usize));
    println!("Frame3: {:?}", seq.frame(2usize));
    println!("All   : {:?}", seq.all_codons_in_all_frames());
    println!("-> {}", tra);
    println!("-> {}", pep);
}
