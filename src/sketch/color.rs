use std::cmp::*;


/// A generalized structure for working with colors.
/// Colors are represented in the RGBA model using short values.
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Create a new color with alpha set to opaque
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self::new_with_alpha(r, g, b, 255)
    }

    /// Create a new color with an alpha value
    pub fn new_with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    pub fn with_opacity(mut self, opacity: f64) -> Self {
        if opacity <= 0f64 {
            self.a = 255u8
        } else if opacity > 1f64 {
            self.a = 0u8
        } else {
            self.a = 255u8 - (255f64 * opacity) as u8
        }
        self
    }

    pub fn has_alpha(&self) -> bool {
        self.a < 255
    }

    pub fn opacity(&self) -> f64 {
        (self.a as f64) / 255f64
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

    pub fn lighten(self) -> Self {
        self.lighten_by(10)
    }

    pub fn lighten_by(mut self, by: u8) -> Self {
        self.r = match self.r > 255 - by {
            true => 255,
            false => self.r + by,
        };
        self.g = match self.g > 255 - by {
            true => 255,
            false => self.g + by,
        };
        self.b = match self.b > 255 - by {
            true => 255,
            false => self.b + by,
        };
        self
    }

    pub fn darken(self) -> Self {
        self.lighten_by(10)
    }

    pub fn darken_by(mut self, by: u8) -> Self {
        self.r = match self.r < by {
            true => 0,
            false => self.r - by,
        };
        self.g = match self.g < by {
            true => 0,
            false => self.g - by,
        };
        self.b = match self.b < by {
            true => 0,
            false => self.b - by,
        };
        self
    }

    /// Create a new color which is completely black
    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }

    pub fn red() -> Self {
        Self::new(255, 0, 0)
    }

    pub fn yellow() -> Self {
        Self::new(255, 255, 0)
    }

    pub fn orange() -> Self {
        Self::new(255, 165, 0)
    }

    pub fn green() -> Self {
        Self::new(0, 255, 0)
    }

    pub fn blue() -> Self {
        Self::new(0, 0, 255)
    }

    pub fn light_blue() -> Self {
        Self::blue().lighten_by(80)
    }

    pub fn gray() -> Self {
        Self::new(100, 100, 100)
    }

    pub fn grey() -> Self {
        Self::gray()
    }

    pub fn transparent() -> Self {
        Self::new_with_alpha(255, 255, 255, 255)
    }
}


impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}
impl Eq for Color {}


impl PartialOrd<Color> for Color {
    fn partial_cmp(&self, other: &Color) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Color {
    fn cmp(&self, other: &Color) -> Ordering {
        if self.r != other.r {
            self.r.cmp(&other.r)
        } else if self.g != other.g {
            self.g.cmp(&other.g)
        } else {
            self.b.cmp(&other.b)
        }
    }
}
