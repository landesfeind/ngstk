use std::fmt;
use std::str::FromStr;
use model::Strand;
use util;

#[derive(Clone,Debug)]
pub enum GtfFeature {
	StartCodon, StopCodon, Exon, CDS, Intron, Gene, Transcript
}

#[derive(Clone,Debug,PartialEq)]
pub enum GtfAnnotation {
	GeneId(String),
	TranscriptId(String),
	ExonNumber(usize),
	Unknown(String,String)
}

#[derive(Clone,Debug)]
pub struct GtfRecord {
	seqname: String,
	source: Option<String>,
	feature: Option<GtfFeature>,
	start: u64,
	end: u64,
	score: Option<f64>,
	strand: Option<Strand>,
	frame: Option<usize>,
	annotations: Vec<GtfAnnotation>
}

impl GtfRecord {
	pub fn new<TN: ToString>(template_name: &TN, start: u64, end: u64) -> GtfRecord {
		assert!(start >      0);
		assert!(end   >= start);
		GtfRecord {
			seqname: template_name.to_string(),
			start: start,
			end: end,
			source: None,
			feature: None,
			score: None,
			strand: None,
			frame: None,
			annotations: Vec::new()
		}
	}

	pub fn seqname(&self) -> &str {
		self.seqname.as_ref()
	}

	pub fn with_seqname<T: ToString>(mut self, new_seqname: &T) -> Self {
		self.seqname = new_seqname.to_string();
		self
	}

	pub fn start(&self) -> u64 {
		self.start
	}

	pub fn with_start(mut self, new_start: u64) -> Self {
		assert!(new_start > 0);
		self.start = new_start;
		self
	}

	pub fn end(&self) -> u64 {
		self.end
	}

	pub fn with_end(mut self, new_end: u64) -> Self {
		assert!(new_end >= self.start);
		self.start = new_end;
		self
	}

	pub fn has_source(&self) -> bool {
		self.source.is_some()
	}
	pub fn source(&self) -> Option<String> {
		self.source.clone()
	}
	pub fn with_source<T: ToString>(mut self, new_source: &T) -> Self {
		self.source = Some(new_source.to_string());
		self
	}
	pub fn without_source(mut self) -> Self {
		self.source = None;
		self
	}


	pub fn has_feature(&self) -> bool {
		self.feature.is_some()
	}
	pub fn feature(&self) -> Option<GtfFeature> {
		self.feature.clone()
	}
	pub fn with_feature(mut self, new_feature: GtfFeature) -> Self {
		self.feature = Some(new_feature);
		self
	}
	pub fn without_feature(mut self) -> Self {
		self.feature = None;
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
		self.feature = None;
		self
	}

	
	pub fn has_strand(&self) -> bool {
		self.strand.is_some()
	}
	pub fn strand(&self) -> Option<Strand> {
		self.strand.clone()
	}
	pub fn with_strand(mut self, new_strand: Strand) -> Self {
		self.strand = Some(new_strand);
		self
	}
	pub fn without_strand(mut self) -> Self {
		self.strand = None;
		self
	}


	pub fn has_frame(&self) -> bool {
		self.frame.is_some()
	}
	pub fn frame(&self) -> Option<usize> {
		self.frame.clone()
	}
	pub fn with_frame(mut self, new_frame: usize) -> Self {
		self.frame = Some(new_frame);
		self
	}
	pub fn without_frame(mut self) -> Self {
		self.frame = None;
		self
	}


	pub fn has_annotations(&self) -> bool {
		self.annotations.len() > 0
	}
	pub fn annotations(&self) -> Vec<GtfAnnotation> {
		self.annotations.clone()
	}
	pub fn with_annotations(mut self, new_annotations: Vec<GtfAnnotation>) -> Self {
		self.annotations = new_annotations;
		self
	}
	pub fn without_annotations(mut self) -> Self {
		self.annotations.clear();
		self
	}

	pub fn add_annotation(mut self, new_annotation: GtfAnnotation) -> Self {
		self.annotations.push(new_annotation);
		self
	}

	pub fn remove_annotation(mut self, annotation_to_remove: GtfAnnotation) -> Self {
		let p = self.annotations.iter().position(|item| *item == annotation_to_remove);
		if p.is_some() {
			self.annotations.remove(p.unwrap());
		}
		self
	}
}

impl FromStr for GtfFeature {
	type Err = String;

	fn from_str(s: &str) -> Result<GtfFeature, String> {
		match s.to_lowercase().as_ref() {
			"gene" => Ok(GtfFeature::Gene),
			"transcript" => Ok(GtfFeature::Transcript),
			"cds" => Ok(GtfFeature::CDS),
			"exon" => Ok(GtfFeature::Exon),
			"intron" => Ok(GtfFeature::Intron),
			"start_codon" => Ok(GtfFeature::StartCodon),
			"stop_codon" => Ok(GtfFeature::StopCodon),
			_ => Err(format!("No such feature '{}'", s))
		}
	}
}
impl fmt::Display for GtfFeature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	match self.clone() {
    		GtfFeature::Gene => write!(f, "gene"),
    		GtfFeature::Transcript => write!(f, "transcript"),
    		GtfFeature::CDS => write!(f, "CDS"),
    		GtfFeature::Exon => write!(f, "exon"),
    		GtfFeature::Intron => write!(f, "intron"),
    		GtfFeature::StartCodon => write!(f, "start_codon"),
    		GtfFeature::StopCodon => write!(f, "stop_codon")
    	}
	}
}
impl FromStr for GtfAnnotation {
	type Err = String;

