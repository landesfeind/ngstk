pub mod sequence;
pub mod alignment;
mod sketch;
mod util;

use sequence::*;
use sequence::dna::*;
use sequence::rna::*;
use sequence::aminoacid::*;
use alignment::*;
use sketch::GraphicsOutput;
use sketch::ascii::AsciiOutput;

fn main() {
    translate();
    align();
}

fn translate() {
    let seq = DnaSequence::from_str(&"ATGTGGTGCTGATG").expect("Can not parse DNA sequence string");
    let tra = RnaSequence::from(&seq);
    let pep = Peptide::from(&seq);
    println!("<{}", seq.complement());
    println!(">{}", seq);
    println!("Frame1: {:?}", seq.frame(0usize));
    println!("Frame2: {:?}", seq.frame(1usize));
    println!("Frame3: {:?}", seq.frame(2usize));
    println!("-> {}", tra);
    println!("-> {}", pep.to_string());
}

fn align() {
    let t = DnaSequence::from_str(&"ATGTGGTGCTGATG").expect("Can not parse DNA sequence string");
    let s = DnaSequence::from_str(&"GTGGGTAG").expect("Can not parse DNA sequence string");
    let mut a = Alignment::new_aligned(t.clone(), s);
    a.add_segment(0, 4,  2, 4, false);
    a.add_segment(4, 0,  6, 3, false);
    a.add_segment(4, 4, 10, 4, true);

    let mut out = AsciiOutput::new(0usize, t.length());

    out.append_section("Reference");
    out.append_sequence(&t);
    out.append_section("Alignment");
    out.append_alignment(&a);
}

//fn sketch() {
//    let seq = DnaSequence::from_str(&"ATGTGGTGCTGATG").expect("Can not parse DNA sequence string");
//    println!("{}", sketch::svg::SVG::new(reference).to_string());
//}
