extern crate svgdom;
pub use self::svgdom::*;

use sketch::Color;
use sketch::Style;

mod sectionheader;
pub use self::sectionheader::SectionHeaderDecorator;

pub trait Decorator {
    fn with_style(self, style: Style) -> Self;

    fn style(&self) -> Style;

    fn append(&self, document: &mut svgdom::Document, offset_y: u64) -> (svgdom::Node, u64);

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

    fn add_group(document: &mut Document) -> Node {
        document.create_element(ElementId::G)
    }

    fn add_text<S: ToString>(
        document: &mut Document,
        text: S,
        pos_x: f64,
        pos_y: f64,
        font_size: u64,
        align_center: bool,
        valign_center: bool,
        color: Option<Color>,
    ) -> Node {
        let mut text_node = document.create_element(ElementId::Text);
        let data_node = document.create_node(NodeType::Text, text.to_string().as_ref());
        text_node.append(&data_node);

        debug!("Adding text '{}' at position ({}, {})", text.to_string(), pos_x, pos_y);
        text_node.set_attribute(AttributeId::X, pos_x);
        text_node.set_attribute(AttributeId::Y, pos_y);

        text_node.set_attribute(AttributeId::FontSize, format!("{}px", font_size));
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

        Self::set_fill_color(&mut text_node, color);

        text_node
    }




    fn add_rect(
        document: &mut Document,
        pos_x: f64,
        pos_y: f64,
        width: f64,
        height: f64,
        fill_color: Option<Color>,
    ) -> Node {
        let mut rect = document.create_element(ElementId::Rect);
        rect.set_attribute(AttributeId::Fill, "none");
        rect.set_attribute(AttributeId::Stroke, "black");
        rect.set_attribute(AttributeId::StrokeWidth, "1");
        rect.set_attribute(AttributeId::X, pos_x);
        rect.set_attribute(AttributeId::Y, pos_y);
        rect.set_attribute(AttributeId::Width, width);
        rect.set_attribute(AttributeId::Height, height);
        Self::set_fill_color(&mut rect, fill_color);
        rect
    }
}
