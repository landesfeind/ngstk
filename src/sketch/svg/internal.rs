extern crate svgdom;
pub use self::svgdom::*;

pub use sketch::scale::Scale;
pub use sketch::scale::genomic::SequenceScale;
pub use sketch::scale::genomic::NucleotideColorScale;
pub use sketch::scale::numerical::NumericalScale;

use sequence::*;
use dna::*;
use region::*;

pub trait SvgDecorator {

    fn x_from<I : RegionIdentifier, E : SequenceElement, T : Region<I, E>>(&self, xscale: &SequenceScale<I,E,T>) -> f64;
    fn x_to<I : RegionIdentifier, E : SequenceElement, T : Region<I, E>>(&self, xscale: &SequenceScale<I,E,T>) -> f64;

    fn y_from(&self, yscale: &NumericalScale) -> f64;
    fn y_to(&self  , yscale: &NumericalScale) -> f64;

    fn to_node<I : RegionIdentifier, E : SequenceElement,T : Region<I, E>>(&self, doc: &mut Document, xscale: &SequenceScale<I,E,T>, yscale: &NumericalScale) -> Node;

    fn draw_text(&self, document: &mut Document, text: &str, pos_x: f64, pos_y: f64, font_size: usize, align_center: bool, valign_center: bool) -> Node {
        let text_node = document.create_element(ElementId::Text);
        let data_node = document.create_node(NodeType::Text, text);
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
        if valign_center {
            text_node.set_attribute(AttributeId::DominantBaseline, "middle");
        }
        else {
            text_node.set_attribute(AttributeId::DominantBaseline, "bottom");
        }
        text_node
    }
    
    fn draw_rect(&self, document: &mut Document, pos_x: f64, pos_y: f64, width: f64, height: f64) -> Node {
        let rect = document.create_element(ElementId::Rect);
        rect.set_attribute(AttributeId::Fill, "none");
        rect.set_attribute(AttributeId::Stroke, "black");
        rect.set_attribute(AttributeId::StrokeWidth, "1");
        rect.set_attribute(AttributeId::X, pos_x);
        rect.set_attribute(AttributeId::Y, pos_y);
        rect.set_attribute(AttributeId::Width, width);
        rect.set_attribute(AttributeId::Height, height);
        rect
    }
    
    fn draw_circ(&self, document: &mut Document, pos_x: f64, pos_y: f64, radius: f64) -> Node {
        let circ = document.create_element(ElementId::Circle);
        circ.set_attribute(AttributeId::Fill, "none");
        circ.set_attribute(AttributeId::Stroke, "black");
        circ.set_attribute(AttributeId::StrokeWidth, "1");
        circ.set_attribute(AttributeId::X, pos_x);
        circ.set_attribute(AttributeId::Y, pos_y);
        circ.set_attribute(AttributeId::R, radius);
        circ
    }

    fn draw_dnasequence(&self, document: &mut Document, seq: &DnaSequence, pos_x: f64, pos_y: f64, width: f64, height: f64) -> Node {
        let per_nuc_width = width / ((seq.length() + 1) as f64);
        let seq_node = document.create_element(ElementId::G);

        let cscale = NucleotideColorScale::new();

        for (i, nuc) in seq.iter().enumerate() {
            let x = pos_x + (i as f64) * per_nuc_width;
            let bg = self.draw_rect(document, x, pos_y, per_nuc_width, height);
            bg.set_attribute(AttributeId::Fill, cscale.scale(nuc).to_hex());

            let txt = self.draw_text(document,  &nuc.to_string(),
                        x + per_nuc_width/2f64, pos_y + height/2f64, (height - 4f64) as usize,  
                        true, true);

            seq_node.append(&bg);
            seq_node.append(&txt);
        }

        seq_node
    }
}
