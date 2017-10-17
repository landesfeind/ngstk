use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Region {
    name: String,
    offset: Option<usize>,
    length: Option<usize>,
}

impl Region {
    pub fn new<S: ToString>(name: S) -> Self {
        Region {
            name: name.to_string(),
            offset: None,
            length: None,
        }
    }
    pub fn new_with_coordinates<S: ToString>(name: S, offset: usize, length: usize) -> Self {
        Region {
            name: name.to_string(),
            offset: Some(offset),
            length: Some(length),
        }
    }

    pub fn with_coordinates(&self, offset: usize, length: usize) -> Self {
        Self::new_with_coordinates(self.name(), offset, length)
    }

    /// Returns the name of the (template) region
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns true if the region has start and end coordinates
    pub fn has_coordinates(&self) -> bool {
        self.offset.is_some() && self.length.is_some()
    }

    /// Returns the offset of the region is given
    pub fn offset(&self) -> Option<usize> {
        self.offset
    }

    /// Returns the length of the region if given
    pub fn length(&self) -> Option<usize> {
        self.length
    }

    /// Returns the end position of the region. This is the last
    /// position being part of the region.
    pub fn end(&self) -> Option<usize> {
        if self.has_coordinates() {
            Some(self.offset.unwrap() + self.length.unwrap() - 1usize)
        } else {
            None
        }
    }

    pub fn overlaps<O: Into<Region>>(&self, other: O) -> bool {
        let other_region : Region = other.into();
        if self.name() != other_region.name() {
            return false
        }

        if ! self.has_coordinates() || ! other_region.has_coordinates() {
            return true;
        }


        let self_offset = self.offset().unwrap();
        let self_end    = self.end().unwrap();
        let other_offset = other_region.offset().unwrap();
        let other_end    = other_region.end().unwrap();

        if self_offset < other_offset {
            self_end >= other_offset
        }
        else {
            self_offset <= other_end   
        }

    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.offset() {
            None => write!(f, "{}", self.name),
            Some(offset) => {
                match self.end() {
                    None => write!(f, "{}:{}", self.name, offset),
                    Some(end) => write!(f, "{}:{}-{}", self.name, offset, end),
                }
            }
        }
    }
}

impl FromStr for Region {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(|c: char| c == ':' || c == '-').collect();
        if split.len() == 1 {
            Ok(Region::new(split[0].to_string()))
        } else if split.len() == 2 {
            match usize::from_str(split[1]) {
                Ok(pos) => {
                    Ok(Region::new_with_coordinates(
                        split[0].to_string(),
                        pos - 1,
                        usize::max_value() - pos - 1,
                    ))
                }
                Err(e) => Err(format!("Can not parse coordinate: {}", e)),
            }
        } else if split.len() == 3 {
            match usize::from_str(split[1]) {
                Ok(pos1) => {
                    match usize::from_str(split[2]) {
                        Ok(pos2) => {
                            match pos1 <= pos2 {
                                true => {
                                    Ok(Region::new_with_coordinates(
                                        split[0].to_string(),
                                        pos1 - 1,
                                        pos2 - pos1 + 1,
                                    ))
                                }
                                false => {
                                    Err(format!(
                                        "Second coordinate '{}' is lower than first '{}'",
                                        pos2,
                                        pos1
                                    ))
                                }
                            }
                        }
                        Err(e) => Err(format!("Can not parse second coordinate: {}", e)),
                    }
                }
                Err(e) => Err(format!("Can not parse first coordinate: {}", e)),
            }
        } else {
            Err(format!(
                "Splitting region string does not return correct number of split elements: {:?}",
                split
            ))
        }
    }
}


impl<'a> From<&'a Region> for Region {
    fn from(r: &'a Region) -> Region {
        r.clone()
    }
}


impl PartialEq for Region {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl Eq for Region {}

impl PartialOrd<Self> for Region {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // They have different names, so sort by name
        if self.name() != other.name() {
            return self.name().partial_cmp(&other.name())
        }

        // They both have same name but no coordinates -> consider them equal
        if ! self.has_coordinates() && ! other.has_coordinates() {
            return Some(Ordering::Equal)
        }
        else if ! self.has_coordinates() {
            return Some(Ordering::Less)
        }
        else if ! other.has_coordinates() {
            return Some(Ordering::Greater)
        }
        else {
            if self.offset() != other.offset() {
                return self.offset.partial_cmp(&other.offset());
            }
            else {
                return self.length().partial_cmp(&other.length())
            }
        }
    }
}

impl Ord for Region {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}



#[cfg(test)]
mod tests {
    use region::*;
    use std::cmp::Ordering;

    #[test]
    fn test_parse_single_coord() {
        let r = Region::from_str("chr1:100").expect("Can not parse");
        assert_eq!(r.name(), "chr1");
        assert_eq!(r.offset(), Some(99usize));
    }

    #[test]
    fn test_parse_two_coord() {
        let r = Region::from_str("chr1:100-200").expect("Can not parse");
        assert_eq!(r.name(), "chr1");
        assert_eq!(r.offset(), Some(99usize));
        assert_eq!(r.length(), Some(101usize));
    }

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


    
}
