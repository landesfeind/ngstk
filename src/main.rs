#![allow(dead_code,unused_must_use)]
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

extern crate clap;
use clap::{App, SubCommand, Arg};

pub mod sequence;
pub mod alignment;
mod sketch;
mod util;
mod region;
mod io;

use io::bam::IndexedBamReader;
use io::fasta::FastaReader;
use region::Region;

use sequence::*;
use sequence::aminoacid::*;
use sequence::dna::*;
use sketch::SvgOutput;
use sketch::color::SequenceColors;
use std::fs::File;
use std::io::stdin;

fn main() {
    match pretty_env_logger::init() {
        Err(why) => panic!("Can not initialize logging facility: {:?}", why),
        Ok(_) => {}
    }

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
                .arg(Arg::with_name("outfile")
                     .short("o")
                     .long("out")
                     .visible_alias("svg")
                     .help("Write to this file instead of stdout")
                     .value_name("filename")
                     .takes_value(true)
                    )
                .arg(Arg::with_name("image-width")
                     .long("image-width")
                     .help("Set the desired width of the output image")
                     .takes_value(true)
                    )
            )
        .get_matches();

    match app_matches.subcommand() {
        ("translate", Some(sub_m)) => translate(sub_m),
        ("sketch", Some(sub_m)) => sketch(sub_m),
        _ => {}
    }
}

fn translate(matches: &clap::ArgMatches) {
    match matches.values_of("sequence") {
        Some(seqs) => {
            for seqstring in seqs {
                match DnaSequence::from_str(seqstring.as_ref()) {
                    Ok(dna) => println!("{}", Peptide::from(dna)),
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
        None => {
            let fasta = FastaReader::from(stdin());
            for (header, sequence) in fasta {
                println!(">{}", header);
                match DnaSequence::from_str(sequence.as_ref()) {
                    Ok(dna) => println!("{}", Peptide::from(dna)),
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
    }
}

fn sketch(args: &clap::ArgMatches) {
    let region = match Region::from_str(args.value_of("region").unwrap_or("ref")) {
        Ok(r) => {
            debug!("Using region: {}", r);
            r
        }
        Err(e) => panic!("Error: {}", e),
    };

    // Read the reference sequence
    let filename_reference = args.value_of("reference").unwrap_or("testdata/toy.fasta");
    let file_reference = match File::open(filename_reference) {
        Ok(f) => {
            debug!("Loading reference FASTA sequence from: {}", filename_reference);
            f
        }
        Err(e) => panic!("Can not open file '{}' for read: {}", filename_reference, e),
    };
    let reference = match FastaReader::from(file_reference).search(&region) {
        Some(seq) => {
            match DnaSequence::from_str(seq.as_ref()) {
                Ok(dna) => dna,
                Err(e) => panic!("Can not parse DNA sequence '{}': {}", seq, e),
            }
        },
        None => panic!("Can not find reference sequence with header '{}'", region.name())
    };
    debug!("Using sequence of {} elements: {}", reference.length(), reference);


    // Parse output image information
    let image_width = match args.value_of("image-width") {
        Some(s) => {
            match usize::from_str(s) {
                Ok(w) => w,
                Err(e) => panic!("Can not parse --image-width parameter '{}': {}", s, e),
            }
        }
        None => region.length().unwrap_or(reference.length()) * 15usize,
    };


    // Generate output SVG
    let mut out = SvgOutput::new(
        region.offset().unwrap_or(0usize),
        region.length().unwrap_or(reference.length()),
        image_width,
        SequenceColors::default(),
    );
    out.append_section(format!("{}", region).as_ref());
    out.append_section(filename_reference);
    if region.has_coordinates() {
        out.append_sequence(&reference.subsequence(
            region.offset().unwrap(),
            region.length().unwrap(),
        ));
    } else {
        out.append_sequence(&reference);
    }

    match args.values_of("bam") {
        None => {}
        Some(values) => {
            for filename_bam in values {
                debug!("Processing BAM file: {}", filename_bam);
                match IndexedBamReader::load_alignments(&region, reference.clone(), &filename_bam) {
                    None => panic!("Can not load alignments"),
                    Some(alignments) => {
                        debug!("Found {} aligned reads", alignments.len());
                        out.append_alignments(&alignments)
                    }
                }
            }
        }
    }

    match args.value_of("outfile") {
        Some(p) => {
            match File::create(p) {
                Ok(mut f) => {
                    debug!("Writing to output file: {}", p);
                    out.write(&mut f);
                },
                Err(e) => panic!("Can not open '{}' for writing: {}", p, e),
            }
        }
        None => out.write(&mut std::io::stdout()),
    }
}
