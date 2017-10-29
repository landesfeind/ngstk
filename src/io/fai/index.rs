use std::fs::File;
use std::io;
use std::io::{Write,BufWriter,BufRead,BufReader};
use std::path;
use std::str::FromStr;

use io::fai::FaiRecord;

/// A fasta index represents all the records within a FAI file
#[derive(Clone, Debug)]
pub struct FaiIndex {
    records: Vec<FaiRecord>,
}

impl FaiIndex {
    
    /// Returns the number of records in this index
    pub fn num_records(&self) -> usize {
        self.records.len()
    }

    pub fn records(&self) -> Vec<FaiRecord> {
        self.records.clone()
    }

    /// Returns the names for the records in this index
    pub fn record_names(&self) -> Vec<String> {
        self.records.iter().map(|r| r.name()).collect()
    }

    /// Searches for a record with given name and return its position in the `self.records` files.
    pub fn find_record_index<P: ToString>(&self, name: P) -> Option<usize> {
        let name_s = name.to_string();
        self.records.iter().position(|r| r.name() == name_s)
    }

    /// Searches for a record with given name and returns the record.
    pub fn find_record<P: ToString>(&self, name: P) -> Option<FaiRecord> {
        match self.find_record_index(name) {
            Some(pos) => Some(self.records[pos].clone()),
            None => None,
        }
    }

    /// Tries to discover a fasta index file for a given FASTA file.
    pub fn discover<P: AsRef<path::Path>>(fasta_filename: &P) -> Option<File> {
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

    /// Tries to discover a fasta index file for a given FASTA file.
    pub fn read_fai<P: AsRef<path::Path>>(fai_filename: &P) -> Result<FaiIndex, io::Error> {
        match File::open(fai_filename) {
            Ok(fh) => Ok(FaiIndex::from(BufReader::new(fh))),
            Err(e) => Err(e)
        }
    }

    pub fn write_fai<P: AsRef<path::Path>>(&self, fai_filename: &P) -> Result<usize, io::Error> {
        let fh = File::create(fai_filename)?;
        let mut wr = BufWriter::new(fh);
        let mut counter = 0usize;

        for record in &self.records {
            counter += 1;
            write!(&mut wr, "{}" , record)?
        }

        Ok(counter)
    }
}

impl<R: BufRead> From<R> for FaiIndex {
    fn from(input: R) -> FaiIndex {
        let mut records = Vec::new();

        for line_result in input.lines() {
            match line_result {
                Ok(line) => {
                    match FaiRecord::from_str(line.as_ref()) {
                        Ok(r) => records.push(r),
                        Err(e) => warn!("Can not parse line '{}': {}", line, e),
                    }
                }
                Err(e) => warn!("Can not read line: {}", e),
            }
        }

        FaiIndex { records: records }
    }
}

#[cfg(test)]
mod tests {
    use io::fai::*;
    use std::io::BufReader;

    #[test]
    pub fn test_discovery() {
        assert!(FaiIndex::discover(&"testdata/toy.fasta").is_some());
        assert!(FaiIndex::discover(&"testdata/toy.fa").is_none());
    }

    #[test]
    pub fn test_read_fasta() {
        let index_result = FaiIndex::read_fai(&"testdata/toy.fasta.fai");
        assert!(index_result.is_ok());
        let index = index_result.unwrap();

        assert_eq!(index.num_records(), 2);
        assert_eq!(index.record_names(), vec!["ref".to_string(), "ref2".to_string()]);

        let records = index.records();
        assert_eq!(records[0].name(), "ref");
        assert_eq!(records[0].length(), 45);
        assert_eq!(records[0].offset(),  5);
        assert_eq!(records[0].linebases(), 23);
        assert_eq!(records[0].linewidth(), 24);
        assert_eq!(records[1].name(), "ref2");
        assert_eq!(records[1].length(), 40);
        assert_eq!(records[1].offset(),  58);
        assert_eq!(records[1].linebases(), 12);
        assert_eq!(records[1].linewidth(), 13);
    }
}
