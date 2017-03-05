extern crate svgdom;
pub use self::svgdom::*;

mod internal;

use genomicregion::GenomicRegion;
use genomicrange::GenomicRange;
use sketch::scale::numerical::NumericalScale;
use sketch::scale::genomic::GenomicScale;
use sketch::scale::genomic::NucleotideColorScale;

pub use sketch::GraphicsOutput;

pub struct SVG {
    reference: GenomicRegion,
    document: Document,
    node_svg: Node,
    xscale: GenomicScale
}

impl GraphicsOutput for SVG {

    fn new(reference: GenomicRegion) -> Self {
        let doc = Document::new();
        let svg = doc.create_element(ElementId::Svg);
        doc.append(&svg);

        let gr = GenomicRange::from(&reference);

        SVG {
            reference: reference,
            document: doc,
            node_svg: svg,
            xscale: GenomicScale::new(gr)
        }
    }
}

impl SVG {
    pub fn to_string(&self) -> String {
       self.document.to_string_with_opt(&WriteOptions::default())
    }

}
