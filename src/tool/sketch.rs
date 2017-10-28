extern crate clap;

use std::io::stdout;
use std::convert::AsRef;
use std::path::Path;
use std::fs::File;
use std::fmt::Display;


use io::bed::*;
use model::{Region,SimpleRegion};
use io::fasta::{FastaReader,IndexedFastaFile};
use sequence::aminoacid::*;
use sequence::dna::*;

use sketch;
use sketch::Canvas;

use tool::Tool;
use util;

pub struct Sketch {}

impl Tool for Sketch {
    fn args<'a, 'b>(s: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
        s.about("Generate a sketch of the data in the given region")
            .arg(
                clap::Arg::with_name("region")
                    .short("r")
                    .long("region")
                    .help("Visualize the given region")
                    .takes_value(true)
                    .required(true)
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
        let (template, offset, length) = match util::parse_region_string(args.value_of("region").unwrap()) {
                Ok(a) => a,
                Err(e) => { error!("Can not parse region string '{}': {}", args.value_of("region").unwrap(), e); return },
        };
        let mut region = SimpleRegion::new(template, offset, length);
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

        if reference.length() < region.length() {
            region = SimpleRegion::new(region.template(), region.offset(), reference.length())
        }
       
        //moz-extension://e94c6c94-63b8-4a7f-aadf-380f4931c605/main-blocked.html?details=eyJ1cmwiOiJodHRwOi8vYml0LmRvLzNEQmlvbG9neSIsImhuIjoiYml0LmRvIiwid2h5IjoiPyJ9 Create the drawing
        let mut drawing = sketch::Sketch::new(sketch::canvas::Svg::new(region.clone()));
        // Parse output image information
        drawing = match args.value_of("image-width") {
            Some(s) => {
                match f64::from_str(s) {
                    Ok(w) => drawing.with_canvas_width(w),
                    Err(e) => { error!("Can not parse --image-width parameter '{}': {}", s, e); return },
                }
            }
            None => drawing.with_canvas_width(reference.length() as f64 * 15f64)
        };

        // Write given title or use the region to display
        match args.value_of("title") {
            Some(s) => {
                drawing.append_title(s);
            }
            None => drawing.append_title(format!("{}: {} - {} ({} bp)", 
                region.template(), 
                region.offset() + 1usize, 
                region.end(), 
                region.length()))
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
                    Err(e) => { error!("Can not open '{}' for writing: {}", p, e); }
                }
            }
            None => { drawing.write(stdout()); }
        }
    }

}

impl Sketch {

    fn load_reference_sequence<P: AsRef<Path> + Display, R: Region>(filename: &P, region: &R) -> Result<DnaSequence,String> {
        let mut fasta = match IndexedFastaFile::open(filename) {
            Ok(f) => f,
            Err(e) => return Err(format!("{}", e))
        };

        let seq = match fasta.search_region_as_dna(region.template(), region.offset(), region.length()) {
                Some(s) => s,
                None => return Err(format!("Can not find region '{}' in: {}", region.template(), filename))
            };

        return Ok(seq);
    }

    fn draw_from_file<P: AsRef<Path> + Display, C: sketch::Canvas, R: Region>(mut drawing: sketch::Sketch<C>, region: &R, filename: &P) -> sketch::Sketch<C> {
        let fss = filename.to_string();

        if fss.ends_with("bam") {
            error!("BAM visualization not yet implemented: {}", fss);
        }
        else if fss.ends_with("bed") || fss.ends_with("bed.gz") {
            match BedStream::open(fss.clone()) {
                Ok(mut r) => drawing.append_bed_records(r.read_records_in_region(region)),
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
