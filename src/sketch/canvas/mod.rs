use std::io::Write;
pub mod svg;


pub use self::svg::Svg;
use sketch::Color;

pub trait Canvas: Default {

    fn with_image_width(self, new_width: f64) -> Self;

    fn image_width(&self) -> f64;
    
    fn with_image_height(self, new_width: f64) -> Self;
    
    fn image_height(&self) -> f64;

    fn write<W: Write>(&self, out: W);

    fn draw_text<S: ToString>(
        &self,
        text: S,
        pos_x: f64,
        pos_y: f64,
        font_size: f64,
        font_weight: &str,
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

    fn draw_circ(&mut self, pos_cx: f64, pos_cy: f64, radius: f64, fill_color: Option<Color>);


    fn draw_line(
        &mut self,
        pos_x1: f64,
        pos_y1: f64,
        pos_x2: f64,
        pos_y2: f64,
        color: Option<Color>,
    );

}
