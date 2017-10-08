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
            .arg(
                clap::Arg::with_name("sequence")
                    .help(
                        "The DNA sequence to translate. If '-', will read FASTA from stdin.",
                    )
            )
            .arg(clap::Arg::with_name("output").long("out").short("o").help(
                "The output destination. If omitted or \"-\" will write to stdout",
            ))
    }

    fn run(matches: &clap::ArgMatches) {
        match matches.value_of("output") {
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

        match matches.value_of("sequence") {
            Some(seqstring) => {
                match seqstring {
                    "-" => Translate::translate_read_write(stdin(), output),
                    _ => Translate::translate_string(&seqstring, output),
                }
            }
            None => Translate::translate_read_write(stdin(), output),
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
