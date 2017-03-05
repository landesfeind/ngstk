use sketch::scale::Scale;
use sketch::color::Color;

use genomicrange::GenomicRange;
use dna::*;


#[derive(Clone,Debug)]
pub struct GenomicScale {
    domain: GenomicRange,
    nuc_width: f64
}

impl GenomicScale {
    pub fn new(domain: GenomicRange) -> Self {
        Self::new_with_nuc_width(domain, 15f64)
    }

    pub fn new_with_nuc_width(domain: GenomicRange, nuc_width:f64) -> Self {
        GenomicScale {
            domain: domain,
            nuc_width: nuc_width
        }
    }

    pub fn new_with_max_width(domain: GenomicRange, max_width:f64) -> Self {
        let w = max_width / (domain.length() as f64);
        Self::new_with_nuc_width(domain, w)
    }

}

impl Scale<usize, f64> for GenomicScale {
    fn scale(&self, d: usize) -> f64 {
        ((d - self.domain.offset()) as f64) * self.nuc_width
    }
}

#[derive(Clone,Debug)]
pub struct NucleotideColorScale {}

impl NucleotideColorScale {
    pub fn new() -> Self {
        NucleotideColorScale {}
    }
}

impl Scale<DnaNucleotide, Color> for NucleotideColorScale {
    fn scale(&self, d: DnaNucleotide) -> Color {
        self.scale(&d)
    }
}


impl<'a> Scale<&'a DnaNucleotide, Color> for NucleotideColorScale {
    fn scale(&self, d: &DnaNucleotide) -> Color {
        match *d {
            DnaNucleotide::A => Color::new(235u8,176u8,164u8),
            DnaNucleotide::C => Color::new(129u8,213u8,221u8),
            DnaNucleotide::G => Color::new(198u8,182u8,229u8),
            DnaNucleotide::T => Color::new(192u8,213u8,165u8),
            _ => Color::new(125u8,125u8,125u8)
        }
    }
}



#[cfg(test)]
mod test {

    use genomicrange::GenomicRange;
    use sketch::scale::Scale;
    use sketch::scale::genomic::GenomicScale;
    
    #[test]
    fn test_simple_scale() {
        let gr = GenomicRange::new(&"chr", 0, 100);
        let s = GenomicScale::new_with_nuc_width(gr, 1f64);
        assert_eq!( s.scale(   0usize ),   0f64);
        assert_eq!( s.scale(   1usize ),   1f64);
        assert_eq!( s.scale(  10usize ),  10f64);
        assert_eq!( s.scale( 100usize ), 100f64);
    }

}



