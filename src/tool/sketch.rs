extern crate clap;

use std::io::stdout;
use std::convert::AsRef;
use std::path::Path;
use std::fs::File;
use std::fmt::Display;


use io::bed::*;

use io::fasta::{FastaReader, FastaStream, IndexedFastaFile};
use region::Region;
use sequence::aminoacid::*;
use sequence::dna::*;

use sketch;

use tool::Tool;

pub struct Sketch {}

impl Tool for Sketch {
    fn args<'a, 'b>(s: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
        s.about("Generate a sketch of the data in the given region")
            .arg(
                clap::Arg::with_name("region")
                    .short("r")
                    .long("region")
                    .help("Visualize the given region")
                    .takes_value(true),
            )
            .arg(
                clap::Arg::with_name("title")
                    .short("t")
                    .long("title")
                    .help("Use the given title (defaults to visualized region)")
                    .takes_value(true),
            )
            .arg(
                clap::Arg::with_name("outfile")
                    .short("o")
                    .long("out")
                    .visible_alias("svg")
                    .help("Write to this file instead of stdout")
                    .value_name("filename")
                    .takes_value(true),
            )
            .arg(
                clap::Arg::with_name("image-width")
                    .long("image-width")
                    .help("Set the desired width of the output image")
                    .takes_value(true),
            )
            .arg(
                clap::Arg::with_name("reference")
                    .short("f")
                    .long("fasta-reference")
                    .visible_alias("reference")
                    .help("Use this file to load the reference sequence from (must be a faidx-indexed FASTA file)")
                    .value_name("filename")
                    .takes_value(true)
                    .required(true)
            )
            .arg(
                clap::Arg::with_name("tracks")
                    .help("Visualize these files")
                    .value_name("filename")
                    .takes_value(true)
                    .multiple(true)
                    .required(true)
            )
    }

    fn run(args: &clap::ArgMatches) {
        // Check for a given region
        let mut region = match Region::from_str(args.value_of("region").unwrap_or("ref")) {
            Ok(r) => {
                debug!("Using region: {}", r);
                r
            }
            Err(e) => { error!("{}", e); return },
        };
        debug!("Start visualization of region: {}", region);

        // Load the reference sequence
        let reference_filename = match args.value_of("reference") {
            None => { error!("Did not found reference file parameter"); return; },
            Some(s) => s
        };
        let reference = match Self::load_reference_sequence(&reference_filename, &region) {
                Err(e) => { error!("{}", e); return }
                Ok(r) => r
        };
        if ! region.has_coordinates() {
            region = Region::new_with_coordinates(region.name(), 0usize, reference.length())
        }

        // Create the drawing
        let mut drawing = sketch::Sketch::default();
        // Parse output image information
        drawing = match args.value_of("image-width") {
            Some(s) => {
                match f64::from_str(s) {
                    Ok(w) => drawing.with_canvas_width(w),
                    Err(e) => { error!("Can not parse --image-width parameter '{}': {}", s, e); return },
                }
            }
            None => drawing.with_canvas_width(reference.length() as f64 * 10f64)
        };

        // Write given title or use the region to display
        match args.value_of("title") {
            Some(s) => {
                drawing.append_title(s);
            }
            None => drawing.append_title(format!("{}: {} - {} ({} bp)", 
                region.name(), 
                region.offset().unwrap() + 1usize, 
                region.end().unwrap(), 
                region.length().unwrap()))
        }
        drawing.append_section(&reference_filename);
        drawing.append_dna_sequence(reference);
        

        match args.values_of("tracks") {
            None => {}
            Some(values) => {
                for filename in values {
                    debug!("Processing track: {}", filename);
                    drawing.append_section(filename);  

                    drawing = Self::draw_from_file(drawing, &region, &filename);
                }
            }
        }
        
        match args.value_of("outfile") {
            Some(p) => {
                match File::create(p) {
                    Ok(f) => {
                        debug!("Writing to output file: {}", p);
                        drawing.write(f);
                    }
                    Err(e) => { error!("Can not open '{}' for writing: {}", p, e); return }
                }
            }
            None => drawing.write(stdout())
        }
    }

}

impl Sketch {

    fn load_reference_sequence<P: AsRef<Path> + Display>(filename: &P, region: &Region) -> Result<DnaSequence,String> {
        let mut fasta = match IndexedFastaFile::open(filename) {
            Ok(f) => f,
            Err(e) => return Err(format!("{}", e))
        };

        let seq = match region.has_coordinates() {
            true => match fasta.search_region_as_dna(region.name(), region.offset().unwrap(), region.length().unwrap()) {
                Some(s) => s,
                None => return Err(format!("Can not find region '{}' in: {}", region, filename))
            },
            false => match fasta.search_as_dna(region.name()) {
                Some(s) => s,
                None => return Err(format!("Can not find region '{}' in: {}", region, filename))
            }
        };

        return Ok(seq);
    }

    fn draw_from_file<P: AsRef<Path> + Display, C: sketch::Canvas>(mut drawing: sketch::Sketch<C>, region: &Region, filename: &P) -> sketch::Sketch<C> {
        let fss = filename.to_string();

        if fss.ends_with("bam") {
            error!("BAM visualization not yet implemented: {}", fss);
        }
        else if fss.ends_with("bed") || fss.ends_with("bed.gz") {
            match BedStream::open(fss.clone()) {
                Ok(mut r) => drawing.append_bed_records(r.read_records_in_region(region), region),
                Err(e) => error!("Can not read BED records from '{}': {}", fss, e)
            }
        }
        else if fss.ends_with("vcf") || fss.ends_with("vcf.gz") {
            error!("VCF visualization not yet implemented: {}", fss);
        }
        else {
            error!("Don't know how to visualize file: {}", fss);
        }

        drawing
    }
}
