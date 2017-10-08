extern crate clap;

use io::fasta;
use sequence::aminoacid::*;
use sequence::dna::*;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use tool::Tool;

pub struct Translate {}

impl Tool for Translate {
    fn args<'a, 'b>(s: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
        s.about("Translates a given input sequence into amino acid sequence")
            .arg(clap::Arg::with_name("sequence").help(
                "The DNA sequence to translate. If '-', will read FASTA from stdin.",
            ))
            .arg(clap::Arg::with_name("in").long("in").short("i").takes_value(true).help(
                "The input destination. If omitted or \"-\" will read from stdin",
            ))
            .arg(clap::Arg::with_name("out").long("out").short("o").takes_value(true).help(
                "The output destination. If omitted or \"-\" will write to stdout",
            ))
    }

    fn run(matches: &clap::ArgMatches) {
        match matches.value_of("out") {
            None => Translate::run_with_output(matches, stdout()),
            Some(target) => {
                match target {
                    "-" => Translate::run_with_output(matches, stdout()),
                    _ => {
                        match File::create(target) {
                            Ok(fh) => Translate::run_with_output(matches, fh),
                            Err(e) => panic!("Can not open '{}' for write: {}", target, e),
                        }
                    }
                }
            }
        }
    }
}

impl Translate {
    fn run_with_output<W: Write>(matches: &clap::ArgMatches, output: W) {
        // Check for input sequence first
        match matches.value_of("in") {
            Some(filename) => {
                debug!("Found parameter '--in' with value '{}'", filename);
                match filename {
                    "-" => {
                        debug!("Reading from standard input");
                        Translate::translate_read_write(stdin(), output);
                    },
                    _ => {
                        debug!("Try to read from file: {}", filename);
                        match File::open(filename) {
                            Ok(fh) => Translate::translate_read_write(fh, output),
                            Err(e) => panic!("Can not open '{}' for read: {}", filename, e),
                        }
                    }
                }
            }
            // Check for additional command line parameters
            None => {
                match matches.value_of("sequence") {
                    Some(seqstring) => {
                        debug!("Found sequence with value {}", seqstring);
                        match seqstring {
                            "-" => Translate::translate_read_write(stdin(), output),
                            _ => Translate::translate_string(&seqstring, output),
                        }
                    }
                    None => Translate::translate_read_write(stdin(), output),
                }
            }
        }
    }

    fn translate_string<S: ToString, W: Write>(sequence: &S, mut output: W) {
        match DnaSequence::from_str(sequence.to_string().as_ref()) {
            Ok(dna) => {
                match write!(output, "{}\n", Peptide::from(dna)) {
                    Ok(_) => {}
                    Err(e) => panic!("Error: {}", e),
                }
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    fn translate_read_write<R: Read, W: Write>(input: R, output: W) {
        Translate::translate_fasta(
            fasta::FastaStream::from(input),
            fasta::FastaWriter::from(output),
        );
    }

    // Translates a FASTA source into a FASTA destination
    fn translate_fasta<R: Read, W: Write>(
        input: fasta::FastaStream<R>,
        mut output: fasta::FastaWriter<W>,
    ) {
        for (header, sequence) in input {
            match DnaSequence::from_str(sequence.as_ref()) {
                Ok(dna) => {
                    match output.append(header, Peptide::from(dna)) {
                        Ok(_) => {}
                        Err(e) => panic!("Can not write: {}", e),
                    }
                }
                Err(e) => panic!("Can not parse sequence: {}", e),
            }
        }
    }
}
