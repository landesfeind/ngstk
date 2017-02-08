
pub struct Chromosome {
    name: String,
    length: Option<u64>
}

impl Chromosome {

    pub fn new(name: &str) -> Self {
        return Chromosome { name: name.to_string(), length: None };
    }

}

