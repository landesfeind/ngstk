
use region::Region;
use sketch::Color;
use std::fmt;
use std::str::FromStr;

use util;
/// https://genome.ucsc.edu/FAQ/FAQformat.html#format1
#[derive(Clone, Debug)]
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
    block_sizes: Option<Vec<usize>>,
    block_starts: Option<Vec<usize>>,
}
impl BedRecord {
    pub fn new<S: ToString>(chrom: &S, chrom_start: usize, chrom_end: usize) -> Self {
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
            block_starts: None,
            block_sizes: None,
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

    pub fn with_name<S: ToString>(mut self, new_name: &S) -> Self {
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

    pub fn is_forward_strand(&self) -> Option<bool> {
    	match self.strand() {
    		Some(c) => Some( c == '+'),
    		None => None
    	}
    }

	pub fn is_reverse_strand(&self) -> Option<bool> {
    	match self.strand() {
    		Some(c) => Some( c == '+'),
    		None => None
    	}
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

    pub fn has_thick(&self) -> bool {
        self.thick_start.is_some() && self.thick_end.is_some()
    }

    pub fn with_thick(mut self, new_thick_start: usize, new_thick_end: usize) -> Self {
        self.thick_start = Some(new_thick_start);
        self.thick_end = Some(new_thick_end);
        self
    }

    pub fn without_thick(mut self) -> Self {
        self.thick_start = None;
        self.thick_end = None;
        self
    }

    pub fn thick_start(&self) -> Option<usize> {
        self.thick_start.clone()
    }

    pub fn thick_end(&self) -> Option<usize> {
        self.thick_end.clone()
    }


    pub fn has_item_rgb(&self) -> bool {
        self.item_rgb.is_some()
    }

    pub fn item_rgb(&self) -> Option<Color> {
        self.item_rgb.clone()
    }

    pub fn with_item_rgb(mut self, new_item_rgb: Color) -> Self {
        self.item_rgb = Some(new_item_rgb);
        self
    }

    pub fn without_item_rgb(mut self) -> Self {
        self.item_rgb = None;
        self
    }


    pub fn has_blocks(&self) -> bool {
        self.block_starts.is_some() && self.block_sizes.is_some()
    }

    pub fn with_blocks(mut self, sizes: Vec<usize>, starts: Vec<usize>) -> Self {
        assert_eq!(
            starts.len(),
            sizes.len(),
            "Length of starts and sizes vector must be equal"
        );
        self.block_starts = Some(starts);
        self.block_sizes = Some(sizes);
        self
    }

    pub fn without_blocks(mut self) -> Self {
        self.block_starts = None;
        self.block_sizes = None;
        self
    }

    pub fn block_count(&self) -> Option<usize> {
        if self.has_blocks() {
            match self.block_starts() {
                Some(b) => Some(b.len()),
                None => None,
            }
        } else {
            None

        }
    }

    pub fn block_sizes(&self) -> Option<Vec<usize>> {
        self.block_sizes.clone()
    }

    pub fn block_starts(&self) -> Option<Vec<usize>> {
        self.block_starts.clone()
    }
}

impl From<Region> for BedRecord {
    fn from(r: Region) -> Self {
        Self::new(&r.name(), r.offset().unwrap(), r.end().unwrap() + 1usize)
    }
}

impl From<BedRecord> for Region {
    fn from(r: BedRecord) -> Region {
        Region::new_with_coordinates(r.chrom(), r.chrom_start(), r.length())
    }
}

impl fmt::Display for BedRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cells: Vec<String> = Vec::new();

        cells.push(self.chrom());
        cells.push(self.chrom_start().to_string());
        cells.push(self.chrom_end().to_string());

        if self.has_name() {
            cells.push(self.name().unwrap());

            if self.has_score() {
                cells.push(self.score().unwrap().to_string());
                if self.has_strand() {
                    cells.push(self.strand().unwrap().to_string());

                    if self.has_thick() {
                        cells.push(self.thick_start().unwrap().to_string());
                        cells.push(self.thick_end().unwrap().to_string());

                        if self.has_blocks() {
                            cells.push(self.block_count().unwrap().to_string());
                        }
                    }
                }
            }
        }

        write!(f, "{}", util::join(cells, "\t"))
    }
}




impl FromStr for BedRecord {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let cells = util::split(s, '\t');

