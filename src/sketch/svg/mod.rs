extern crate svgdom;
pub use self::svgdom::*;
use std::marker::PhantomData;
use std::fmt;

use sequence::*;
use alignment::*;
use sketch::scale::Scale;
use sketch::color;
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
        let h = (FONT_SIZE + (2*PADDING)) as f64;
        let w = self.xscale.scale(1) - self.xscale.scale(0);

        let segments = alignment.segments();

        // Deletions go in the background
        for seg in segments.iter().filter(|s| s.is_deletion()) {
            let seg_x = self.xscale.scale(seg.template_offset().expect("Not aligned"));
            let seg_w = w * ( seg.template_length().expect("Not aligned") as f64);
            
            let bg   = internal::draw_rect(&mut self.document, seg_x, (self.image_height + PADDING) as f64, seg_w, h - (2f64*PADDING as f64), Some(color::deletion()));
            self.node_svg.append(&bg);
        }
        
        for seg in segments.iter().filter(|s| s.is_match() || s.is_mismatch()) {
            let seg_x = self.xscale.scale(seg.template_offset().expect("Not aligned"));
            let seg_w = self.xscale.scale(seg.template_offset().expect("Not aligned") + seg.template_length().expect("Not aligned")) - seg_x;

            let path = if seg.is_reverse() {
                svgdom::types::path::Builder::new()
                    .move_to(seg_x                          , self.image_height as f64)
                    .line_to(seg_x + seg_w                  , self.image_height as f64)
                    .line_to(seg_x + seg_w                  , self.image_height as f64 + h)
                    .line_to(seg_x                          , self.image_height as f64 + h)
                    .line_to(seg_x         - (w as f64)/4f64, self.image_height as f64 + h/2f64)
                    .close_path()
                    .finalize()
            } else {
                svgdom::types::path::Builder::new()
                    .move_to(seg_x                          , self.image_height as f64)
                    .line_to(seg_x + seg_w                  , self.image_height as f64)
                    .line_to(seg_x + seg_w + (w as f64)/4f64, self.image_height as f64 + h/2f64)
                    .line_to(seg_x + seg_w                  , self.image_height as f64 + h)
                    .line_to(seg_x                          , self.image_height as f64 + h)
                    .close_path()
                    .finalize()
            };

            let bg   = internal::draw_path(&mut self.document, path, Some(Color::black()), Some(Color::light_blue()));
            self.node_svg.append(&bg);

            let t_seq = seg.template_slice().expect("Can not unwrap template slice").as_vec();
            let s_seq = seg.sequence_slice().as_vec();
            for i in 0 .. t_seq.len() {
                if s_seq[i] != t_seq[i] {
                    let n = format!("{}", s_seq[i]);
                    let color = self.element_to_color(&s_seq[i]);
                    let x = self.xscale.scale(seg.template_offset().expect("Not aligned") + i);

                    let bg   = internal::draw_rect(&mut self.document, x, self.image_height as f64, w, h, color);
                    let text = internal::draw_text(&mut self.document, n.as_ref(), 
                                                   x + (w/2f64), (self.image_height + PADDING + FONT_SIZE) as f64,
                                                   FONT_SIZE, 
                                                   true, true, None);
                    self.node_svg.append(&bg);
                    self.node_svg.append(&text);
                }
            }
        }
        
        // Deletions go in the background
        for seg in segments.iter().filter(|s| s.is_insertion()) {
            let seg_x = self.xscale.scale(seg.template_offset().expect("Not aligned"));
            let seg_w = (w as f64) / 4f64;

            let path = svgdom::types::path::Builder::new()
                .move_to(seg_x - seg_w, self.image_height as f64)
                .line_to(seg_x + seg_w, self.image_height as f64)
                .move_to(seg_x - seg_w, self.image_height as f64 + h)
                .line_to(seg_x + seg_w, self.image_height as f64 + h)
                .move_to(seg_x        , self.image_height as f64)
                .line_to(seg_x        , self.image_height as f64 + h)
                .finalize();
            let path   = internal::draw_path(&mut self.document, path, Some(color::insertion()), None);
            path.set_attribute(AttributeId::StrokeWidth, 2);
            self.node_svg.append(&path);
        }


        self.image_height += FONT_SIZE + (2*PADDING);
    }

}

impl<E:SequenceElement, CS: Scale<E, Color>> fmt::Display for SvgOutput<E, CS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.node_svg.set_attribute(AttributeId::Width, self.xscale.scale(self.template_offset + self.template_length));
        self.node_svg.set_attribute(AttributeId::Height, self.image_height as f64);
        write!(f, "{}", self.document.to_string_with_opt(&WriteOptions::default()))
    }
}

