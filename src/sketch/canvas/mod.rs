use std::io::Write;
pub mod svg;


pub use self::svg::Svg;
use sketch::Color;

pub trait Canvas : Default {

    fn write<W:Write>(&self, out: W);

    fn draw_text<S: ToString>(
        &self,
        text: S,
        pos_x: f64,
        pos_y: f64,
        font_size: u64,
        align_center: bool,
        valign_center: bool,
        color: Option<Color>,
    );

    fn draw_rect(
        &mut self,
        pos_x: f64,
        pos_y: f64,
        width: f64,
        height: f64,
        fill_color: Option<Color>,
    );
}
