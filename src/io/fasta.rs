use std::io::{BufReader,Read};
use std::io::BufRead;

pub struct FastaReader<R: Read> {
    reader: BufReader<R>
}

impl<R: Read> From<R> for FastaReader<R> {
    fn from(r: R) -> FastaReader<R> {
        let mut fr = FastaReader{ reader: BufReader::new(r) };
        let mut s = Vec::new();
        fr.reader.read_until('>' as u8, &mut s);
        fr
    }
}

impl<R: Read> Iterator for FastaReader<R> {
    type Item = (String, String);

    fn next(&mut self) -> Option<(String, String)> {
        let mut header = String::new();
        let mut body = Vec::new();

        match self.reader.read_line(&mut header) {
            Ok(l) => if l == 0 { return None },
            Err(_) => return None
        };

        match self.reader.read_until('>' as u8, &mut body) {
            Ok(l) => if l == 0 { return None },
            Err(_) => return None
        }
        let h          = header.trim().to_string();
        let s : String = body.into_iter().map(|b| b as char).filter(|c| !c.is_whitespace() ).collect();
        Some( (h, s) )
    }
}


