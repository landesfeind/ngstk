use io::fasta::FastaReader;
use std::io::{BufReader, Read};
use std::io::BufRead;
use std::io::Seek;
use std::io::SeekFrom;
use std::iter::Iterator;

/// A stream reader may process only once.
pub struct FastaStreamReader<R: Read> {
    reader: BufReader<R>,
}

impl<R: Read> Iterator for FastaStreamReader<R> {
    type Item = (String, String);

    fn next(&mut self) -> Option<(String, String)> {
        // Variables for header and body
        let mut header = String::new();
        let mut body = Vec::new();

        // Read the header line
        match self.reader.read_line(&mut header) {
            Ok(l) => {
                if l == 0 {
                    return None;
                }
            }
            Err(_) => return None,
        };

        // Read until the next header line
        match self.reader.read_until('>' as u8, &mut body) {
            Ok(l) => {
                if l == 0 {
                    return None;
                }
            }
            Err(_) => return None,
        }

        // Trim everything
        let h = header.trim().to_string();
        let s: String = body.into_iter()
            .map(|b| b as char)
            .filter(|c| !c.is_whitespace())
            .filter(|c| *c != '>')
            .collect();
        Some((h, s))
    }
}

impl<R: Read + Seek> FastaStreamReader<R> {
    /// Reset the reader to get back to the beginning
    fn reset(&mut self) {
        // Reset to beginning of file
        self.reader.seek(SeekFrom::Start(0));
        // Read in the first part until '>'
        let mut s = Vec::new();
        self.reader.read_until('>' as u8, &mut s);
    }
}

impl<R: Read> FastaReader for FastaStreamReader<R> {
    /// Searches for a specific sequence
    fn search(&mut self, name: &str) -> Option<String> {
        for (h, b) in self {
            if h == name {
                debug!("Found region {} with body {}", h, b);
                return Some(b);
            }
        }

        None
    }

    /// Search for a specific sequence and extracts the subsequence
    fn search_region(&mut self, name: &str, offset: usize, length: usize) -> Option<String> {
        match self.search(name) {
            None => None,
            Some(s) => Some(s.chars().skip(offset).take(length).collect()),
        }
    }
}

impl<R: Read> From<R> for FastaStreamReader<R> {
    fn from(r: R) -> FastaStreamReader<R> {
        let mut fr = FastaStreamReader { reader: BufReader::new(r) };
        let mut s = Vec::new();
        fr.reader.read_until('>' as u8, &mut s);
        fr
    }
}


#[cfg(test)]
mod tests {

    use io::fasta::FastaReader;
    use io::fasta::stream::FastaStreamReader;
    use std::fs::File;

    #[test]
    pub fn test_next() {
        let file = File::open("testdata/toy.fasta");
        assert!(file.is_ok(), "Creating file");

        let mut reader = FastaStreamReader::from(file.unwrap());

        let mut read_opt = reader.next();
        assert!(read_opt.is_some());
        let mut read = read_opt.unwrap();
        assert_eq!(read.0, "ref");
        assert_eq!(read.1, "AGCATGTTAGATAAGATAGCTGTGCTAGTAGGCAGTCAGCGCCAT");
        
        read_opt = reader.next();
        assert!(read_opt.is_some());
        read = read_opt.unwrap();
        assert_eq!(read.0, "ref2");
        assert_eq!(read.1, "aggttttataaaacaattaagtctacagagcaactacgcg");

        read_opt = reader.next();
        assert!(read_opt.is_none());
    }    

    #[test]
    pub fn test_search_first_entry() {
        let file = File::open("testdata/toy.fasta");
        assert!(file.is_ok(), "Creating file");

        let mut reader = FastaStreamReader::from(file.unwrap());
        let read = reader.search(&"ref");
        assert_eq!(read, Some("AGCATGTTAGATAAGATAGCTGTGCTAGTAGGCAGTCAGCGCCAT".to_string()));
    }

    #[test]
    pub fn test_search_second_entry() {
        let file = File::open("testdata/toy.fasta");
        assert!(file.is_ok(), "Creating file");

        let mut reader = FastaStreamReader::from(file.unwrap());
        let read = reader.search(&"ref2");
        assert_eq!(read, Some("aggttttataaaacaattaagtctacagagcaactacgcg".to_string()));
    }

    #[test]
    pub fn test_search_no_entry() {
        let file = File::open("testdata/toy.fasta");
        assert!(file.is_ok(), "Creating file");

        let mut reader = FastaStreamReader::from(file.unwrap());
        let read = reader.search(&"ref3");
        assert_eq!(read, None);
    }

    #[test]
    pub fn test_search_region_begin() {
        let file = File::open("testdata/toy.fasta");
        assert!(file.is_ok(), "Creating file");

        let mut reader = FastaStreamReader::from(file.unwrap());
        let read = reader.search_region(&"ref2", 0, 5);
        assert_eq!(read, Some("aggtt".to_string()));
    }

    #[test]
    pub fn test_search_region_one() {
        let file = File::open("testdata/toy.fasta");
        assert!(file.is_ok(), "Creating file");

        let mut reader = FastaStreamReader::from(file.unwrap());
        let read = reader.search_region(&"ref2", 1, 5);
        assert_eq!(read, Some("ggttt".to_string()));
    }

    #[test]
    pub fn test_search_region_zero_length() {
        let file = File::open("testdata/toy.fasta");
        assert!(file.is_ok(), "Creating file");

        let mut reader = FastaStreamReader::from(file.unwrap());
        let read = reader.search_region(&"ref2", 1, 0);
        assert_eq!(read, Some("".to_string()));
    }
}
