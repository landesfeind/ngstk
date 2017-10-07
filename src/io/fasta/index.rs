use std::convert::AsRef;
use std::fs::File;
use std::io::BufRead;
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
    pub fn find_for<P: AsRef<Path>>(fasta_filename: P) -> Option<File> {
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



#[cfg(test)]
mod tests {
    use io::fasta::index::*;
    use std::io::BufReader;

    #[test]
    pub fn test_discovery() {
        assert!(FastaIndex::find_for("testdata/toy.fasta").is_some());
        assert!(FastaIndex::find_for("testdata/toy.fa").is_none());
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

}
