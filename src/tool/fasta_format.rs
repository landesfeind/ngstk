extern crate clap;

use io::fasta::FastaStream;
use io::fasta::FastaWriter;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;

use tool::Tool;

pub struct FastaFormat {}

impl Tool for FastaFormat {

    fn args<'a, 'b>(s: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
        s.about("Reformats a FASTA file toward a given line length")
            .arg(
                clap::Arg::with_name("in")
                    .long("in")
                    .short("i")
                    .takes_value(true)
                    .help("The input file (use standard input if not given)"),
            )
            .arg(
                clap::Arg::with_name("out")
                    .long("out")
                    .short("o")
                    .takes_value(true)
                    .help("The output file (use standard output if not given)"),
            )
            .arg(
                clap::Arg::with_name("linelength")
                    .long("linelength")
                    .short("l")
                    .takes_value(true)
                    .help(
                        "The length of individual sequence lines should have (defaults to 80)",
                    ),
            )
    }


    fn run(args: &clap::ArgMatches) {
        match args.value_of("in") {
            Some(filename) => {
                match File::open(filename) {
                    Ok(fh) => FastaFormat::with_input(args, fh),
                    Err(e) => panic!("Can not open '{}' for read: {}", filename, e),

                }
            }
            None => FastaFormat::with_input(args, stdin()),
        };

    }
}



impl FastaFormat {
    fn with_input<R: Read>(args: &clap::ArgMatches, reader: R) {
        match args.value_of("out") {
            Some(filename) => {
                match File::create(filename) {
                    Ok(fh) => FastaFormat::with_input_and_output(args, reader, fh),
                    Err(e) => panic!("Can not open '{}' for write: {}", filename, e),

                }
            }
            None => FastaFormat::with_input_and_output(args, reader, stdout()),
        };

    }


    fn with_input_and_output<R: Read, W: Write>(args: &clap::ArgMatches, reader: R, writer: W) {
        let fasta_reader = FastaStream::from(reader);
        let mut fasta_writer = FastaWriter::from(writer);

        match args.value_of("linelength").unwrap_or("80").parse::<usize>() {
            Ok(l) => fasta_writer.set_linelength(l),
            Err(e) => panic!("Can not parse linelength: {}", e),
        }

        for (h, b) in fasta_reader {
            fasta_writer.append(&h, &b);
        }

        fasta_writer.flush();
    }
}
