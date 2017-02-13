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
        assert!(start > end);
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

}

impl fmt::Display for GenomicRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}-{}", self.refname, self.start + 1, self.end + 1)
    }
}

#[derive(Clone,Debug)]
pub struct ParseGenomicRangeError (String);

impl Error for ParseGenomicRangeError {
    fn description(&self) -> &str {
        self.0.as_str()
    }
}
impl fmt::Display for ParseGenomicRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl FromStr for GenomicRange {
    type Err = ParseGenomicRangeError;

    fn from_str(s: &str) -> result::Result<Self,Self::Err> { 
        let v1 : Vec<&str> = s.split(":").collect();
        if v1.len() != 2 {
            return Err( ParseGenomicRangeError (String::from("Can not parse genomic range string: can not extract reference name")) )
        }
        let v2 : Vec<str> = v1[1].split("-").collect();
        if v2.len() != 2 {
            return Err( ParseGenomicRangeError (String::from("Can not parse genomic range string: need two positions ")) )
        }
        
        let v3 : Vec<usize> = v2.iter().map( |n| n.parse::<usize>().unwrap() ).collect();
        if v3[0] == 0 || v3[1] == 0 {
            return Err( ParseGenomicRangeError (String::from("Can not parse genomic range string: position must be larger than zero")) )
        }
        if v3[0] < v3[1] {
            return Err( ParseGenomicRangeError (String::from("Can not parse genomic range string: fist position must be lower than second position")) )
        }

        Ok( GenomicRange { refname: v1[0].to_string(), start: v3[0]-1, end: v3[1]-1 } )
    }
}



#[cfg(test)]
mod tests {

    use data::genomicrange::*;

    #[test]
    fn test_string_parse_1(){
        let rg : Result<GenomicRange, ParseGenomicRangeError> = "chr1:1232-121144".parse();
        assert!(rg.is_ok());
        let g = rg.unwrap();
        assert_eq!(g.refname(), "chr1");
        assert_eq!(g.start(), 1231);
        assert_eq!(g.end(), 121143);
    }

    #[test]
    fn test_string_parse_2(){
        let rg : Result<GenomicRange, ParseGenomicRangeError> = "chr1:0-123".parse();
        assert!(! rg.is_ok());
    }

}
