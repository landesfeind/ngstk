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
mod tool;

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
        .subcommand(SubCommand::with_name("fasta-format")
                .about("Reformats a FASTA file toward a given line length")
                .arg(Arg::with_name("in")
                        .short("i")
                        .takes_value(true)
                        .help("The input file (use standard input if not given)")
                    )
                .arg(Arg::with_name("out")
                        .short("o")
                        .takes_value(true)
                        .help("The output file (use standard output if not given)")
                    )
                .arg(Arg::with_name("linelength")
                     .short("l")
                     .takes_value(true)
                     .help("The length of individual sequence lines should have (defaults to 80)")
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
        ("translate", Some(sub_m)) => tool::translate::run(sub_m),
        ("sketch", Some(sub_m)) => tool::sketch::run(sub_m),
        ("fasta-format", Some(sub_m)) => tool::fasta_format::run(sub_m),
        _ => {}
    }
}
