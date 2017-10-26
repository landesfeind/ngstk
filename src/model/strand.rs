use std::str::FromStr;
use std::fmt;

#[derive(Clone,Debug)]
pub enum Strand {
	Forward, Backward
}

impl FromStr for Strand {
	type Err = String;

	fn from_str(s: &str) -> Result<Strand, String> {
		let c = s.chars().filter(|c| ! c.is_whitespace() ).nth(0usize);
		match c {
			Some('+') => Ok(Strand::Forward),
			Some('-') => Ok(Strand::Backward),
			_ => Err(format!("Can not parse '{}'", s))
		}
	}
}

impl fmt::Display for Strand {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Strand::Forward => write!(f, "+"),
			Strand::Backward => write!(f, "-")
		}
    }
}

impl PartialEq<Strand> for Strand {
	fn eq(&self, other: &Strand) -> bool {
		match *self {
			Strand::Forward => match *other {
				Strand::Forward => true,
				Strand::Backward => false
			},
			Strand::Backward => match *other {
				Strand::Forward => false,
				Strand::Backward => true
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use model::strand::Strand;

	#[test]
	pub fn parse_forward() {
		assert_eq!( "+".parse::<Strand>() , Ok(Strand::Forward));
		assert_eq!(" +".parse::<Strand>() , Ok(Strand::Forward));
		assert_eq!("+ ".parse::<Strand>() , Ok(Strand::Forward));
		assert_eq!(" + ".parse::<Strand>(), Ok(Strand::Forward));
	}

	#[test]
	pub fn parse_backward() {
		assert_eq!( "-".parse::<Strand>() , Ok(Strand::Backward));
		assert_eq!(" -".parse::<Strand>() , Ok(Strand::Backward));
		assert_eq!("- ".parse::<Strand>() , Ok(Strand::Backward));
		assert_eq!(" - ".parse::<Strand>(), Ok(Strand::Backward));
	}

}