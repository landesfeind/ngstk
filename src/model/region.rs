use std::cmp::Ordering;
use std::fmt;

/// A (genomic, transcriptomic, or proteomic) region is defined as a part of a
/// larger template sequence. The template sequence can either be a DNA, RNA, aminoacid
/// or any other genomic 
pub trait Region {
    
    /// Returns the name of the template for this region
    fn template(&self) -> String;

    /// Returns the offset of the region. That is the number of sequence elements
    /// that come before this region starts. 
    fn offset(&self) -> usize;

    /// Returns the length of the sequence, that is the number of 
    fn length(&self) -> usize;

    /// Returns the end position of the region. This is the last
    /// position being part of the region.
    fn end(&self) -> usize {
        self.offset() + self.length()
    }

    /// Calculate if two regions overlap
    fn overlaps<O: Region>(&self, other: &O) -> bool {
        self.overlap_length(other) > 0
    }

    /// Calculate the length of the overlap between two regions.
    fn overlap_length<O: Region>(&self, other: &O) -> usize {
        if self.template() != other.template() {
            return 0
        }

        let self_offset = self.offset();
        let self_end    = self.end();
        let other_offset = other.offset();
        let other_end    = other.end();

        if self_offset == other_offset {
            if self_end < other_end {
                self.length()
            }
            else {
                other.length()
            }
        }    
        else if self_offset < other_offset {
            if self_end < other_offset {
                0
            }
            else if self_end <= other_end {
                self_end - other_offset + 1
            }
            else {
                other.length()
            }
        }
        else if self_offset < other_end { // && self_offset > other_offset
            if self_end <= other_end {
                self.length()
            } 
            else {
                other_end - self_offset
            }
        } 
        else { // self_offset >= other_end
            0
        }
    }    

    /// Returns string that resembles the 
    fn display_string(&self) -> String {
        format!("{}:{}-{}", self.template(), self.offset()+1, self.end()+1)
    }

    /// Compare two regions by 
    ///  - templatename
    ///  - offset
    ///  - length
    /// The ordering is total.
    fn compare<O:Region>(&self, other: &O) -> Ordering {
        if self.template() != other.template() {
            self.template().cmp(&other.template())
        }
        else if self.offset() < other.offset() {
            Ordering::Less
        } 
        else if self.offset() > other.offset() {
            Ordering::Greater
        }
        else if self.length() < other.length() {
            Ordering::Less
        }
        else if self.length() > other.length() {
            Ordering::Greater
        }
        else {
            Ordering::Equal
        }
    }

    /// Compares the two regions and returns `true` if the 
    /// the two regions are identical.
    fn equal<O: Region>(&self, other: &O) -> bool {
        self.compare(other) == Ordering::Equal
    }
}


#[derive(Clone, Debug)]
pub struct SimpleRegion {
    name: String,
    offset: usize,
    length: usize,
}

impl SimpleRegion {
    pub fn new<S: ToString>(name: S, offset: usize, length: usize) -> Self {
        SimpleRegion {
            name: name.to_string(),
            offset: offset,
            length: length,
        }
    }
}

impl Region for SimpleRegion {
    
    /// Returns the name of the (template) region
    fn template(&self) -> String {
        self.name.clone()
    }

    /// Returns the offset of the region is given
    fn offset(&self) -> usize {
        self.offset
    }

    /// Returns the length of the region if given
    fn length(&self) -> usize {
        self.length
    }
}


impl fmt::Display for SimpleRegion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}-{}", self.template(), self.offset()+1, self.end() + 1 )
    }
}

#[cfg(test)]
mod tests {
    use model::region::*;

    #[test]
    fn test_simple_region(){
        let r = SimpleRegion::new("ref", 10, 5);
        assert_eq!(r.template(), "ref");
        assert_eq!(r.offset(), 10);
        assert_eq!(r.length(), 5);
    }
/*
    #[test]
    fn test_calculation_end() {
        let r = Region::new_with_coordinates(&"ref", 0, 100);
        assert_eq!(r.end(), Some(99usize), "Last inclusive element is 99");
    }

    #[test]
    fn test_equality() {
        let mut region = Region::new(&"foo");
        assert_eq!(region, region, "Identical object without coordinates");
        region = Region::new_with_coordinates(&"foo", 100, 2);
        assert_eq!(region, region, "Identical object with coordinates");
        
        assert!( Region::new(&"ref" ) == Region::new(&"ref" ), "Same name no coordinates" );
        assert!( Region::new(&"ref" ) != Region::new(&"ref2"), "Different names no coordinates" );
        assert!( Region::new(&"ref2") != Region::new(&"ref" ), "Different names no coordinates" );
    }

    #[test]
    fn test_comparisons() {
        assert_eq!(Region::new(&"ref" ).partial_cmp(&Region::new(&"ref" )), Some(Ordering::Equal));
        assert_eq!(Region::new(&"ref" ).partial_cmp(&Region::new(&"ref2")), Some(Ordering::Less));
        assert_eq!(Region::new(&"ref2").partial_cmp(&Region::new(&"ref" )), Some(Ordering::Greater));
    
        assert_eq!(Region::new_with_coordinates(&"ref" , 1, 2).partial_cmp(&Region::new(&"ref" )), Some(Ordering::Greater));
        assert_eq!(Region::new_with_coordinates(&"ref" , 1, 2).partial_cmp(&Region::new(&"ref2")), Some(Ordering::Less));
        assert_eq!(Region::new_with_coordinates(&"ref2", 1, 2).partial_cmp(&Region::new(&"ref" )), Some(Ordering::Greater));
    }


    #[test]
    fn test_overlap() {
        assert!( ! Region::new_with_coordinates(&"ref" ,  0, 10).overlaps(&Region::new_with_coordinates(&"ref", 10, 10)), "Two regions right after each other");
        assert!(   Region::new_with_coordinates(&"ref" ,  0, 11).overlaps(&Region::new_with_coordinates(&"ref", 10, 10)), "Two regions right after each other but 1bp overlap");
        assert!(   Region::new_with_coordinates(&"ref" , 10, 10).overlaps(&Region::new_with_coordinates(&"ref", 10, 10)), "First region is second region");
        assert!(   Region::new_with_coordinates(&"ref" , 11,  9).overlaps(&Region::new_with_coordinates(&"ref", 10, 10)), "First region in second region");

        assert!(   Region::new_with_coordinates(&"ref" , 19,  10).overlaps(&Region::new_with_coordinates(&"ref", 10, 10)), "Region one at end of second region but 1bp overlap");
        assert!( ! Region::new_with_coordinates(&"ref" , 20,  10).overlaps(&Region::new_with_coordinates(&"ref", 10, 10)), "Region one at end of second region no overlap");


    }


*/    
}
