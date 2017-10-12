use sketch::Scale;
use sketch::Color;
use sequence::{RnaNucleotide,DnaNucleotide};

/// Scale for transforming DnaNucleotide and RnaNucleotide into a Color
pub struct DnaNucleotideColorScale {}

impl Scale<DnaNucleotide, Color> for DnaNucleotideColorScale {
    fn scale(&self, e: DnaNucleotide) -> Color {
        match e { 
            DnaNucleotide::A => Color::new(91, 169, 101),
            DnaNucleotide::C => Color::new(119, 122, 205),
            DnaNucleotide::G => Color::new(173, 150, 61),
            DnaNucleotide::T => Color::new(202, 94, 74),
            _ => Color::new(100, 100, 100),
        }
    }
}

impl Default for DnaNucleotideColorScale {
    fn default() -> DnaNucleotideColorScale {
        DnaNucleotideColorScale {}
    }
}

/// Scale for transforming DnaNucleotide and RnaNucleotide into a Color
pub struct RnaNucleotideColorScale {}

impl Scale<RnaNucleotide, Color> for RnaNucleotideColorScale {
    fn scale(&self, e: RnaNucleotide) -> Color {
        match e { 
            RnaNucleotide::A => Color::new(91, 169, 101),
            RnaNucleotide::C => Color::new(119, 122, 205),
            RnaNucleotide::G => Color::new(173, 150, 61),
            RnaNucleotide::U => Color::new(202, 94, 74),
            _ => Color::new(100, 100, 100),
        }
    }
}

impl Default for RnaNucleotideColorScale {
    fn default() -> RnaNucleotideColorScale {
        RnaNucleotideColorScale {}
    }
}