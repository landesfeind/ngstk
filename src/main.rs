pub mod sequence;
pub mod rna;
pub mod dna;
pub mod aminoacid;
pub mod region;
pub mod template;
pub mod genomicregion;
pub mod readsegment;
pub mod read;
pub mod sketch;

use dna::*;
use rna::*;
use aminoacid::*;

fn main() {
    sketch();
}

fn translate() {
    let seq: DnaSequence = DnaSequence::from("ATGTGGTGCTGATG");
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




use genomicregion::GenomicRegion;
use sketch::GraphicsOutput;
use sketch::svg::SVG;
fn sketch() {
    let seq: DnaSequence = DnaSequence::from("ATGTGGTGCTGATG");
    let reference = GenomicRegion::new(&"chr", 0, seq);
    println!("{}", sketch::svg::SVG::new(reference).to_string());
}
