extern crate svgdom;
pub use self::svgdom::*;

pub use sketch::color::Color;

pub fn set_color(n: &mut Node, co: Option<Color>){
    if co.is_some() {
        let c = co.expect("Can not extract color from option");
        n.set_attribute(AttributeId::Color, c.to_hex());
        n.set_attribute(AttributeId::Opacity, c.opacity());
    }
}

pub fn set_fill_color(n: &mut Node, co: Option<Color>){
    if co.is_some() {
        let c = co.expect("Can not extract color from option");
        n.set_attribute(AttributeId::Fill, c.to_hex());
        n.set_attribute(AttributeId::FillOpacity, c.opacity());
    }
}


pub fn set_stroke(n: &mut Node, co: Option<Color>, w: Option<usize>){
    if co.is_some() {
        let c = co.expect("Can not extract color from option");
        n.set_attribute(AttributeId::Stroke, c.to_hex());
        n.set_attribute(AttributeId::StrokeOpacity, c.opacity());
    }
    if w.is_some() {
        n.set_attribute(AttributeId::StrokeWidth, format!("{}", w.unwrap()));
    }
}

pub fn draw_text(document: &mut Document, text: &str, pos_x: f64, pos_y: f64, font_size: usize, align_center: bool, valign_center: bool, color: Option<Color>) -> Node {
    let mut text_node = document.create_element(ElementId::Text);
    let mut data_node = document.create_node(NodeType::Text, text);
    text_node.append(&data_node);

    text_node.set_attribute(AttributeId::X, pos_x);
    text_node.set_attribute(AttributeId::Y, pos_y);

    text_node.set_attribute(AttributeId::FontSize, format!("{}px", font_size));
    if align_center {
        text_node.set_attribute(AttributeId::TextAnchor, "middle");
    }
    else {
        text_node.set_attribute(AttributeId::TextAnchor, "left");
    }

    text_node.set_attribute(AttributeId::DominantBaseline, "bottom");
    if valign_center {
        text_node.set_attribute(AttributeId::Y , pos_y - (font_size as f64 / 2f64));
        text_node.set_attribute(AttributeId::Dy,          font_size as f64 / 3f64 );
    }

    set_fill_color(&mut text_node, color);

    text_node
}

pub fn draw_path(document: &mut Document, path: svgdom::types::path::Path, stroke_color: Option<Color>, fill_color: Option<Color>) -> Node {
    let mut pn = document.create_element(ElementId::Path);
    pn.set_attribute(AttributeId::D, path);
    set_stroke(&mut pn, stroke_color, Some(1));
    set_fill_color(&mut pn, fill_color);
    pn
}

pub fn draw_line(document: &mut Document, pos_x1: f64, pos_y1: f64, pos_x2: f64, pos_y2: f64, color: Option<Color>) -> Node {
    let mut rect = document.create_element(ElementId::Line);
    rect.set_attribute(AttributeId::X1, pos_x1);
    rect.set_attribute(AttributeId::Y1, pos_y1);
    rect.set_attribute(AttributeId::X2, pos_x2);
    rect.set_attribute(AttributeId::Y2, pos_y2);
    set_stroke(&mut rect, color, Some(1));
    rect
}

pub fn draw_rect(document: &mut Document, pos_x: f64, pos_y: f64, width: f64, height: f64, fill_color: Option<Color>) -> Node {
    let mut rect = document.create_element(ElementId::Rect);
    rect.set_attribute(AttributeId::Fill, "none");
    rect.set_attribute(AttributeId::Stroke, "black");
    rect.set_attribute(AttributeId::StrokeWidth, "1");
    rect.set_attribute(AttributeId::X, pos_x);
    rect.set_attribute(AttributeId::Y, pos_y);
    rect.set_attribute(AttributeId::Width, width);
    rect.set_attribute(AttributeId::Height, height);
    set_fill_color(&mut rect, fill_color);
    rect
}

pub fn draw_circ(document: &mut Document, pos_x: f64, pos_y: f64, radius: f64, fill_color: Option<Color>) -> Node {
    let mut circ = document.create_element(ElementId::Circle);
    circ.set_attribute(AttributeId::Fill, "none");
    circ.set_attribute(AttributeId::Stroke, "black");
    circ.set_attribute(AttributeId::StrokeWidth, "1");
    circ.set_attribute(AttributeId::X, pos_x);
    circ.set_attribute(AttributeId::Y, pos_y);
    circ.set_attribute(AttributeId::R, radius);
    set_fill_color(&mut circ, fill_color);
    circ
}

//fn draw_dnasequence(&self, document: &mut Document, seq: &DnaSequence, pos_x: f64, pos_y: f64, width: f64, height: f64) -> Node {
//    let per_nuc_width = width / ((seq.length() + 1) as f64);
//    let seq_node = document.create_element(ElementId::G);
//
//    let cscale = NucleotideColorScale::new();
//
//    for (i, nuc) in seq.iter().enumerate() {
//        let x = pos_x + (i as f64) * per_nuc_width;
//        let bg = self.draw_rect(document, x, pos_y, per_nuc_width, height);
//        bg.set_attribute(AttributeId::Fill, cscale.scale(nuc).to_hex());
//
//        let txt = self.draw_text(document,  &nuc.to_string(),
//                    x + per_nuc_width/2f64, pos_y + height/2f64, (height - 4f64) as usize,  
//                    true, true);
//
//        seq_node.append(&bg);
//        seq_node.append(&txt);
//    }
//
//    seq_node
//}

