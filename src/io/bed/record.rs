use region::Region;
use sketch::Color;
/// https://genome.ucsc.edu/FAQ/FAQformat.html#format1

#[derive(Clone,Debug)]
pub struct BedRecord {
	chrom: String,
	chrom_start: usize,
	chrom_end: usize,
	name: Option<String>,
	score: Option<f64>,
	strand: Option<char>,
	thick_start: Option<usize>,
	thick_end: Option<usize>,
	item_rgb: Option<Color>,
	block_count: Option<usize>,
	block_sizes: Option<Vec<usize>>,
	block_starts: Option<Vec<usize>>
}
impl BedRecord {
	pub fn new<S:ToString>(chrom: &S, chrom_start: usize, chrom_end: usize) -> Self {
		BedRecord {
			chrom: chrom.to_string(),
			chrom_start: chrom_start,
			chrom_end: chrom_end,
			name: None,
			score: None,
			strand: None,
			thick_start: None,
			thick_end: None,
			item_rgb: None,
			block_count: None,
			block_starts: None,
			block_sizes: None
		}
	}

	pub fn chrom(&self) -> String {
		self.chrom.clone()
	}
	pub fn with_chrom<S: ToString>(mut self, new_chrom: &S) -> Self {
		self.chrom = new_chrom.to_string();
		self
	}

	pub fn chrom_start(&self) -> usize {
		self.chrom_start
	}

	pub fn with_chrom_start(mut self, new_chrom_start: usize) -> Self {
		self.chrom_start = new_chrom_start;
		self
	}

	pub fn chrom_end(&self) -> usize {
		self.chrom_end
	}

	pub fn with_chrom_end(mut self, new_chrom_end: usize) -> Self {
		self.chrom_end = new_chrom_end;
		self
	}


	pub fn length(&self) -> usize {
		self.chrom_end() - self.chrom_start()
	}


	pub fn has_name(&self) -> bool {
		self.name.is_some()
	}

	pub fn name(&self) -> Option<String> {
		self.name.clone()
	}

	pub fn with_name<S:ToString>(mut self, new_name: &S) -> Self {
		self.name = Some(new_name.to_string());
		self
	}

	pub fn without_name(mut self) -> Self {
		self.name = None;
		self
	}



	pub fn has_strand(&self) -> bool {
		self.strand.is_some()
	}

	pub fn strand(&self) -> Option<char> {
		self.strand.clone()
	}

	pub fn with_strand(mut self, new_strand: char) -> Self {
		self.strand = match new_strand {
			'+' => Some('+'),
			'-' => Some('-'),
			_ => None,
		};

		self
	}

	pub fn without_strand(mut self) -> Self {
		self.strand = None;
		self
	}



	pub fn has_score(&self) -> bool {
		self.score.is_some()
	}

	pub fn score(&self) -> Option<f64> {
		self.score.clone()
	}

	pub fn with_score(mut self, new_score: f64) -> Self {
		self.score = Some(new_score);
		self
	}

	pub fn without_score(mut self) -> Self {
		self.score = None;
		self
	}
}

impl From<Region> for BedRecord {
	fn from(r: Region) -> Self {
		Self::new(
			&r.name(), 
			r.offset().unwrap(), 
			r.end().unwrap() + 1usize)
	}
}

impl From<BedRecord> for Region {
	fn from(r: BedRecord) -> Region {
		Region::new_with_coordinates(
			r.chrom(),
			r.chrom_start(),
			r.length()
		)
	}
}


#[cfg(test)]
mod tests {
	use region::Region;
	use io::bed::record::BedRecord;

	#[test]
	fn test_length() {
		let r = BedRecord::new(&"ref", 0, 100);
		assert_eq!(r.length(), 100usize);
	}

	
	#[test]
	fn test_from_region() {
		let region = Region::new_with_coordinates(&"ref", 0, 100);
		let bed = BedRecord::from(region.clone());
		assert_eq!(bed.chrom(), region.name(), "Chromosome vs. name");
		assert_eq!(bed.chrom_start(), region.offset().unwrap(), "Test equal offset/start");
		assert_eq!(bed.length(), region.length().unwrap(), "Test equal length");
		assert_eq!(bed.chrom_end(), region.end().unwrap() + 1usize, "Test equal end");
	}


	
	#[test]
	fn test_into_region() {
		let bed = BedRecord::new(&"ref", 0, 100);
		let region = Region::from(bed.clone());
		assert_eq!(bed.chrom(), region.name(), "Chromosome vs. name");
		assert_eq!(bed.chrom_start(), region.offset().unwrap(), "Test equal offset/start");
		assert_eq!(bed.length(), region.length().unwrap(), "Test equal length");
		assert_eq!(bed.chrom_end(), region.end().unwrap() + 1usize, "Test equal end");
	}

	
	#[test]
	fn test_convert_through_region() {
		let bed1 = BedRecord::new(&"ref", 0, 100);
		let region = Region::from(bed1.clone());
		let bed2 = BedRecord::from(region);

		assert_eq!(bed1.chrom()      , bed2.chrom(), "Test equal chrom");
		assert_eq!(bed1.chrom_start(), bed2.chrom_start(), "Test equal chrom_start");
		assert_eq!(bed1.chrom_end()  , bed2.chrom_end(), "Test equal chrom_end");
	}





}

