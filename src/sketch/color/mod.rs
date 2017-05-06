use sequence::dna::*;
use sequence::rna::*;
use sketch::scale::Scale;

#[derive(Clone,Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Color {
    
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color::new_with_alpha(r, g, b, 255)
    }

    pub fn new_with_alpha(r: u8, g: u8, b: u8, a:u8) -> Self {
        Color { r: r, g: g, b: b, a: a }
    }

    pub fn black() -> Self {
        Color::new(0, 0, 0)
    }

    pub fn white() -> Self {
        Color::new(255, 255, 255)
    }

    pub fn red() -> Self {
        Color::new(255, 0, 0)
    }

    pub fn green() -> Self {
        Color::new(0, 255, 0)
    }

    pub fn blue() -> Self {
        Color::new(0, 0, 255)
    }

    pub fn light_blue() -> Self {
        Color::new(80, 80, 255)
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn to_hexa(&self) -> String {
        format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
    }

    pub fn to_rgb(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    pub fn to_rgba(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }

    pub fn has_alpha(&self) -> bool {
        self.a < 255
    }

    pub fn opacity(&self) -> f64 {
        (self.a as f64) / 255f64
    }
}


pub fn insertion() -> Color {
    Color::new(197,90,159)
}

pub fn deletion() -> Color {
    Color::black()
}

pub fn border() -> Color {
    Color::new(200,200,200)
}

pub fn clip() -> Color {
    Color::red()
}

pub struct SequenceColors {}


impl Scale<DnaNucleotide, Color> for SequenceColors {
    fn scale(&self, e: DnaNucleotide) -> Color {
        match e { 
            DnaNucleotide::A => Color::new(91,169,101),
            DnaNucleotide::C => Color::new(119,122,205),
            DnaNucleotide::G => Color::new(173,150,61),
            DnaNucleotide::T => Color::new(202,94,74),
            _ => Color::new(100, 100, 100)
        }
    }
}

impl Scale<RnaNucleotide, Color> for SequenceColors {
    fn scale(&self, e: RnaNucleotide) -> Color {
        match e { 
            RnaNucleotide::A => Color::new(91,169,101),
            RnaNucleotide::C => Color::new(119,122,205),
            RnaNucleotide::G => Color::new(173,150,61),
            RnaNucleotide::U => Color::new(202,94,74),
            _ => Color::new(100, 100, 100)
        }
    }
}

impl Default for SequenceColors {
    fn default() -> SequenceColors {
        SequenceColors {} 
    }
}
