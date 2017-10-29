use std::fmt;
use std::str::FromStr;
/// A fasta index record as defined by http://www.htslib.org/doc/faidx.html
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct FaiRecord {
    name: String,
    length: usize,
    offset: usize,
    linebases: usize,
    linewidth: usize,
}

impl FaiRecord {
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

impl FromStr for FaiRecord {
    type Err = String;

    fn from_str(s: &str) -> Result<FaiRecord, Self::Err> {
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

        Ok(FaiRecord {
            name: entries[0].clone(),
            length: length,
            offset: offset,
            linebases: linebases,
            linewidth: linewidth,
        })
    }
}

impl fmt::Display for FaiRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}\t{}\t{}\t{}\n", self.name, self.length, self.offset, self.linebases, self.linewidth)
    }
}

#[cfg(tests)]
mod tests {

    #[test]
    pub fn test_parse_fasta_index_record() {
        assert!(
            FaiRecord::from_str("ref\t0\t1\t2").is_err(),
            "String with 4 cells results in error"
        );
        assert!(
            FaiRecord::from_str("ref\t0\t1\t2\t3\t4").is_err(),
            "String with 6 cells results in error"
        );

        let parse_result = FaiRecord::from_str("ref\t0\t1\t2\t3");
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
}
