use std::iter::Iterator;
use std::io::Write;
use std::io::BufWriter;
use std::io::Error;

pub struct FastaWriter<W: Write> {
    inner: BufWriter<W>,
    linelength: Option<usize>
}


impl<W: Write> FastaWriter<W> {

	pub fn set_linelength(&mut self, new_linelength: usize){
		assert!(new_linelength > 0, "Line length must be larger than zero");
		self.linelength = Some(new_linelength)
	}

	pub fn get_ref(&self) -> &W {
		self.inner.get_ref()
	}

	pub fn flush(&mut self) -> Result<(), Error> {
		self.inner.flush()
	}

	pub fn append(&mut self, header: &str, body: &str) -> Result<usize, Error> {
		let mut ret;

		match write!(self.inner, ">{}\n", header) {
			Ok(_) => ret = header.chars().count() + 1,
			Err(e) => return Err(e)
		}

		match self.linelength {
			None => match write!(self.inner, "{}\n", body) {
				Ok(_) => ret = body.chars().count() + 1,
				Err(e) => return Err(e)
			},
			Some(len) => {
				let vchars : Vec<char> = body.chars().collect();
				for chunk in vchars.chunks(len) {
					let s : String = chunk.into_iter().collect();
					match write!(self.inner, "{}\n", s) {
						Ok(_) => ret += s.bytes().count(),
						Err(e) => return Err(e)
					}
				}
			}
		}

		return Ok(ret)
	}
}

impl<W: Write> From<W> for FastaWriter<W> {
	fn from(inner: W) -> FastaWriter<W> {
		FastaWriter { inner: BufWriter::new(inner), linelength: None }
	}
}

#[cfg(test)]
mod tests {
	use io::fasta::FastaWriter;

	#[test]
	fn test_write_single() {
		let mut writer = FastaWriter::from(Vec::new());
		let r = writer.append(&"ref", "ACGT");
		writer.flush();
		assert!(r.is_ok());
		let result = writer.get_ref();
		assert_eq!(result.iter().map(|u| *u as char).collect::<String>(), ">ref\nACGT\n");
	}

	#[test]
	fn test_write_single_with_linewidth() {
		let mut writer = FastaWriter::from(Vec::new());
		writer.set_linelength(2usize);
		let r = writer.append(&"ref", "ACGTA");
		writer.flush();
		assert!(r.is_ok());
		let result = writer.get_ref();
		assert_eq!(result.iter().map(|u| *u as char).collect::<String>(), ">ref\nAC\nGT\nA\n");
	}


	#[test]
	fn test_write_two() {
		let mut writer = FastaWriter::from(Vec::new());
		assert!(writer.append("ref", "ACGT").is_ok());
		assert!(writer.append("ref2", "TGCA").is_ok());
		writer.flush();

		let result = writer.get_ref();
		assert_eq!(result.iter().map(|u| *u as char).collect::<String>(), ">ref\nACGT\n>ref2\nTGCA\n");
	}

	#[test]
	fn test_write_two_with_linewidth() {
		let mut writer = FastaWriter::from(Vec::new());
		writer.set_linelength(2usize);
		assert!(writer.append("ref", "ACGT").is_ok());
		assert!(writer.append("ref2", "TGCA").is_ok());
		writer.flush();

		let result = writer.get_ref();
		assert_eq!(result.iter().map(|u| *u as char).collect::<String>(), ">ref\nAC\nGT\n>ref2\nTG\nCA\n");
	}


}