use std::io::Write;
pub mod svg;

use model::*;
pub use self::svg::Svg;
use sketch::Color;

pub enum DrawOperation {
    MoveTo(f64, f64),
    LineTo(f64, f64),
}

pub trait Canvas {
    type Viewport : Region;

    fn new(v: Self::Viewport) -> Self;
    
    fn with_viewport(self, viewport: Self::Viewport) -> Self;
    fn viewport(&self) -> Self::Viewport;

    fn with_image_width(self, new_width: f64) -> Self;
    fn image_width(&self) -> f64;


    fn with_image_height(self, new_width: f64) -> Self;
    fn image_height(&self) -> f64;

    fn write<W: Write>(&self, out: W);

    fn bandwidth(&self) -> f64 {
        self.image_width() / self.viewport().length() as f64
    }

    fn scale_position_x<R: Region>(&self, r: &R) -> Option<(f64, f64)> {
        if self.viewport().overlaps(r) {
            let offset = (r.offset() as f64 - self.viewport().offset() as f64 ) * self.bandwidth();
            let length = offset + self.bandwidth() * r.length() as f64;
            Some((offset, offset + length))
        }
        else {
            None
        }
    }

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

    fn draw_path(&mut self, path: Vec<DrawOperation>, stroke_color: Option<Color>, fill_color: Option<Color>);
}
