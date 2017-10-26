pub mod matrix;

pub fn split<S: ToString, P: ToString>(record: S, cell_separator: P) -> Vec<String> {
    record
        .to_string()
        .split(&cell_separator.to_string())
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

pub fn join<S1: ToString, S2: ToString>(l: Vec<S1>, sep: S2) -> String {
    l.iter().fold("".to_string(), |a, b| if a.len() > 0 {
        a + sep.to_string().as_ref()
    } else {
        a
    } + b.to_string().as_ref())
}


/// Parses a region identifer string in the form of
/// ```
///  template_name:position
///  template_name:start.position-end.postion
/// ```
/// Thereby, the start and end region 
pub fn parse_region_string<S: ToString>(s: S) -> Result<(String, usize, usize), String> {
    let parts = s.to_string()
        .split(|c| c == ':' || c == '-')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let name = parts[0].to_string();

    let offset = match parts.len() > 1 {
        true => match parts[1].parse::<usize>() {
            Ok(o) => o - 1,
            Err(e) => return Err(format!("Can not parse start part '{}'", parts[1]))
        }, 
        false => 0
    };

    let length = match parts.len() > 2 {
        true  => match parts[2].parse::<usize>() {
            Ok(end) => match offset < end {
                true => end - offset,
                false => return Err(format!("Region end after start '{}'", parts[2])),
            },
            Err(e) => return Err(format!("Can not parse start part '{}'", parts[2]))
        },
        false => 1usize
    };

    Ok((name, offset, length))
}




#[cfg(test)]
mod tests {
    use util::parse_region_string;

    #[test]
    fn test_parse_region_string_single_coordinate() {
        let (r,o,l) = parse_region_string("chr1:100").expect("Can not parse");
        assert_eq!(r, "chr1", "Correct template name");
        assert_eq!(o, 99    , "Correct offset");
        assert_eq!(l,  1    , "Correct length");
    }

    #[test]
    fn test_parse_region_string_two_coordinate() {
        let (r,o,l) = parse_region_string("chr1:200-300").expect("Can not parse");
        assert_eq!(r, "chr1", "Correct template name");
        assert_eq!(o,    199, "Correct offset");
        assert_eq!(l,    101, "Correct length");
    }

    #[test]
    fn test_parse_region_string_end_equal_start() {
        let (r,o,l) = parse_region_string("chr1:200-200").expect("Can not parse");
        assert_eq!(r, "chr1", "Correct template name");
        assert_eq!(o,    199, "Correct offset");
        assert_eq!(l,      1, "Correct length");
    }

    #[test]
    fn test_parse_region_string_end_before_start() {
        assert!( parse_region_string("chr1:200-199").is_err() );
    }
}
