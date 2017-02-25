use std::result;
use std::fmt;
use std::str::FromStr;

use genomicregion::GenomicRegion;

/// Implements a genomic range determined by a genomic 
/// reference name (e.g., a specific chromosome), a offset
/// and an end. The offset end 
#[derive(Clone,Debug)]
pub struct GenomicRange {
    refname: String, 
    offset: usize, 
    length: usize
}

impl GenomicRange {
    /// Create a new genomic range
    ///
    /// # Arguments
    ///
    /// * `refname` - the name of the reference template sequence
    /// * `offset` - the first position that is part of the genomic range (indexing starts at 0)
    /// * `length` - the length of the genomic range
    ///
    pub fn new(refname: &str, offset: usize, length: usize) -> GenomicRange {
        GenomicRange { refname: refname.to_string(), offset: offset, length: length }
    }

    /// Returns the reference sequence name
    pub fn refname(&self) -> &str {
        self.refname.as_str()
    }

    /// Returns the offset position of the genomic range (indexing starts with 0, inclusive)
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Returns the end position of the genomic range (indexing starts with 0, exclusive)
    pub fn end(&self) -> usize{
        self.offset + self.length
    }
        
    pub fn length(&self) -> usize {
        self.length
    }


    /// Intersects two genomic regions
    pub fn intersect(&self, other: &GenomicRange) -> Option<GenomicRange> {
        if self.refname() != other.refname() {
            return None
        }

        let s = match self.offset() > other.offset() {
                true => self.offset(),
                false => other.offset()
            };
        let e = match self.end() < other.end() {
                true => self.end(),
                false => other.end()
            };

        if e < s {
            return None
        }
        
        Some( GenomicRange::new(self.refname().clone(), s, e) )
    }

}

impl fmt::Display for GenomicRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}-{}", self.refname, self.offset + 1, self.offset + self.length + 1)
    }
}


impl<'a> From<&'a GenomicRegion> for GenomicRange {
    fn from(gr: &GenomicRegion) -> GenomicRange {
        GenomicRange { refname: gr.refname().to_string(), offset: gr.offset(), length: gr.length() }
    }
}


impl FromStr for GenomicRange {
    type Err = &'static str;

    fn from_str(s: &str) -> result::Result<GenomicRange,Self::Err> { 
        let mut iter = s.split(|c| c == ':' || c == '-' );
        
        let chr = match iter.next() {
            None => return Err("Can not extract reference name"),
            Some(s) => s.to_string()
        };

        let offset = match iter.next() {
            None => return Err("Can not extract offset position"),
            Some(s) => match s.parse::<usize>() {
                    Err(_) => return Err("Can not parse offset position"),
                    Ok(v) => match v > 0 {
                        false => return Err("Start position must be 1 or larger"),
                        true => v
                    }
                }
        
        };

        let end = match iter.next() {
            None => return Err("Can not extract end position"),
            Some(s) => match s.parse::<usize>() {
                    Err(_) => return Err("Can not parse end position"),
                    Ok(v) => match v >= offset {
                        false => return Err("End position must be greater or equal start position"),
                        true => v
                    }
                }
        
        };

        Ok( GenomicRange::new(chr.as_ref(), offset-1, end-offset+1) )
    }
}



#[cfg(test)]
mod tests {

    use genomicrange::*;

    #[test]
    fn test_string_parse_1(){
        let rg : Result<GenomicRange, &str> = "chr1:1232-1235".parse();
        assert!(rg.is_ok());
        let g = rg.unwrap();
        assert_eq!(g.refname(), "chr1");
        assert_eq!(g.offset(), 1231);
        assert_eq!(g.length(), 4);
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
        assert_eq!(g.offset(), 0);
        assert_eq!(g.end(), 1);
        assert_eq!(g.length(), 1);
    }
}
