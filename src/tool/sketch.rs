extern crate clap;

use io::bam::IndexedBamReader;

use io::fasta::{FastaReader, FastaStream};
use region::Region;
use sequence::aminoacid::*;
use sequence::dna::*;
use sketch::SvgOutput;
use sketch::color::SequenceColors;
use std::fs::File;
use std::io::stdout;

use tool::Tool;

pub struct Sketch {}

impl Tool for Sketch {
    fn args<'a, 'b>(s: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
        s.about("Generate a sketch of the data in the given region")
                .arg(clap::Arg::with_name("region")
                     .short("r")
                     .long("region")
                     .help("Analyze the given region")
                     .takes_value(true)
                    )
                .arg(clap::Arg::with_name("reference")
                     .short("f")
                     .long("fasta")
                     .visible_alias("fasta-ref")
                     .value_name("filename")
                     .help("Show sequence from this file as reference (must be indexed with faidx)")
                     .takes_value(true)
                    )
                .arg(clap::Arg::with_name("bam")
                     .short("b")
                     .long("bam")
                     .visible_alias("bamfile")
                     .multiple(true)
                     .help("Show alignments from these mapping files (must be indexed with samtool index)")
                     .value_name("filename")
                     .takes_value(true)
                     .multiple(true)
                    )
                .arg(clap::Arg::with_name("outfile")
                     .short("o")
                     .long("out")
                     .visible_alias("svg")
                     .help("Write to this file instead of stdout")
                     .value_name("filename")
                     .takes_value(true)
                    )
                .arg(clap::Arg::with_name("image-width")
                     .long("image-width")
                     .help("Set the desired width of the output image")
                     .takes_value(true)
                    )
    }

    fn run(args: &clap::ArgMatches) {
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
                debug!(
                    "Loading reference FASTA sequence from: {}",
                    filename_reference
                );
                f
            }
            Err(e) => panic!("Can not open file '{}' for read: {}", filename_reference, e),
        };
        let reference = match region.has_coordinates() {
            false => {
                match FastaStream::from(file_reference).search(region.name()) {
                    Some(record) => record.as_dna(),
                    None => {
                        panic!(
                            "Can not find reference sequence with header '{}'",
                            region.name()
                        )
                    }
                }
            }
            true => {
                match FastaStream::from(file_reference).search_region(
                    region.name(),
                    region.offset().unwrap(),
                    region.length().unwrap(),
                ) {
                    Some(record) => record.as_dna(),
                    None => {
                        panic!(
                            "Can not find reference sequence with header '{}'",
                            region.name()
                        )
                    }
                }
            }
        };
        debug!(
            "Using sequence of {} elements: {}",
            reference.length(),
            reference
        );


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
                    match IndexedBamReader::load_alignments(
                        &region,
                        reference.clone(),
                        &filename_bam,
                    ) {
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
                    }
                    Err(e) => panic!("Can not open '{}' for writing: {}", p, e),
                }
            }
            None => out.write(&mut stdout()),
        }
    }
}
