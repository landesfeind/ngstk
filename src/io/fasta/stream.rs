use io::fasta::{FastaReader,FastaRecord};
use std::io::{BufReader, Read};
use std::io::BufRead;
use std::io::Seek;
use std::io::SeekFrom;
use std::iter::Iterator;

/// A stream reader may process only once.
pub struct FastaStream<R: Read> {
    reader: BufReader<R>,
}

impl<R: Read> Iterator for FastaStream<R> {
    type Item = FastaRecord;

    fn next(&mut self) -> Option<FastaRecord> {
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
        let h = header.trim();
        let s: String = body.into_iter()
            .map(|b| b as char)
            .filter(|c| !c.is_whitespace())
            .filter(|c| *c != '>')
            .collect();
        Some(FastaRecord::new(h, s))
    }
}

impl<R: Read + Seek> FastaStream<R> {
    /// Reset the reader to get back to the beginning
    fn reset(&mut self) {
        // Reset to beginning of file
        self.reader.seek(SeekFrom::Start(0));
        // Read in the first part until '>'
        let mut s = Vec::new();
        self.reader.read_until('>' as u8, &mut s);
    }
}

impl<R: Read> FastaReader for FastaStream<R> {
    /// Searches for a specific sequence
    fn search<P: ToString>(&mut self, name: P) -> Option<FastaRecord> {
        for record in self {
            if record.name() == name.to_string() {
                debug!("Found region {} with body {}", record.name(), record.sequence());
                return Some(record);
            }
        }

        None
    }

    /// Search for a specific sequence and extracts the subsequence
    fn search_region<P: ToString>(&mut self, name: P, offset: usize, length: usize) -> Option<FastaRecord> {
        match self.search(name) {
            None => None,
            Some(s) => Some(FastaRecord::new(
                    s.name(), 
                    s.sequence().chars().skip(offset).take(length).collect::<String>())
                ),
        }
    }
}

impl<R: Read> From<R> for FastaStream<R> {
    fn from(r: R) -> FastaStream<R> {
        let mut fr = FastaStream { reader: BufReader::new(r) };
        let mut s = Vec::new();
        fr.reader.read_until('>' as u8, &mut s);
        fr
    }
}


#[cfg(test)]
mod tests {

    use io::fasta::{FastaReader,FastaRecord};
    use io::fasta::stream::FastaStream;
    use std::fs::File;

    #[test]
    pub fn test_next() {
        let file = File::open("testdata/toy.fasta");
        assert!(file.is_ok(), "Creating file");

        let mut reader = FastaStream::from(file.unwrap());

        let mut read_opt = reader.next();
        assert!(read_opt.is_some());
        let mut read = read_opt.unwrap();
        assert_eq!(read, FastaRecord::new("ref", "AGCATGTTAGATAAGATAGCTGTGCTAGTAGGCAGTCAGCGCCAT"));

        read_opt = reader.next();
        assert!(read_opt.is_some());
        read = read_opt.unwrap();
        assert_eq!(read, FastaRecord::new("ref2", "aggttttataaaacaattaagtctacagagcaactacgcg"));

        read_opt = reader.next();
        assert!(read_opt.is_none());
    }

    #[test]
    pub fn test_search_first_entry() {
        let file = File::open("testdata/toy.fasta");
        assert!(file.is_ok(), "Creating file");

        let mut reader = FastaStream::from(file.unwrap());
        let read = reader.search("ref");
        assert_eq!(
            read,
            Some(FastaRecord::new("ref", "AGCATGTTAGATAAGATAGCTGTGCTAGTAGGCAGTCAGCGCCAT" ))
        );
    }

    #[test]
    pub fn test_search_second_entry() {
        let file = File::open("testdata/toy.fasta");
        assert!(file.is_ok(), "Creating file");

        let mut reader = FastaStream::from(file.unwrap());
        let read = reader.search("ref2");
        assert_eq!(
            read,
            Some(FastaRecord::new("ref2", "aggttttataaaacaattaagtctacagagcaactacgcg"))
        );
    }

    #[test]
    pub fn test_search_no_entry() {
        let file = File::open("testdata/toy.fasta");
        assert!(file.is_ok(), "Creating file");

        let mut reader = FastaStream::from(file.unwrap());
        let read = reader.search("ref3");
        assert_eq!(read, None);
    }

    #[test]
    pub fn test_search_region_begin() {
        let file = File::open("testdata/toy.fasta");
        assert!(file.is_ok(), "Creating file");

        let mut reader = FastaStream::from(file.unwrap());
        let read = reader.search_region("ref2", 0, 5);
        assert_eq!(read, Some(FastaRecord::new("ref2", "aggtt")));
    }

    #[test]
    pub fn test_search_region_one() {
        let file = File::open("testdata/toy.fasta");
        assert!(file.is_ok(), "Creating file");

        let mut reader = FastaStream::from(file.unwrap());
        let read = reader.search_region("ref2", 1, 5);
        assert_eq!(read, Some(FastaRecord::new("ref2", "ggttt")));
    }

    #[test]
    pub fn test_search_region_zero_length() {
        let file = File::open("testdata/toy.fasta");
        assert!(file.is_ok(), "Creating file");

        let mut reader = FastaStream::from(file.unwrap());
        let read = reader.search_region("ref2", 1, 0);
        assert_eq!(read, Some(FastaRecord::new("ref2", "")));
    }
}
