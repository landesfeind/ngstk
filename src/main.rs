extern crate clap;
use clap::{App,AppSettings,SubCommand,Arg};

pub mod sequence;
pub mod alignment;
mod sketch;
mod util;
mod region;
mod io;

use std::io::stdin;
use std::fs::File;

use sequence::*;
use sequence::dna::*;
use sequence::rna::*;
use sequence::aminoacid::*;
use alignment::*;
use region::Region;
use sketch::GraphicsOutput;
use sketch::ascii::AsciiOutput;
use sketch::svg::SvgOutput;
use sketch::color::SequenceColors;
use io::fasta::FastaReader;

fn main() {
   let app_matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        //.setting(AppSettings::SubcommandRequired)
        .arg(Arg::with_name("verbose")
             .short("v")
             .long("verbose")
             .multiple(true)
             .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("translate")
                .about("Translates a given input sequence into amino acid sequence")
                .arg(Arg::with_name("sequence")
                     .help("The DNA sequence to translate")
                     .multiple(true)
                    )
                .arg(Arg::with_name("outrna")
                     .long("print-rna")
                     .help("Write out RNA instead of the peptide sequence")
                    )
                .arg(Arg::with_name("reverse")
                     .short("r")
                     .long("reverse")
                     .help("Interpret the sequence as aminoacid sequence and try to reverse translate")
                    )
            )
        .subcommand(SubCommand::with_name("sketch")
                .about("Generate a sketch of the data in the given region")
                .arg(Arg::with_name("region")
                     .short("r")
                     .long("region")
                     .help("Analyze the given region")
                     .takes_value(true)
                    )
                .arg(Arg::with_name("reference")
                     .short("f")
                     .long("fasta")
                     .visible_alias("fasta-ref")
                     .value_name("filename")
                     .help("Show sequence from this file as reference (must be indexed with faidx)")
                     .takes_value(true)
                    )
                .arg(Arg::with_name("bam")
                     .short("b")
                     .long("bam")
                     .visible_alias("bamfile")
                     .multiple(true)
                     .help("Show alignments from these mapping files (must be indexed with samtool index)")
                     .value_name("filename")
                     .takes_value(true)
                     .multiple(true)
                    )
                .arg(Arg::with_name("image-width")
                     .long("image-width")
                     .help("Set the desired width of the output image")
                    )
            )
        .get_matches();

   match app_matches.subcommand() {
       ("translate", Some(sub_m)) => translate(sub_m),
       ("sketch"   , Some(sub_m)) => sketch(sub_m),
       _ => align() 
   }
}

fn translate(matches: &clap::ArgMatches) {
    match matches.values_of("sequence") {
        Some(seqs) => {
            for seqstring in seqs {
                match DnaSequence::from_str(seqstring.as_ref()) {
                    Ok(dna) => println!("{}", Peptide::from(dna)),
                    Err(e) => println!("Error: {}", e)
                }
            }
        },
        None => {
            let fasta = FastaReader::from(stdin());
            for (header, sequence) in fasta {
                println!(">{}", header);
                match DnaSequence::from_str(sequence.as_ref()) {
                    Ok(dna) => println!("{}", Peptide::from(dna)),
                    Err(e) => println!("Error: {}", e)
                }
            }
        }
    }

    //let seq = DnaSequence::from_str(&"ATGTGGTGCTGATG").expect("Can not parse DNA sequence string");
    //let tra = RnaSequence::from(&seq);
    //let pep = Peptide::from(&seq);
    //println!("<{}", seq.complement());
    //println!(">{}", seq);
    //println!("Frame1: {:?}", seq.frame(0usize));
    //println!("Frame2: {:?}", seq.frame(1usize));
    //println!("Frame3: {:?}", seq.frame(2usize));
    //println!("-> {}", tra);
    //println!("-> {}", pep.to_string());
}

fn align() {
    let t = DnaSequence::from_str(&"ATGTGGTCTGATG").expect("Can not parse DNA sequence string");
    let s = DnaSequence::from_str(&"GAGGTTGTAG").expect("Can not parse DNA sequence string");
    let mut a = Alignment::new_aligned(t.clone(), s);
    a.add_segment(0, 4,  2, 4, false);
    a.add_segment(4, 0,  6, 3, false);
    a.add_segment(4, 2,  9, 0, false);
    a.add_segment(6, 4,  9, 4, true);

    //let mut out = AsciiOutput::new(0usize, t.length());
    let mut out = SvgOutput::new(0usize, t.length(), 500, SequenceColors::default());
    out.append_section("Reference");
    out.append_sequence(&t);
    out.append_section("Alignment");
    out.append_alignment(&a);

    println!("{}", out);

}

fn sketch(matches: &clap::ArgMatches) {
    let region = match Region::from_str( matches.value_of("region").unwrap_or("seq1") ) {
            Ok(r) => r,
            Err(e) => panic!("Error: {}", e)
        };
    
    // Read the reference sequence
    let filename_reference = matches.value_of("reference").unwrap_or("testdata/ex1.fasta");
    let file_reference = match File::open(filename_reference) {
            Ok(f) => f,
            Err(e) => panic!("Can not open file '{}' for read: {}", filename_reference, e)
        };
    let reference = match FastaReader::from(file_reference).search(region.name().as_ref()) {
            Some(seq) => match DnaSequence::from_str(seq.as_ref()) {
                Ok(dna) => dna,
                Err(e) => panic!("Can not parse DNA sequence '{}': {}", seq, e)
            },
            None => panic!("Can not find reference sequence with header '{}'", region.name())
        };

    // Parse output image information
    let image_width = match matches.value_of("image-width") {
            Some(s) => match usize::from_str(s) {
                Ok(w) => w,
                Err(e) => panic!("Can not parse image width '{}': {}", s, e)
            },
            None => reference.length() * 15usize
        };
    
    let mut out = SvgOutput::new(region.offset(), reference.length(), image_width, SequenceColors::default());
    out.append_section(filename_reference);
    out.append_sequence(&reference);
    println!("{}", out);
}
