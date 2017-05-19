use std::fmt;
use std::str::FromStr;

#[derive(Clone,Debug)]
pub struct Region {
    name: String,
    offset: Option<usize>,
    length: Option<usize>
}

impl Region {

    pub fn new(name: String) -> Self {
        Region {
            name: name,
            offset: None,
            length: None
        }
    }
    pub fn new_with_coordinates(name: String, offset: usize, length: usize) -> Self {
        Region {
            name: name,
            offset: Some(offset),
            length: Some(length)
        }
    }

    pub fn name(&self) -> String  { self.name.clone() }

    pub fn has_coordinates(&self) -> bool {
        self.offset.is_some() 
            && self.length.is_some()
    }

    pub fn offset(&self) -> Option<usize> { self.offset }

    pub fn length(&self) -> Option<usize> { self.length }

    pub fn end(&self) -> Option<usize> { 
        if self.has_coordinates() {
            Some( self.offset.unwrap() + self.length.unwrap() )
        } else {
            None
        }
    }

}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.has_coordinates() {
            write!(f, "{}", self.name)
        }
        else {
            write!(f, "{}:{}-{}", self.name, self.offset.unwrap() + 1, self.offset.unwrap() + self.length.unwrap())
        }
    }
}

impl FromStr for Region {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split : Vec<&str> = s.split(|c:char| c == ':' || c == '-').collect();
        if split.len() == 1 {
            Ok(Region::new(split[0].to_string()))
        }
        else if split.len() == 2 {
            match usize::from_str(split[1]) {
                Ok(pos) => Ok(Region::new_with_coordinates(split[0].to_string(), pos-1, usize::max_value() - pos - 1)),
                Err(e) => Err(format!("Can not parse coordinate: {}", e))
            }
        }
        else if split.len() == 3 {
            match usize::from_str(split[1]) {
                Ok(pos1) => match usize::from_str(split[2]) {
                    Ok(pos2) => match pos1 <= pos2 {
                        true  => Ok(Region::new_with_coordinates(split[0].to_string(), pos1 - 1, pos2 - pos1 + 1)),
                        false => Err(format!("Second coordinate '{}' is lower than first '{}'", pos2, pos1)),
                    },
                    Err(e) => Err(format!("Can not parse second coordinate: {}", e))
                },
                Err(e) => Err(format!("Can not parse first coordinate: {}", e))
            }
        }
        else {
            Err(format!("Splitting region string does not return correct number of split elements: {:?}", split))
        }
    }
}



#[cfg(test)]
mod tests {
    use region::*;

    #[test]
    fn test_parse_single_coord(){
        let r = Region::from_str("chr1:100").expect("Can not parse");
        assert_eq!(r.name(), "chr1");
        assert_eq!(r.offset(), 99usize);
    }

    #[test]
    fn test_parse_two_coord(){
        let r = Region::from_str("chr1:100-200").expect("Can not parse");
        assert_eq!(r.name(), "chr1");
        assert_eq!(r.offset(), 99usize);
        assert_eq!(r.length(), 101usize);
    }

}

