extern crate svgdom;
pub use self::svgdom::*;
use std::marker::PhantomData;
use std::fmt;

use sequence::*;
use alignment::*;
use sketch::scale::Scale;
use sketch::scale::genomic::SequenceScale;
mod internal;
use sketch::svg::internal::*;

pub use sketch::GraphicsOutput;

const FONT_SIZE : usize = 12;
const PADDING : usize = 2;

pub struct SvgOutput<E : SequenceElement, CS: Scale<E,Color>> {
    template_offset: usize,
    template_length: usize,
    image_width: usize,
    image_height: usize,
    document: Document,
    node_svg: Node,
    xscale: SequenceScale,
    color_profile: CS,
    _marker: PhantomData<E>
}

impl<E : SequenceElement, CS: Scale<E,Color>> SvgOutput<E, CS> {
    /// Create a new output using the given offset and the given length. 
    /// Both parameters are given in number of `SequenceElement`s.
    pub fn new(template_offset: usize, template_length: usize, image_width: usize, color_profile: CS) -> Self {
        let mut doc = Document::new();
        let mut svg = doc.create_element(ElementId::Svg);
        doc.append(&svg);

        let s = SequenceScale::new_with_max_width(template_offset, template_length, image_width);

        SvgOutput {
            template_offset: template_offset,
            template_length: template_length,
            image_width: image_width,
            image_height: 0usize,
            document: doc,
            node_svg: svg,
            xscale: s,
            color_profile: color_profile,
            _marker: PhantomData
        }
    }

    fn element_to_color(&self, e: &E) -> Option<Color> {
        Some(self.color_profile.scale(e.clone()))
    }
}

impl<E : SequenceElement, CS: Scale<E,Color>> GraphicsOutput<E> for SvgOutput<E, CS> {

    fn template_offset(&self) -> usize {
        self.template_offset
    }

    fn template_length(&self) -> Option<usize> {
        Some(self.template_length)
    }

    fn append_section(&mut self, title: &str){
        let bg   = internal::draw_rect(&mut self.document, 
                                       0f64, self.image_height as f64, 
                                       self.image_width as f64, (FONT_SIZE + (2*PADDING)) as f64, 
                                       Some(Color::black()));
        let text = internal::draw_text(&mut self.document, title,
                                       PADDING as f64, 
                                       (self.image_height + PADDING + FONT_SIZE) as f64,
                                       FONT_SIZE, 
                                       false, true, Some(Color::white()));
        self.node_svg.append(&bg);
        self.node_svg.append(&text);

        self.image_height += FONT_SIZE + (2*PADDING);
    }

    fn append_sequence_with_offset<S: Sequence<E>>(&mut self, sequence: &S, offset: usize){

        let h = (FONT_SIZE + (2*PADDING)) as f64;
        let w = self.xscale.scale(1) - self.xscale.scale(0);

        let vec = sequence.as_vec();

        for i in 0 .. sequence.length() {
            let n = format!("{}", vec[i]);
            let x = self.xscale.scale(offset + i);

            let color = self.element_to_color(&vec[i]);

            let bg   = internal::draw_rect(&mut self.document, x, self.image_height as f64, w, h, color);
            let text = internal::draw_text(&mut self.document, n.as_ref(), 
                                           x + (w/2f64), (self.image_height + PADDING + FONT_SIZE) as f64,
                                           FONT_SIZE, 
                                           true, true, None);
            self.node_svg.append(&bg);
            self.node_svg.append(&text);
        }
        self.image_height += FONT_SIZE + (2*PADDING);
    }

    fn append_alignment<S: Sequence<E>>(&mut self, alignment: &Alignment<E,S>){
        //unimplemented!();
    }

}

impl<E:SequenceElement, CS: Scale<E, Color>> fmt::Display for SvgOutput<E, CS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.node_svg.set_attribute(AttributeId::Width, self.xscale.scale(self.template_offset + self.template_length));
        self.node_svg.set_attribute(AttributeId::Height, self.image_height as f64);
        write!(f, "{}", self.document.to_string_with_opt(&WriteOptions::default()))
    }
}

