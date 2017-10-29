
use io::fai::FaiRecord;
use sequence::*;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct FaiSequence {
    record: FaiRecord,
    filename: PathBuf,
}

impl FaiSequence {
    pub fn new<P: AsRef<Path>>(record: FaiRecord, filename: &P) -> FaiSequence {
        FaiSequence {
            record: record,
            filename: filename.as_ref().to_path_buf(),
        }
    }
}

impl Sequence<DnaNucleotide> for FaiSequence {
    type SubsequenceType = DnaSequence;

    fn length(&self) -> usize {
        self.record.length()
    }

    fn vec(&self) -> Vec<DnaNucleotide> {
        self.subsequence(0, self.length()).vec()
    }

    fn subsequence(&self, offset: usize, length: usize) -> DnaSequence {
        let n_lines = offset / self.record.linebases();
        let n_bases = offset - (n_lines * self.record.linebases());
        let file_offset = self.record.offset() + n_lines * self.record.linewidth() + n_bases;

        let mut fh = match File::open(&self.filename) {
            Err(_) => return DnaSequence::default(),
            Ok(fh) => fh,
        };

        if ! fh.seek(SeekFrom::Start(file_offset as u64)).is_ok() {
            return DnaSequence::default();
        }


        let sequence: Vec<DnaNucleotide> = fh.bytes()
            .map(|b| b.unwrap() as char)
            .take_while(|c| *c != '>') // Break at new record
            .filter(|c| ! c.is_whitespace() ) // drop whitespaces
            .take(length)
            .map(|c| DnaNucleotide::from(c))
            .collect();

        DnaSequence::from(sequence)
    }
}


impl fmt::Display for FaiSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FaiSequence[{}:{}bp]",
            self.record.name(),
            self.record.length()
        )
    }
}


#[cfg(test)]
mod tests {
    use io::fai::*;
    use sequence::*;

    #[test]
    fn test_a(){
        let index_result = FaiIndex::read_fai(&"testdata/toy.fasta.fai");
        assert!(index_result.is_ok());
        let index = index_result.unwrap();

        let record = index.find_record(&"ref").expect(&"Expected to find a record with name 'ref'");
        let chrom = FaiSequence::new(record, &"testdata/toy.fasta");

        assert_eq!(chrom.subsequence(0,4).to_string(), "AGCA");
        assert_eq!(chrom.subsequence(1,3).to_string(), "GCA");

    }

}