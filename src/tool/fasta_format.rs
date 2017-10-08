extern crate clap;

use io::fasta::FastaStream;
use io::fasta::FastaWriter;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;

pub fn run(args: &clap::ArgMatches) {
    match args.value_of("in") {
        Some(filename) => {
            match File::open(filename) {
                Ok(fh) => with_input(args, fh),
                Err(e) => panic!("Can not open '{}' for read: {}", filename, e),

            }
        }
        None => with_input(args, stdin()),
    };

}


fn with_input<R: Read>(args: &clap::ArgMatches, reader: R) {
    match args.value_of("out") {
        Some(filename) => {
            match File::create(filename) {
                Ok(fh) => with_input_and_output(args, reader, fh),
                Err(e) => panic!("Can not open '{}' for write: {}", filename, e),

            }
        }
        None => with_input_and_output(args, reader, stdout()),
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
