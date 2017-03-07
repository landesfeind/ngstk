use region::RegionIdentifier;
use genomicregion::GenomicRegion;
use sketch::svg::internal::*;

impl<I : RegionIdentifier> SvgDecorator for GenomicRegion<I> {

    fn x_from<E,T>(&self, xscale: &SequenceScale<E,T>) -> f64 {
        0f64
    }
    fn x_to<E,T>(&self, xscale: &SequenceScale<E,T>) -> f64 {
        xscale.scale(self.length() + 1usize)
    }

    fn y_from(&self, yscale: &NumericalScale) -> f64 {
        0f64
    }
    fn y_to(&self  , yscale: &NumericalScale) -> f64 {
        yscale.scale(1f64)
    }

    fn to_node<E,T>(&self, doc: &mut Document, xscale: &SequenceScale<E,T>, yscale: &NumericalScale) -> Node {
        self.draw_dnasequence(doc, self.sequence(),
            self.x_from(xscale), 0f64,
            self.x_to(xscale), 20f64)
    }

}
