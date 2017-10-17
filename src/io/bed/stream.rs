

use io::bed::BedRecord;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::str::FromStr;
use std::path::Path;
use std::fs::File;
use std::io;

pub struct BedStream<R: Read> {
    inner: BufReader<R>,
}

impl<R: Read> BedStream<R> {
    pub fn new(src: R) -> Self {
        BedStream { inner: BufReader::new(src) }
    }

    pub fn read_records(&mut self) -> Vec<BedRecord> {
    	self.collect()
    }

}

impl BedStream<File> {
	pub fn open<A: AsRef<Path>>(filename: &A) -> Result<Self,io::Error> {
		match File::open(filename) {
			Ok(fh) => Ok(Self::new(fh)),
			Err(e) => Err(e)
		}
	}
}

impl<R: Read + Seek> BedStream<R> {
    pub fn reset(&mut self) -> bool {
        match self.inner.seek(SeekFrom::Start(0u64)) {
            Ok(p) => 0u64 == p,
            Err(e) => {
                warn!("Can not seek to start of BED stream: {}", e);
                false
            }
        }
    }
}

impl<R: Read> Iterator for BedStream<R> {
    type Item = BedRecord;

    fn next(&mut self) -> Option<BedRecord> {

        loop {
            let mut line = String::new();
            let read_result = self.inner.read_line(&mut line);
            if read_result.is_err() {
                warn!("Can not read from BED stream: {:?}", read_result.err());
                return None;
            }

            if !line.starts_with('#') {
                match BedRecord::from_str(&line) {
                    Ok(r) => return Some(r),
                    Err(e) => {
                        warn!("Can not parse record from BED stream: {}", e);
                        return None;
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
	use io::bed::stream::BedStream;
	use std::fs::File;

	#[test]
	pub fn test_read_file(){
		let mut reader = match BedStream::open(&"testdata/toy.bed") {
			Ok(r) => r,
			Err(e) => panic!("Can not open BedStream: {}", e)
		};
		let records = reader.read_records();

		assert_eq!(records.len(), 4usize);
	}


}