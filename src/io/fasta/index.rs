use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::convert::AsRef;
use io::csv::Csv;

#[derive(Clone,Debug)]
pub struct FastaIndex {
	records: Vec<FastaIndexRecord>
}

/// A fasta index record as defined by http://www.htslib.org/doc/faidx.html
#[derive(Clone,Debug)]
struct FastaIndexRecord {
	name: String,
	length: usize,
	offset: usize,
	linebases: usize,
	linewidth: usize
}
/**
impl From<Vec<FastaIndexRecord>> for FastaIndex {
	fn from(records: Vec<FastaIndexRecord>) -> FastaIndex {
		FastaIndex {
			records: records
		}
	}
}
*/

impl FastaIndex {

	/// Tries to discover a fasta index file for a given FASTA file.
	pub fn open_for<P: AsRef<Path>>(fasta_filename: P) -> Option<FastaIndex> {
		let fasta_path = fasta_filename.as_ref();
		if ! fasta_path.exists() {
			return None;
		}

		if fasta_path.parent().is_none() {
			return None;
		}

		let parent = fasta_path.parent().unwrap();
		let basename = fasta_path.file_name().unwrap();

		let faidx_with_suffix = parent.join(format!("{}.fai", basename.to_str().unwrap()));		
		if faidx_with_suffix.exists() {
			match File::open(&faidx_with_suffix) {
				Ok(fh) => {
					debug!("Found fasta index for '{:?}' at: {:?}",fasta_path, faidx_with_suffix);
					return Some(FastaIndex::from(fh))
				},
				Err(_) => {} ,
			}
		}
		None
	}
}


impl<R: Read> From<R> for FastaIndex {
	fn from(input: R) -> FastaIndex {
		let csv = Csv::from(input);
		let mut records = Vec::new();
		for entry in csv {
			assert_eq!(entry.len(), 5, "FastaIndex file must contain 5 cells each" );
			records.push( 
				FastaIndexRecord {
					name: entry[0].clone(),
					length: entry[1].parse::<usize>().unwrap(),
					offset: entry[2].parse::<usize>().unwrap(),
					linebases: entry[3].parse::<usize>().unwrap(),
					linewidth: entry[4].parse::<usize>().unwrap(),
				}
			)
		}

		FastaIndex {
			records: records
		}
	}
}




#[cfg(test)]
mod tests {

    use io::fasta::index::*;
    
    #[test]
    pub fn test_discovery() {
        assert!( FastaIndex::open_for("testdata/toy.fasta").is_some() );
        assert!( FastaIndex::open_for("testdata/toy.fa").is_none() );
    }
}