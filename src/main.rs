pub mod sequence;
pub mod dna;
pub mod rna;
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
    let seq : Vec<DnaNucleotide> = parse_dna_to_vec("ATGTGGTGCTGATG");
    let tra = seq.transcribe();
    let pep = seq.translate();
    println!("<{}", seq.reverse_strand().to_string());
    println!(">{}", seq.to_string());
    println!("Frame1: {:?}", seq.frame(0usize));
    println!("Frame2: {:?}", seq.frame(1usize));
    println!("Frame3: {:?}", seq.frame(2usize));
    println!("-> {}", tra.to_string());
    println!("-> {}", pep.to_string());
}
fn align() {
    let t: Vec<DnaNucleotide> = parse_dna_to_vec("ATGTGGTGCTGATG");
    let s: Vec<DnaNucleotide> = parse_dna_to_vec("GTGGGTAG");
    let mut a = DefaultAlignment::new(t.clone(), s);
    a.add_segment(0, 4,  2, 4, false);
    a.add_segment(4, 4, 10, 4, true);

    println!("Alignment");
    println!("{}", t.to_string());
    for segment in a.segments() {
        for _ in 0 .. segment.template_offset().unwrap() {
            print!(" ");
        }
        println!("{}", segment.sequence_slice().to_string());
    }
}

//fn sketch() {
//    let seq: DnaSequence = DnaSequence::from("ATGTGGTGCTGATG");
//    println!("{}", sketch::svg::SVG::new(reference).to_string());
//}
