pub mod sequence;
pub mod rna;
pub mod dna;
pub mod aminoacid;
pub mod alignment;

use dna::*;
use rna::*;
use aminoacid::*;
use alignment::*;

fn main() {
    translate();
    align();
    //sketch();
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
fn align() {
    let t: DnaSequence = DnaSequence::from("ATGTGGTGCTGATG");
    let s: DnaSequence = DnaSequence::from("GTGGGTAG");
    let mut a = DefaultAlignment::new(t.clone(), s);
    a.add_segment(0, 4,  2, 4, false);
    a.add_segment(4, 4, 10, 4, true);

    println!("Alignment");
    println!("{}", t);
    for segment in a.segments() {
        for _ in 0 .. segment.template_offset().unwrap() {
            print!(" ");
        }
        println!("{}", segment.sequence_slice());
    }
}

//fn sketch() {
//    let seq: DnaSequence = DnaSequence::from("ATGTGGTGCTGATG");
//    println!("{}", sketch::svg::SVG::new(reference).to_string());
//}
