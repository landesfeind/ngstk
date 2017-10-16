use std::io::{BufReader, Read};
use std::io::BufRead;
use util;

pub struct Csv<R: Read> {
    reader: BufReader<R>,
    record_separator: char,
    cell_separator: char,
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
        match self.reader.read_until(
            self.record_separator as u8,
            &mut record,
        ) {
            Ok(l) => {
                if l == 0 {
                    return None;
                }
            }
            Err(_) => return None,
        };

        let record_string: String = record
            .into_iter()
            .map(|x| x as char)
            .filter(|x| *x != self.record_separator)
            .collect();


        Some(util::split(record_string, self.cell_separator))
    }
}

impl<R: Read> From<R> for Csv<R> {
    fn from(r: R) -> Csv<R> {
        Csv {
            reader: BufReader::new(r),
            record_separator: '\n',
            cell_separator: '\t',
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
        assert_eq!(
            line1,
            Some(vec![
                "ref".to_string(),
                "45".to_string(),
                "5".to_string(),
                "23".to_string(),
                "24".to_string(),
            ])
        );

        let line2 = csv.next();
        assert_eq!(
            line2,
            Some(vec![
                "ref2".to_string(),
                "40".to_string(),
                "58".to_string(),
                "12".to_string(),
                "13".to_string(),
            ])
        );

        let line3 = csv.next();
        assert_eq!(line3, None);
    }
}
