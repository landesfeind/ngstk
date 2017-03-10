extern crate svgdom;
pub use self::svgdom::*;

mod internal;
mod genomicregion;

use region::*;
use template::*;
use genomicregion::GenomicRegion;
use sketch::scale::Scale;
use sketch::scale::numerical::NumericalScale;
use sketch::scale::genomic::SequenceScale;
use sketch::scale::genomic::NucleotideColorScale;

pub use sketch::GraphicsOutput;
use sketch::svg::internal::SvgDecorator;

pub struct SVG<I : RegionIdentifier, E : SequenceElement, R : Region<I, E>> {
    region: GenomicRegion<I>,
    document: Document,
    node_svg: Node,
    xscale: SequenceScale<I, E, R>,
    yoffset: f64
}

impl<I : RegionIdentifier, E : SequenceElement, S : Sequence<E>, T : Template<I, E, S>> GraphicsOutput<I, E, S, T> for SVG<I, E, T> {

    fn new(gr: T) -> Self {
        let mut doc = Document::new();
        let mut svg = doc.create_element(ElementId::Svg);
        doc.append(&svg);

        let s = SequenceScale::new(gr.clone());

        let yaxis = NumericalScale::new(vec![0f64,1f64], vec![0f64,20f64], 1f64);
        let gr_node = gr.to_node(&mut doc, &s, &yaxis);
        svg.append(&gr_node);
        
        SVG {
            region: gr,
            document: doc,
            node_svg: svg,
            xscale: s,
            yoffset: 20f64
        }
    }

}

impl<I : RegionIdentifier, E : SequenceElement, R : Region<I, E>> SVG<I,E,R> {
    pub fn to_string(&mut self) -> String {
        self.node_svg.set_attribute(AttributeId::Width, self.xscale.scale(self.region.length()));
        self.node_svg.set_attribute(AttributeId::Height, self.yoffset);
        self.document.to_string_with_opt(&WriteOptions::default())
    }
}
