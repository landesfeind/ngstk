
use io::fasta::FastaReader;
use std::convert::AsRef;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::io::BufRead;
use std::io::BufReader;
use std::io::{Seek,SeekFrom};
use std::path::Path;
use std::str::FromStr;

/// A fasta index record as defined by http://www.htslib.org/doc/faidx.html
#[derive(Clone, Debug)]
pub struct FastaIndexRecord {
    name: String,
    length: usize,
    offset: usize,
    linebases: usize,
    linewidth: usize,
}

impl FastaIndexRecord {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn linebases(&self) -> usize {
        self.linebases
    }

    pub fn linewidth(&self) -> usize {
        self.linewidth
    }

    /// Calculates the exact file offset for a given sequence offset
    pub fn offset_region(&self, sequence_offset: usize) -> usize {
        let n_lines = sequence_offset / self.linebases();
        let n_bases = sequence_offset - (n_lines * self.linebases());

        self.offset() + n_lines * self.linewidth() + n_bases
    }
}


impl FromStr for FastaIndexRecord {
    type Err = String;

    fn from_str(s: &str) -> Result<FastaIndexRecord, Self::Err> {
        let entries: Vec<String> = s.split_whitespace().map(|s| s.to_string()).collect();
        if entries.len() != 5 {
            return Err(format!(
                "Expecting 5 cells separated by whitespace for parsing, but found {}",
                entries.len()
            ));
        }
        let length = match entries[1].parse::<usize>() {
            Ok(l) => l,
            Err(e) => return Err(format!("{}", e)),
        };
        let offset = match entries[2].parse::<usize>() {
            Ok(l) => l,
            Err(e) => return Err(format!("{}", e)),
        };
        let linebases = match entries[3].parse::<usize>() {
            Ok(l) => l,
            Err(e) => return Err(format!("{}", e)),
        };
        let linewidth = match entries[4].parse::<usize>() {
            Ok(l) => l,
            Err(e) => return Err(format!("{}", e)),
        };

        Ok(FastaIndexRecord {
            name: entries[0].clone(),
            length: length,
            offset: offset,
            linebases: linebases,
            linewidth: linewidth,
        })
    }
}



#[derive(Clone, Debug)]
pub struct FastaIndex {
    records: Vec<FastaIndexRecord>,
}

impl FastaIndex {
    /// Returns the number of records in this index
    pub fn num_records(&self) -> usize {
        self.records.len()
    }

    /// Returns the names for the records in this index
    pub fn record_names(&self) -> Vec<String> {
        self.records.iter().map(|r| r.name.clone()).collect()
    }

    /// Searches for a record with given name and return its position in the `self.records` files.
    pub fn find_record_index(&self, name: &str) -> Option<usize> {
        self.records.iter().position(|r| r.name == name)
    }

    /// Searches for a record with given name and returns the record.
    pub fn find_record(&self, name: &str) -> Option<FastaIndexRecord> {
        match self.find_record_index(name) {
            Some(pos) => Some(self.records[pos].clone()),
            None => None,
        }
    }

    /// Tries to discover a fasta index file for a given FASTA file.
    pub fn find_for<P: AsRef<Path>>(fasta_filename: &P) -> Option<File> {
        // Check if the FASTA file exists
        let fasta_path = fasta_filename.as_ref();
        if !fasta_path.exists() {
            return None;
        }

        // Get the directory and basename
        if fasta_path.parent().is_none() {
            return None;
        }
        let parent = fasta_path.parent().unwrap();
        let basename = fasta_path.file_name().unwrap();

        // Build the basename for the FASTA Index
        let faidx_with_suffix = parent.join(format!("{}.fai", basename.to_str().unwrap()));
        if faidx_with_suffix.exists() {
            debug!(
                "Found FASTA index for '{:?}' at: {:?}",
                fasta_path,
                faidx_with_suffix
            );
            match File::open(faidx_with_suffix) {
                Ok(fh) => return Some(fh),
                Err(e) => warn!("Can not read existing FASTA index: {}", e),
            }
        }
        None
    }
}

impl<R: BufRead> From<R> for FastaIndex {
    fn from(input: R) -> FastaIndex {
        let mut records = Vec::new();

        for line_result in input.lines() {
            match line_result {
                Ok(line) => {
                    match FastaIndexRecord::from_str(line.as_ref()) {
                        Ok(r) => records.push(r),
                        Err(e) => warn!("Can not parse line '{}': {}", line, e),
                    }
                }
                Err(e) => warn!("Can not read line: {}", e),
            }
        }

        FastaIndex { records: records }
    }
}


#[derive(Debug)]
pub struct IndexedFastaFile {
    index: FastaIndex,
    fh: File,
}

impl IndexedFastaFile {
    pub fn open<P: AsRef<Path> + Display>(fasta_filename: &P) -> Result<Self, String> {
        let index = match FastaIndex::find_for(fasta_filename) {
            Some(fh) => FastaIndex::from(BufReader::new(fh)),
            None => {
                return Err(format!(
                    "Can not find FASTA index file for: {}",
                    fasta_filename
                ))
            }
        };

        let fasta_fh = match File::open(fasta_filename) {
            Ok(fh) => fh,
            Err(e) => {
                return Err(format!(
                    "Can not open FASTA file '{}': {}",
                    fasta_filename,
                    e
                ))
            }
        };


        return Ok(IndexedFastaFile {
            index: index,
            fh: fasta_fh,
        });
    }
}

