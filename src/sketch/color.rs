

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

    pub fn red() -> Self {
        Color::new(255, 0, 0)
    }

    pub fn green() -> Self {
        Color::new(0, 255, 0)
    }

    pub fn blue() -> Self {
        Color::new(0, 0, 255)
    }

    pub fn to_hex(&self) -> String {
        format!("#{:x}{:x}{:x}{:x}", self.r, self.g, self.b, self.a)
    }

    pub fn to_rgb(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    pub fn to_rgba(&self) -> String {
        format!("rgb({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

