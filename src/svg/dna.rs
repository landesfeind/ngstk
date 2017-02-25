use svg::internal::*;
use sequence::*;
use dna::*;

impl SvgDecorator for DnaNucleotide {
    
    fn width(&self) -> f64 {
        15f64
    }

    fn height(&self) -> f64 {
        self.width()
    }

    fn to_svg(&self, doc: &Document, xscale: &Scale, yscale: &Scale) -> Node {
        let background = doc.create_element(ElementId::Rect);
        background.set_attribute(AttributeId::Fill, "none");
        background.set_attribute(AttributeId::Stroke, "black");
        background.set_attribute(AttributeId::StrokeWidth, "0.5");
        background.set_attribute(AttributeId::X, xscale.to(0.5f64));
        background.set_attribute(AttributeId::Y, yscale.to(0.5f64));
        background.set_attribute(AttributeId::Width,  self.width() );
        background.set_attribute(AttributeId::Height, self.height());

        let node_symbol = self.to_string().to_svg(doc, xscale, yscale);
        node_symbol.set_attribute(AttributeId::X, xscale.to(self.width() / 2.0));

        let self_node = doc.create_element(ElementId::G);
        self_node.append(&background);
        self_node.append(&node_symbol);
        self_node
    }
}


impl SvgDecorator for DnaSequence {

    /// Returns the width of an element `t`.
    fn width(&self) -> f64 {
         DnaNucleotide::from('N').width() * (self.length() as f64)
    }

    /// Returns the height for an element `t`.
    fn height(&self) -> f64 {
        DnaNucleotide::from('N').height() as f64
    }

    fn to_svg(&self, doc: &Document, xscale: &Scale, yscale: &Scale) -> Node {
        let self_node = doc.create_element(ElementId::G);
        
        let n_width = DnaNucleotide::from('N').width();
        let mut offset = 0f64;
        for n in self.iter() {
            let nnode = n.to_svg(doc, &xscale.rescale(offset, 1f64), yscale);
            self_node.append(&nnode);
            offset += n_width;
        }

        self_node
    }
}


