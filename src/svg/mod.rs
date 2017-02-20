extern crate svgdom;

use std::fmt;
use std::rc::Rc;
use std::collections::HashMap;

pub use self::svgdom::*;
use data::sequence::*;
use data::dna::*;

struct Scale {
    offset: f64, length: f64
}
impl Scale {
    fn scale(&self, p: f64) -> f64 {
        assert!(p>=0f64);
        assert!(p<=1f64);
        self.offset + (p * self.length)
    }


    fn length(&self) -> f64 {
        self.length
    }

    fn subscale(&self, min: f64, max: f64) -> Scale {
        Scale { offset: self.scale(min), length: self.scale(max) }
    }
}

pub trait ToSvg {
    fn to_svg(&self, doc: &Document, xscale: &Scale, yscale: &Scale) -> Node;
}

impl ToSvg for String {
    fn to_svg(&self, doc: &Document, xscale: &Scale, yscale: &Scale) -> Node {
        let mut self_node = doc.create_element(ElementId::Text);
        let textnode = doc.create_node(NodeType::Text, &self.as_ref());
        self_node.append(&textnode);
        self_node
    }

}

impl ToSvg for DnaNucleotide {
    fn to_svg(&self, doc: &Document, xscale: &Scale, yscale: &Scale) -> Node {
        let background = doc.create_element(ElementId::Rect);
        background.set_attribute(AttributeId::Fill, "none");
        background.set_attribute(AttributeId::Stroke, "black");
        background.set_attribute(AttributeId::StrokeWidth, "0.5");
        background.set_attribute(AttributeId::X, xscale.scale(0f64));
        background.set_attribute(AttributeId::Y, yscale.scale(0f64));
        background.set_attribute(AttributeId::Width, xscale.scale(1f64));
        background.set_attribute(AttributeId::Height, yscale.scale(1f64));

        let node_symbol = self.to_string().to_svg(doc, xscale, yscale);
        node_symbol.set_attribute(AttributeId::TextAnchor, "middle");
        node_symbol.set_attribute(AttributeId::DominantBaseline, "middle");
        node_symbol.set_attribute(AttributeId::X, xscale.scale(0.5));
        node_symbol.set_attribute(AttributeId::Y, yscale.scale(0.5));

        let mut self_node = doc.create_element(ElementId::G);
        self_node.append(&background);
        self_node.append(&node_symbol);
        self_node
    }
}

impl ToSvg for DnaSequence {
    fn to_svg(&self, doc: &Document, xscale: &Scale, yscale: &Scale) -> Node {
        let mut self_node = doc.create_element(ElementId::G);
        
        let step = 1f64 / self.length() as f64;
        let mut offset = 0f64;
        for n in self.iter() {
            let mut nnode = n.to_svg(doc, &xscale.subscale(offset, offset + step), yscale);
            let t : String = format!("translate({}, 0)", offset);
            nnode.set_attribute(AttributeId::Transform, t);
            offset += step;

            self_node.append(&nnode);
        }

        self_node
    }
}



pub fn sketch<T: ToSvg>(content: &T, width: f64, height: f64) -> String{
    let doc = Document::new();
    let svg = doc.create_element(ElementId::Svg);
    svg.set_attribute(AttributeId::Width, width);
    svg.set_attribute(AttributeId::Height, height);
    doc.append(&svg);

    let content_node = content.to_svg(&doc, &Scale { offset: 0f64, length: width}, &Scale { offset:0f64, length: height });
    svg.append(&content_node);
    
    doc.to_string_with_opt(&WriteOptions::default())
}
