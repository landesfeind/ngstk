extern crate clap;

use std::io::stdout;
use std::convert::AsRef;
use std::path::Path;
use std::fs::File;
use std::fmt::Display;


use io::bam::IndexedBamReader;

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
                clap::Arg::with_name("tracks")
                    .help("Visualize these files")
                    .value_name("filename")
                    .takes_value(true)
                    .multiple(true)
                    .required(true)
            )
    }

    fn run(args: &clap::ArgMatches) {
        let region = match Region::from_str(args.value_of("region").unwrap_or("ref")) {
            Ok(r) => {
                debug!("Using region: {}", r);
                r
            }
            Err(e) => { error!("{}", e); return },
        };
        debug!("Start visualization of region: {}", region);

        // Parse output image information
        let mut style = sketch::Style::default();
        style = match args.value_of("image-width") {
            Some(s) => {
                match u64::from_str(s) {
                    Ok(w) => style.with_image_width(w),
                    Err(e) => { error!("Can not parse --image-width parameter '{}': {}", s, e); return },
                }
            }
            None => style //style.with_image_width(region.length().unwrap() as u64 * 15u64),
        };

        //debug!("Style is: {:?}", style);

        let mut drawing = sketch::Sketch::default().with_canvas_style(style);

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

    fn draw_from_file<P: AsRef<Path> + Display, C: sketch::Canvas>(drawing: sketch::Sketch<C>, region: &Region, filename: &P) -> sketch::Sketch<C> {
        let fss = filename.to_string();

        if fss.ends_with("fa") 
            || fss.ends_with("fasta")
            || fss.ends_with("fn")
            || fss.ends_with("fa.gz") 
            || fss.ends_with("fasta.gz")
            || fss.ends_with("fn.gz") {
            Self::draw_from_fasta(drawing, region, filename)
        }
        else {
            error!("Don't know how to visualize file: {}", fss);
            drawing
        }
    }



    fn draw_from_fasta<P: AsRef<Path> + Display, C: sketch::Canvas>(mut drawing: sketch::Sketch<C>, region: &Region, filename: &P) -> sketch::Sketch<C> {
        let mut fasta = match IndexedFastaFile::open(filename) {
            Ok(f) => f,
            Err(e) => { error!("{}", e); return drawing }
        };

        let seq = match fasta.search_region_as_dna(region.name(), region.offset().unwrap(), region.length().unwrap()) {
            Some(s) => s,
            None => { error!("Can not find region '{}' in: {}", region, filename); return drawing}
        };

        drawing.append_sequence(&seq);

        drawing
    }




}
