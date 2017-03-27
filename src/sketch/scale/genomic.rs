use sketch::scale::Scale;
use sketch::color::Color;

use dna::*;
use region::*;
use std::marker::PhantomData;

/// A sequence scale scales a defined region into 
/// a coordinate.
#[derive(Clone,Debug)]
pub struct SequenceScale<I : RegionIdentifier, E : SequenceElement, R : Region<I, E>> {
    domain: R,
    elem_width: f64,
    _marker_I: PhantomData<I>,
    _marker_E: PhantomData<E>,
}

impl<I : RegionIdentifier, E : SequenceElement, R : Region<I, E>> SequenceScale<I, E, R> {

    /// Create a new scale using a default
    /// width per element.
    pub fn new(domain: R) -> Self {
        Self::new_with_element_width(domain, 15f64)
    }

    /// Create a new scale using a the given per element width.
    pub fn new_with_element_width(domain: R, elem_width:f64) -> Self {
        SequenceScale {
            domain: domain,
            elem_width: elem_width
        }
    }

    /// Create a new scale that has a maximum width/height of `max`.
    pub fn new_with_max_width(domain: R, max:f64) -> Self {
        let w = max / (domain.length() as f64);
        Self::new_with_element_width(domain, w)
    }

}

impl<I : RegionIdentifier, E : SequenceElement, R : Region<I, E>> Scale<usize, f64> for SequenceScale<I, E, R> {
    fn scale(&self, d: usize) -> f64 {
        ((d - self.domain.offset()) as f64) * self.elem_width
    }
}


/// A scale that maps a nucleotide into a color
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
        let s = GenomicScale::new_with_element_width(gr, 1f64);
        assert_eq!( s.scale(   0usize ),   0f64);
        assert_eq!( s.scale(   1usize ),   1f64);
        assert_eq!( s.scale(  10usize ),  10f64);
        assert_eq!( s.scale( 100usize ), 100f64);
    }

}



