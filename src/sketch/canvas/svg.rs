extern crate svgdom;
use self::svgdom::*;
use sketch::Color;
use sketch::canvas::*;
use std::io::Write;

pub struct Svg {
    image_width: f64,
    image_height: f64,
    document: Document,
    node_svg: Node,
}

impl Svg {
    fn set_color(n: &mut Node, co: Option<Color>) {
        match co {
            Some(c) => {
                n.set_attribute(AttributeId::Color, c.to_hex());
                n.set_attribute(AttributeId::Opacity, c.opacity());
            }
            None => {}
        }
    }

    fn set_fill_color(n: &mut Node, co: Option<Color>) {
        match co {
            Some(c) => {
                debug!("Setting fill color to: {}", c.to_hex());
                n.set_attribute(AttributeId::Fill, c.to_hex());
                n.set_attribute(AttributeId::FillOpacity, c.opacity());
            }
            None => {}
        }
    }


    fn set_stroke(n: &mut Node, co: Option<Color>, wo: Option<usize>) {
        match co {
            Some(c) => {
                n.set_attribute(AttributeId::Stroke, c.to_hex());
                n.set_attribute(AttributeId::StrokeOpacity, c.opacity());
            }
            None => {}
        }
        match wo {
            Some(w) => n.set_attribute(AttributeId::StrokeWidth, format!("{}", w)),
            None => {}
        }
    }
}


impl Default for Svg {
    fn default() -> Self {
        let doc = Document::new();
        let svg = doc.create_element(ElementId::Svg);
        doc.append(&svg);

        Svg {
            image_width: 720f64,
            image_height: 1024f64,
            document: doc,
            node_svg: svg,
        }
    }
}



impl Canvas for Svg {
    fn write<W: Write>(&self, mut out: W) {
        self.node_svg.set_attribute(
            AttributeId::Height,
            format!("{}", self.image_height),
        );
        self.node_svg.set_attribute(
            AttributeId::Width,
            format!("{}", self.image_width),
        );
        write!(out, "{}", self.document.to_string());
    }

    fn with_image_width(mut self, new_width: f64) -> Self {
        self.image_width = new_width;
        self
    }

    fn image_width(&self) -> f64 {
        self.image_width
    }

    fn with_image_height(mut self, new_height: f64) -> Self {
        self.image_height = new_height;
        self
    }

    fn image_height(&self) -> f64 {
        self.image_height
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
    ) {
        let mut text_node = self.document.create_element(ElementId::Text);
        let data_node = self.document.create_node(
            NodeType::Text,
            text.to_string().as_ref(),
        );
        text_node.append(&data_node);

        debug!(
            "Adding text '{}' at position ({}, {})",
            text.to_string(),
            pos_x,
            pos_y
        );
        text_node.set_attribute(AttributeId::X, pos_x);
        text_node.set_attribute(AttributeId::Y, pos_y);

        text_node.set_attribute(AttributeId::FontSize, format!("{}px", font_size));
        text_node.set_attribute(AttributeId::FontWeight, font_weight);
        if align_center {
            text_node.set_attribute(AttributeId::TextAnchor, "middle");
        } else {
            text_node.set_attribute(AttributeId::TextAnchor, "left");
        }

        text_node.set_attribute(AttributeId::DominantBaseline, "bottom");
        if valign_center {
            text_node.set_attribute(AttributeId::Y, pos_y - (font_size as f64 / 2f64));
            text_node.set_attribute(AttributeId::Dy, font_size as f64 / 3f64);
        }

        self.node_svg.append(&text_node);

        Self::set_fill_color(&mut text_node, color);
    }

    fn draw_rect(
        &mut self,
        pos_x: f64,
        pos_y: f64,
        width: f64,
        height: f64,
        fill_color: Option<Color>,
    ) {
        let mut rect = self.document.create_element(ElementId::Rect);
        rect.set_attribute(AttributeId::Fill, "none");
        rect.set_attribute(AttributeId::Stroke, "black");
        rect.set_attribute(AttributeId::StrokeWidth, "1");
        rect.set_attribute(AttributeId::X, pos_x);
        rect.set_attribute(AttributeId::Y, pos_y);
        rect.set_attribute(AttributeId::Width, width);
        rect.set_attribute(AttributeId::Height, height);
        Self::set_fill_color(&mut rect, fill_color);

        self.node_svg.append(&rect);
    }

    fn draw_circ(&mut self, pos_cx: f64, pos_cy: f64, radius: f64, fill_color: Option<Color>) {
        let mut circ = self.document.create_element(ElementId::Rect);
        circ.set_attribute(AttributeId::Fill, "none");
        circ.set_attribute(AttributeId::Stroke, "black");
        circ.set_attribute(AttributeId::StrokeWidth, "1");
        circ.set_attribute(AttributeId::Cx, pos_cx);
        circ.set_attribute(AttributeId::Cy, pos_cy);
        circ.set_attribute(AttributeId::Radius, radius);
        Self::set_fill_color(&mut circ, fill_color);

        self.node_svg.append(&circ);
    }


    fn draw_line(
        &mut self,
        pos_x1: f64,
        pos_y1: f64,
        pos_x2: f64,
        pos_y2: f64,
        color: Option<Color>,
    ) {
        let mut rect = self.document.create_element(ElementId::Line);
        rect.set_attribute(AttributeId::X1, pos_x1);
        rect.set_attribute(AttributeId::Y1, pos_y1);
        rect.set_attribute(AttributeId::X2, pos_x2);
        rect.set_attribute(AttributeId::Y2, pos_y2);
        Self::set_stroke(&mut rect, color, Some(1));
        self.node_svg.append(&rect);
    }

    fn draw_path(&mut self, path: Vec<DrawOperation>, stroke_color: Option<Color>, fill_color: Option<Color>) {
        let mut svgpath = svgdom::types::path::Builder::new();
        for op in path {
            match op {
                DrawOperation::MoveTo(x,y) => svgpath = svgpath.move_to(x,y),
                DrawOperation::LineTo(x,y) => svgpath = svgpath.line_to(x,y)
            }
        }
        let mut pn = self.document.create_element(ElementId::Path);
        pn.set_attribute(AttributeId::D, svgpath.finalize());
        Self::set_stroke(&mut pn, stroke_color, Some(1));
        Self::set_fill_color(&mut pn, fill_color);
        self.node_svg.append(&pn);
    }
}