        if cells.len() < 3 {
            return Err(format!(
                "Require BED record to have at least 3 cells: {:?}",
                cells
            ));
        }

        let chrom_start = match cells[1].parse::<usize>() {
            Ok(u) => u,
            Err(e) => return Err(format!("Can not parse '{}' as usize: {}", cells[1], e)),
        };

        let chrom_end = match cells[2].parse::<usize>() {
            Ok(u) => u,
            Err(e) => return Err(format!("Can not parse '{}' as usize: {}", cells[2], e)),
        };

        let mut record = BedRecord::new(&cells[0], chrom_start, chrom_end);

        if cells.len() >= 4 {
            record = record.with_name(&cells[3])
        }

        if cells.len() >= 5 {
            match cells[4].parse::<f64>() {
                Ok(v) => record = record.with_score(v),
                Err(e) => return Err(format!("Can not parse '{}' as f64: {}", cells[2], e)),
            }
        }

        if cells.len() >= 6 {
            if cells[5].starts_with("-") {
                record = record.with_strand('-')
            } else {
                record = record.with_strand('+')
            }
        }


        if cells.len() >= 8 {
            let ts = match cells[6].parse::<usize>() {
                Ok(v) => v,
                Err(e) => return Err(format!("Can not parse '{}' as usize: {}", cells[2], e)),
            };
            let te = match cells[7].parse::<usize>() {
                Ok(v) => v,
                Err(e) => return Err(format!("Can not parse '{}' as usize: {}", cells[2], e)),
            };

            record = record.with_thick(ts, te);
        }

        if cells.len() >= 9 {
            let rgb = util::split(&cells[8], ',');

            let r = if rgb.len() > 0 {
	            	match rgb[0].parse::<u8>() {
	                	Ok(v) => v,
	                	Err(e) => return Err(format!("Can not parse '{}' as u8: {}", rgb[0], e)),
	            	} 
            	}
            	else { 0 };

            let g = if rgb.len() > 1 {
	            	match rgb[1].parse::<u8>() {
	                	Ok(v) => v,
		                Err(e) => return Err(format!("Can not parse '{}' as u8: {}", rgb[1], e)),
		            } 
		        } else { 0 };

            let b = if rgb.len() > 2 {
	            	match rgb[2].parse::<u8>() {
	                	Ok(v) => v,
	                	Err(e) => return Err(format!("Can not parse '{}' as u8: {}", rgb[2], e)),
	            	} 
	            } else { 0 };

            record = record.with_item_rgb(Color::new(r, g, b))
        }

        if cells.len() >= 12 {
            let expected_block_counts = match cells[9].parse::<usize>() {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Can not parse blockCount '{}' to usize: {}",
                        cells[9],
                        e
                    ))
                }
            };


            let block_sizes_strings = cells[10]
                .split(',')
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            let block_starts_strings = cells[11]
                .split(',')
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            if block_sizes_strings.len() != expected_block_counts {
                return Err(format!(
                    "Expected {} values in blockSizes: {:?}",
                    expected_block_counts,
                    block_sizes_strings
                ));
            }

            if block_starts_strings.len() != expected_block_counts {
                return Err(format!(
                    "Expected {} values in blockStarts: {:?}",
                    expected_block_counts,
                    block_starts_strings
                ));
            }

            let mut block_sizes: Vec<usize> = Vec::new();
            let mut block_starts: Vec<usize> = Vec::new();

            for i in 0..expected_block_counts {
                let size = match block_sizes_strings[i].parse::<usize>() {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Can not parse blockSizes entry {} '{}' as u8: {}",
                            i,
                            block_sizes_strings[i],
                            e
                        ))
                    }
                };
                let start = match block_starts_strings[i].parse::<usize>() {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Can not parse blockStarts entry {} '{}' as u8: {}",
                            i,
                            block_starts_strings[i],
                            e
                        ))
                    }
                };
                block_sizes.push(size);
                block_starts.push(start);
            }

            record = record.with_blocks(block_sizes, block_starts);
        }

        if cells.len() > 12 {
            error!(
                "BedRecord is expected to have a maximum of 12 columns, ignoring remaining columns: {:?}",
                &cells[12..]
            );
        }

        Ok(record)
    }
}



