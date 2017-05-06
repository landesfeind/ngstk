use std::fmt;
use std::str::FromStr;

#[derive(Clone,Debug)]
pub struct Region {
    name: String,
    offset: usize,
    length: usize
}

impl Region {

    pub fn new(name: String, offset: usize, length: usize) -> Self {
        Region {
            name: name,
            offset: offset,
            length: length
        }
    }

    pub fn name(&self) -> String  { self.name.clone() }

    pub fn offset(&self) -> usize { self.offset }

    pub fn length(&self) -> usize { self.length }

}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}-{}", self.name, self.offset + 1, self.offset + self.length)
    }
}

impl FromStr for Region {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split : Vec<&str> = s.split(|c:char| c == ':' || c == '-').collect();
        if split.len() == 1 {
            Ok(Region::new(split[0].to_string(), 0, usize::max_value()-1))
        }
        else if split.len() == 2 {
            match usize::from_str(split[1]) {
                Ok(pos) => Ok(Region::new(split[0].to_string(), pos-1, 1)),
                Err(e) => Err(format!("Can not parse coordinate: {}", e))
            }
        }
        else if split.len() == 3 {
            match usize::from_str(split[1]) {
                Ok(pos1) => match usize::from_str(split[2]) {
                    Ok(pos2) => match pos1 <= pos2 {
                        true  => Ok(Region::new(split[0].to_string(), pos1 - 1, pos2 - pos1 + 1)),
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
        assert_eq!(r.length(), 1usize);
    }

    #[test]
    fn test_parse_two_coord(){
        let r = Region::from_str("chr1:100-200").expect("Can not parse");
        assert_eq!(r.name(), "chr1");
        assert_eq!(r.offset(), 99usize);
        assert_eq!(r.length(), 101usize);
    }

}