impl FastaReader for IndexedFastaFile {
    /// Searches for a specific sequence
    fn search(&mut self, name: &str) -> Option<String> {
        match self.index.find_record(name) {
            Some(record) => self.search_region(name, 0usize, record.length()),
            None => None,
        }
    }

    /// Search for a specific sequence-region and extracts the subsequence
    fn search_region(&mut self, name: &str, offset: usize, length: usize) -> Option<String> {
        let record = match self.index.find_record(name) {
            None => return None,
            Some(record) => record,
        };

        let file_offset = record.offset_region(offset);
        match self.fh.seek(SeekFrom::Start(file_offset as u64)) {
        	Err(e) => {
        		warn!("Can not jump to file offset: {}", e); return None
        	},
        	Ok(_) => {}
        }

        let mut sequence : Vec<u8> = Vec::new();
        let mut buffer = [0u8; 1024];
        while sequence.len() < length {
        	match self.fh.read(&mut buffer) {
        		Err(e) => {
        			warn!("Can not read from buffer: {}", e);
        			return None
						}
        		Ok(l) => {
        			let filtered = buffer.iter()
        					.take(l) // ensure we are not running behind the buffer
        					.take_while(|c| (**c as char) != '>' ) // ensure we are not running into the next sequence
        					.filter(|c| ! (**c as char).is_whitespace() ); // drop whitespaces

        			for c in filtered  {
        				sequence.push(*c)
        			}
        		}
        	}
        }

        Some(sequence.iter().take(length).map(|c| *c as char).collect())
    }
}



#[cfg(test)]
mod tests {
    use io::fasta::index::*;
    use std::io::BufReader;

    #[test]
    pub fn test_discovery() {
        assert!(FastaIndex::find_for(&"testdata/toy.fasta").is_some());
        assert!(FastaIndex::find_for(&"testdata/toy.fa").is_none());
    }

    #[test]
    pub fn test_parse_fasta_index_record() {
        assert!(
            FastaIndexRecord::from_str("ref\t0\t1\t2").is_err(),
            "String with 4 cells results in error"
        );
        assert!(
            FastaIndexRecord::from_str("ref\t0\t1\t2\t3\t4").is_err(),
            "String with 6 cells results in error"
        );

        let parse_result = FastaIndexRecord::from_str("ref\t0\t1\t2\t3");
        assert!(
            parse_result.is_ok(),
            "Record with 5 cells is results in record"
        );

        let record = parse_result.unwrap();
        assert_eq!(record.name(), "ref".to_string());
        assert_eq!(record.length(), 0usize);
        assert_eq!(record.offset(), 1usize);
        assert_eq!(record.linebases(), 2usize);
        assert_eq!(record.linewidth(), 3usize);
    }

    #[test]
    pub fn test_parse_fasta_index_file() {
        let index_file = File::open("testdata/toy.fasta.fai").unwrap();
        let index = FastaIndex::from(BufReader::new(index_file));


        assert_eq!(index.num_records(), 2usize);
        assert!(index.find_record("ref").is_some());
        assert!(index.find_record("ref2").is_some());


        let ir_ref = index.find_record("ref").unwrap();
        assert_eq!(ir_ref.name(), "ref".to_string());
        assert_eq!(ir_ref.length(), 45usize);
        assert_eq!(ir_ref.offset(), 5usize);
        assert_eq!(ir_ref.linebases(), 23usize);
        assert_eq!(ir_ref.linewidth(), 24usize);

        let ir_ref2 = index.find_record("ref2").unwrap();
        assert_eq!(ir_ref2.name(), "ref2".to_string());
        assert_eq!(ir_ref2.length(), 40usize);
        assert_eq!(ir_ref2.offset(), 58usize);
        assert_eq!(ir_ref2.linebases(), 12usize);
        assert_eq!(ir_ref2.linewidth(), 13usize);

    }

    #[test]
    pub fn test_read_fasta() {
        let reader_result = IndexedFastaFile::open(&"testdata/toy.fasta");
        assert!(reader_result.is_ok());
        let mut reader = reader_result.unwrap();

        assert_eq!(reader.search_region("ref", 0, 1), Some("A".to_string()));
        assert_eq!(reader.search_region("ref", 0, 2), Some("AG".to_string()));
        assert_eq!(reader.search_region("ref", 1, 1), Some("G".to_string()));

				assert_eq!(reader.search_region("ref", 20, 3), Some("TGT".to_string()));
				assert_eq!(reader.search_region("ref", 20, 5), Some("TGTGC".to_string()));
				assert_eq!(reader.search_region("ref", 42, 3), Some("CAT".to_string()));


				assert_eq!(reader.search_region("ref2", 0, 1), Some("a".to_string()));
        assert_eq!(reader.search_region("ref2", 0, 2), Some("ag".to_string()));
        assert_eq!(reader.search_region("ref2", 1, 1), Some("g".to_string()));
				assert_eq!(reader.search_region("ref2", 8, 4), Some("taaa".to_string()));
				assert_eq!(reader.search_region("ref2", 8, 6), Some("taaaac".to_string()), "Overlap ");
				assert_eq!(reader.search_region("ref2", 38, 3), Some("gcg".to_string()), "End of second sequence");
    }
}