#[cfg(test)]
mod tests {
    use io::bed::record::BedRecord;
    use region::Region;
    use sketch::Color;
    use std::str::FromStr;

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
        assert_eq!(
            bed.chrom_start(),
            region.offset().unwrap(),
            "Test equal offset/start"
        );
        assert_eq!(bed.length(), region.length().unwrap(), "Test equal length");
        assert_eq!(
            bed.chrom_end(),
            region.end().unwrap() + 1usize,
            "Test equal end"
        );
    }



    #[test]
    fn test_into_region() {
        let bed = BedRecord::new(&"ref", 0, 100);
        let region = Region::from(bed.clone());
        assert_eq!(bed.chrom(), region.name(), "Chromosome vs. name");
        assert_eq!(
            bed.chrom_start(),
            region.offset().unwrap(),
            "Test equal offset/start"
        );
        assert_eq!(bed.length(), region.length().unwrap(), "Test equal length");
        assert_eq!(
            bed.chrom_end(),
            region.end().unwrap() + 1usize,
            "Test equal end"
        );
    }


    #[test]
    fn test_convert_through_region() {
        let bed1 = BedRecord::new(&"ref", 0, 100);
        let region = Region::from(bed1.clone());
        let bed2 = BedRecord::from(region);

        assert_eq!(bed1.chrom(), bed2.chrom(), "Test equal chrom");
        assert_eq!(
            bed1.chrom_start(),
            bed2.chrom_start(),
            "Test equal chrom_start"
        );
        assert_eq!(bed1.chrom_end(), bed2.chrom_end(), "Test equal chrom_end");
    }

	#[test]
    fn test_to_string() {
        let mut r = BedRecord::new(&"ref", 10, 100);
        assert_eq!(r.to_string(), "ref\t10\t100".to_string());

        r = r.with_name(&"Feature");
		assert_eq!(r.to_string(), "ref\t10\t100\tFeature".to_string());

		r = r.with_score(199.5);
		assert_eq!(r.to_string(), "ref\t10\t100\tFeature\t199.5".to_string());

		r = r.with_strand('+');
		assert_eq!(r.to_string(), "ref\t10\t100\tFeature\t199.5\t+".to_string());
    }

    #[test]
    fn test_from_str(){
    	let r = match BedRecord::from_str(&"chr22\t1000\t5000\tcloneA\t960\t+\t1000\t5000\t0\t2\t567,488\t0,3512") {
    		Ok(r) => r,
    		Err(e) => panic!("{}", e)
    	};
    	
    	assert_eq!(r.chrom(), String::from("chr22") );
		assert_eq!(r.chrom_start(), 1000);
		assert_eq!(r.chrom_end(), 5000);
		assert_eq!(r.name(), Some(String::from("cloneA")));
		assert_eq!(r.score(), Some(960f64));
		assert_eq!(r.strand(), Some('+'));
		assert_eq!(r.thick_start(), Some(1000));
		assert_eq!(r.thick_end(), Some(5000));
		assert_eq!(r.item_rgb(), Some(Color::black()));
		assert_eq!(r.block_count(), Some(2usize));
		assert_eq!(r.block_sizes(), Some(vec![567usize, 488usize]));
		assert_eq!(r.block_starts(), Some(vec![0usize, 3512usize]));
    }

}
