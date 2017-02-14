use std::result;
use std::fmt;
use std::str::FromStr;
use std::error::Error;

/// Implements a genomic range determined by a genomic 
/// reference name (e.g., a specific chromosome), a start
/// and an end. The start end 
#[derive(Clone,Debug)]
pub struct GenomicRange {
    refname: String, 
    start: usize, 
    end: usize
}

impl GenomicRange {

    /// Create a new genomic range
    ///
    /// # Arguments
    ///
    /// * `refname` - the name of the reference template sequence
    /// * `start` - the first position that is part of the genomic range (indexing starts at 0)
    /// * `end` - the last position that is part of the genomic range (indexing starts at 0). Must
    /// be greater or equal than `start`
    ///
    /// # Panic
    ///
    /// Fails if `start` is larger than `end`.
    ///
    pub fn new(refname: &str, start: usize, end: usize) -> GenomicRange {
        assert!(start <= end);
        GenomicRange { refname: refname.to_string(), start: start, end: end }
    }

    /// Returns the reference sequence name
    pub fn refname(&self) -> &str {
        self.refname.as_ref()
    }

    /// Returns the start position of the genomic range
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the end position of the genomic range
    pub fn end(&self) -> usize {
        self.end
    }

    pub fn length(&self) -> usize {
        self.end - self.start + 1
    }
}

impl fmt::Display for GenomicRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}-{}", self.refname, self.start + 1, self.end + 1)
    }
}

impl FromStr for GenomicRange {
    type Err = &'static str;

    fn from_str(s: &str) -> result::Result<Self,Self::Err> { 
        let mut iter = s.split(|c| c == ':' || c == '-' );
        
        let chr = match iter.next() {
            None => return Err("Can not extract reference name"),
            Some(s) => s.to_string()
        };

        let start = match iter.next() {
            None => return Err("Can not extract start position"),
            Some(s) => match s.parse::<usize>() {
                    Err(e) => return Err("Can not parse start position"),
                    Ok(v) => match v > 0 {
                        false => return Err("Start position must be 1 or larger"),
                        true => v
                    }
                }
        
        };

        let end = match iter.next() {
            None => return Err("Can not extract end position"),
            Some(s) => match s.parse::<usize>() {
                    Err(e) => return Err("Can not parse end position"),
                    Ok(v) => match v >= start {
                        false => return Err("End position must be greater or equal start position"),
                        true => v
                    }
                }
        
        };

        Ok( GenomicRange::new(chr.as_ref(), start-1, end-1) )
    }
}



#[cfg(test)]
mod tests {

    use data::genomicrange::*;

    #[test]
    fn test_string_parse_1(){
        let rg : Result<GenomicRange, &str> = "chr1:1232-121144".parse();
        assert!(rg.is_ok());
        let g = rg.unwrap();
        assert_eq!(g.refname(), "chr1");
        assert_eq!(g.start(), 1231);
        assert_eq!(g.end(), 121143);
    }

    #[test]
    fn test_string_parse_2(){
        let rg : Result<GenomicRange, &str> = "chr1:0-123".parse();
        assert!(! rg.is_ok());
    }

  #[test]
    fn test_string_parse_3(){
        let rg : Result<GenomicRange, &str> = "chr1:1-1".parse();
        assert!(rg.is_ok());
        let g = rg.unwrap();
        assert_eq!(g.refname(), "chr1");
        assert_eq!(g.start(), 0);
        assert_eq!(g.end(), 0);
        assert_eq!(g.length(), 1);
    }

}