	fn from_str(s: &str) -> Result<GtfAnnotation, String> {
		let mut parts = util::split(s, ' ');
		parts[1] = parts[1].chars().filter(|c| *c == '"').collect();

		match parts[0].to_lowercase().as_ref() {
			"gene_id" => Ok(GtfAnnotation::GeneId(parts[1].to_string())),
			"transcript_id" => Ok(GtfAnnotation::TranscriptId(parts[1].to_string())),
			"exon_number" => match parts[1].parse::<usize>() {
				Ok(n) => Ok(GtfAnnotation::ExonNumber(n)),
				Err(e) => Err(format!("Can not parse exon number '{}'", e))
			},
			_ => Ok(GtfAnnotation::Unknown(parts[0].to_string(), parts[1].to_string()))
		}
	}
}

impl fmt::Display for GtfAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	match self.clone() {
    		GtfAnnotation::GeneId(s) => write!(f, "gene_id \"{}\"", s),
    		GtfAnnotation::TranscriptId(s) => write!(f, "transcript_id \"{}\"", s),
    		GtfAnnotation::ExonNumber(s) => write!(f, "exon_number {}", s),
    		GtfAnnotation::Unknown(k,v) => write!(f, "{} \"{}\"", k, v)
    	}
	}	
}

impl FromStr for GtfRecord {
	type Err = String;

	fn from_str(s: &str) -> Result<GtfRecord, String> {
		let parts = util::split(s, '\t');
		if parts.len() != 9 {
			return Err(format!("Expected 9 cells separated by tab but found {}", parts.len()))
		}

		let mut record = match parts[3].parse::<u64>() {
			Err(e) => return Err(format!("Can not parse cell 4 as start position: {}", e)),
			Ok(start) => match parts[4].parse::<u64>() {
				Err(e) => return Err(format!("Can not parse cell 5 as end position: {}", e)),
				Ok(end) => GtfRecord::new(&parts[0], start, end)
			}
		};

		if parts[1] != "." {
			record = record.with_source(&parts[1]);
		}

		match parts[2].parse::<GtfFeature>() {
			Ok(f) => record = record.with_feature(f),
			Err(e) => return Err(format!("Can not parse cell 3 (feature): {}", e))
		}

		if parts[5] != "." {
			match parts[5].parse::<f64>() {
				Ok(f) => record = record.with_score(f),
				Err(e) => return Err(format!("Can not parse cell 6 (score): {}", e))
			}
		}

		if parts[6] != "." {
			match parts[6].parse::<Strand>() {
				Ok(f) => record = record.with_strand(f),
				Err(e) => return Err(format!("Can not parse cell 7 (strand): {}", e))
			}
		}

		if parts[7] != "." {
			match parts[7].parse::<usize>() {
				Ok(f) => record = record.with_frame(f),
				Err(e) => return Err(format!("Can not parse cell 8 (frame): {}", e))
			}
		}

		if parts[8] != "." {
			for annotation in util::split(&parts[8], "; "){
				match annotation.parse::<GtfAnnotation>() {
					Ok(a) => record = record.add_annotation(a),
					Err(e) => return Err(e)
				};
			}
		}

		Ok(record)
	}
}


impl fmt::Display for GtfRecord { 

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let mut cells = Vec::new();

    	cells.push(self.seqname().to_string());
    	match self.source() {
    		Some(s) => cells.push(s),
    		None => cells.push(".".to_string())
    	};
    	match self.feature() {
    		Some(s) => cells.push(s.to_string()),
    		None => cells.push(".".to_string())
    	}
    	cells.push(format!("{}", self.start()));
    	cells.push(format!("{}", self.end()));
    	match self.score() {
    		Some(s) => cells.push(format!("{}", s)),
    		None => cells.push(".".to_string())
    	}
    	match self.strand() {
    		Some(s) => match s {
    			Strand::Forward  => cells.push("+".to_string()),
    			Strand::Backward => cells.push("-".to_string())
    		},
    		None => cells.push(".".to_string())
    	}
    	match self.frame() {
    		Some(s) => cells.push(format!("{}", s)),
    		None => cells.push(".".to_string())
    	}
    	
    	let annots = util::join(self.annotations().iter().map(|x| x.to_string()).collect(), "; ");
    	cells.push(annots);

    	write!(f, "{}", util::join(cells, "\t"))
	}
}


#[cfg(test)]
mod tests {
	use io::gtf::GtfRecord;
	use std::str::FromStr;

	#[test]
	fn test_from_and_to_string(){
		let orig = "chr1\tprocessed_transcript\ttranscript\t11869\t14409\t.\t+\t.\tgene_id \"ENSG00000223972\"; transcript_id \"ENST00000456328\"; gene_name \"DDX11L1\"; gene_source \"havana\"; gene_biotype \"transcribed_unprocessed_pseudogene\"; transcript_name \"DDX11L1-002\"; transcript_source \"havana\"";
		let record = match GtfRecord::from_str(orig) {
			Ok(r) => {
				assert_eq!(r.seqname(), "chr1".to_string());
				assert_eq!(r.start(), 11869u64);
				assert_eq!(r.end(), 14409u64);
			},
			Err(e) => assert!(false, e)
		};
	}
}