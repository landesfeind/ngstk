use std::io::{BufReader, Read};
use std::io::BufRead;

pub struct Csv<R: Read> {
    reader: BufReader<R>,
	record_separator: char,
	cell_separator: char
}

impl<R: Read> Csv<R> {

	pub fn set_cell_sepator(&mut self, new_separator: char) {
		self.cell_separator = new_separator;
	}

	pub fn set_record_separator(&mut self, new_separator: char) {
		self.record_separator = new_separator;
	}
}

impl<R: Read> Iterator for Csv<R> {
	type Item = Vec<String>;

	fn next(&mut self) -> Option<Vec<String>> {
		// Variable to read in the full record
        let mut record = Vec::new();
        
        // Read the header line
        match self.reader.read_until(self.record_separator as u8, &mut record) {
            Ok(l) => {
                if l == 0 {
                    return None;
                }
            }
            Err(_) => return None,
        };

        let record_string : String = record.into_iter()
        		.filter(|x| *x as char != self.record_separator)
        		.map(|x| x as char)
        		.collect();
        
        let cells : Vec<String> = record_string
        		.split(self.cell_separator as char)
        		.map(|x| x.to_string())
        		.collect();
        Some(cells)
	}
}

impl<R: Read> From<R> for Csv<R> {
    fn from(r: R) -> Csv<R> {
        Csv { 
        	reader: BufReader::new(r), 
        	record_separator: '\n',
        	cell_separator: '\t'
  		}
    }
}

#[cfg(test)]
mod tests {

    use io::csv::Csv;
    use std::fs::File;
    use std::iter::Iterator;

    #[test]
    pub fn test_next() {
        let file = File::open("testdata/toy.fasta.fai");
        assert!(file.is_ok(), "Opened file");

        let mut csv = Csv::from(file.unwrap());

        let line1 = csv.next();
        assert_eq!(line1, Some(vec!["ref".to_string(), "45".to_string(), "5".to_string(), "45".to_string(), "46".to_string() ]) );

        let line2 = csv.next();
        assert_eq!(line2, Some(vec!["ref2".to_string(), "40".to_string(), "57".to_string(), "40".to_string(), "41".to_string() ]) );

        let line3 = csv.next();
        assert_eq!(line3, None);
    }
}