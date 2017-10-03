extern crate clap;
use std::io::stdin;

use io::fasta;
use sequence::aminoacid::*;
use sequence::dna::*;


pub fn run(matches: &clap::ArgMatches) {
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
            let fasta = fasta::open_stream(stdin());
            
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