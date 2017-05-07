extern crate svgdom;
pub use self::svgdom::*;
use std::marker::PhantomData;
use std::fmt;
use std::io::Write;

use sequence::*;
use alignment::*;

pub mod color;
pub mod scale;
mod shapes;
use sketch::scale::Scale;
use sketch::scale::genomic::SequenceScale;
use sketch::shapes::*;

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

    fn template_offset(&self) -> usize {
        self.template_offset
    }

    fn template_length(&self) -> Option<usize> {
        Some(self.template_length)
    }

    pub fn append_section(&mut self, title: &str){
        let bg   = shapes::draw_rect(&mut self.document, 
                                       0f64, self.image_height as f64, 
                                       self.image_width as f64, (FONT_SIZE + (2*PADDING)) as f64, 
                                       Some(Color::black()));
        let text = shapes::draw_text(&mut self.document, title,
                                       PADDING as f64, 
                                       (self.image_height + PADDING + FONT_SIZE) as f64,
                                       FONT_SIZE, 
                                       false, true, Some(Color::white()));
        self.node_svg.append(&bg);
        self.node_svg.append(&text);

        self.image_height += FONT_SIZE + (2*PADDING);
    }

    pub fn append_sequence<S: Sequence<E>>(&mut self, sequence: &S){
        let to = self.template_offset();
        self.append_sequence_with_offset(sequence, to);
    }
    pub fn append_sequence_with_offset<S: Sequence<E>>(&mut self, sequence: &S, offset: usize){
        let h = (FONT_SIZE + (2*PADDING)) as f64;
        let w = self.xscale.scale(1) - self.xscale.scale(0);

        let vec = sequence.as_vec();

        let g = shapes::group(&mut self.document);
        self.node_svg.append(&g);

        for i in 0 .. sequence.length() {
            let n = format!("{}", vec[i]);
            let x = self.xscale.scale(offset + i);

            let color = self.element_to_color(&vec[i]);
            let bg   = shapes::draw_rect(&mut self.document, x, self.image_height as f64, w, h, color);
            let text = shapes::draw_text(&mut self.document, n.as_ref(), 
                                           x + (w/2f64), (self.image_height + PADDING + FONT_SIZE) as f64,
                                           FONT_SIZE, 
                                           true, true, None);
            g.append(&bg);
            g.append(&text);
        }
        self.image_height += FONT_SIZE + (2*PADDING);
    }

    pub fn append_alignment<S: Sequence<E>>(&mut self, alignment: &Alignment<E,S>){
        self.append_alignments( &vec![ alignment.clone() ]  );
    }

    pub fn append_alignments<S: Sequence<E>>(&mut self, alignments: &Vec<Alignment<E,S>>){
        let h = (FONT_SIZE + (2*PADDING)) as f64;
        let w = self.xscale.scale(1) - self.xscale.scale(0);

        let g_alignments  = shapes::group(&mut self.document);
        self.node_svg.append(&g_alignments);

        let g_alignment_paths = shapes::group(&mut self.document);
        g_alignments.append(&g_alignment_paths);
        let g_deletions  = shapes::group(&mut self.document);
        g_alignments.append(&g_deletions);
        let g_matches    = shapes::group(&mut self.document);
        g_alignments.append(&g_matches);
        let g_mismatches = shapes::group(&mut self.document);
        g_alignments.append(&g_mismatches);
        let g_insertions = shapes::group(&mut self.document);
        g_alignments.append(&g_insertions);

        for alignment in alignments {
            println!("ALIGNMENT: {:?}", alignment);

            let segments = alignment.segments();

            let alignment_start = segments.iter()
                .filter(|s| s.is_aligned())
                .map(|s| s.template_offset().unwrap())
                .min();
            let alignment_end = segments.iter()
                .filter(|s| s.is_aligned())
                .map(|s| s.template_offset().unwrap() + s.template_length().unwrap())
                .max();
            if alignment_start.is_some() && alignment_end.is_some() {
                let mut alignment_path = shapes::draw_line(
                        &mut self.document,
                        self.xscale.scale(alignment_start.unwrap()),
                        (self.image_height as f64) + (h/2f64),
                        self.xscale.scale(alignment_end.unwrap()),
                        (self.image_height as f64) + (h/2f64),
                        None
                    );
                shapes::set_stroke(&mut alignment_path, Some(Color::new(200, 200, 200)), Some(3usize));
                g_alignment_paths.append(&alignment_path);
            }


            // Draw the alignment path
            for idx in 0 .. segments.len() {
                // Deletions go in the background
                let segment = segments[idx].clone();
                println!("      SEG: {:?}", segment);


                if segment.is_match() || segment.is_mismatch() {
                    println!("ALIGNING: {} {} '{}' '{}'", segment.is_match(), segment.is_mismatch(), segment.template_slice().unwrap(), segment.sequence_slice());
                    let seg_x = self.xscale.scale(segment.template_offset().expect("Not aligned"));
                    let seg_w = self.xscale.scale(   segment.template_offset().expect("Not aligned") 
                                                   + segment.template_length().expect("Not aligned") ) - seg_x;

                    let path = if segment.is_reverse() {
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

                    let bg   = shapes::draw_path(&mut self.document, path, Some(Color::black()), Some(Color::light_blue()));
                    g_matches.append(&bg);

                    let t_seq = segment.template_slice().expect("Can not unwrap template slice").as_vec();
                    let s_seq = segment.sequence_slice().as_vec();
                    for i in 0 .. t_seq.len() {
                        if s_seq[i] != t_seq[i] {
                            let n = format!("{}", s_seq[i]);
                            let color = self.element_to_color(&s_seq[i]);
                            let x = self.xscale.scale( segment.template_offset().expect("Not aligned") + i);

                            let bg   = shapes::draw_rect(&mut self.document, x, self.image_height as f64, w, h, color);
                            let text = shapes::draw_text(&mut self.document, n.as_ref(), 
                                                           x + (w/2f64), (self.image_height + PADDING + FONT_SIZE) as f64,
                                                           FONT_SIZE, 
                                                           true, true, None);
                            g_mismatches.append(&bg);
                            g_mismatches.append(&text);
                        }
                    }
                }
                else if segment.is_deletion() {
                    println!("DELETION");
                    let seg_x = self.xscale.scale(segment.template_offset().expect("Not aligned"));
                    let seg_w = w * ( segment.template_length().expect("Not aligned") as f64);
                    
                    let bg   = shapes::draw_rect(&mut self.document, 
                                                 seg_x, (self.image_height + PADDING) as f64, 
                                                 seg_w, h - (2f64*PADDING as f64), 
                                                 Some(color::deletion())
                                                );
                    g_deletions.append(&bg);
                }
                else if segment.is_insertion() {
                    println!("INSERTION");
                    let seg_x = self.xscale.scale(segment.template_offset().expect("Not aligned"));
                    let seg_w = (w as f64) / 4f64;

                    let path = svgdom::types::path::Builder::new()
                        .move_to(seg_x - seg_w, self.image_height as f64)
                        .line_to(seg_x + seg_w, self.image_height as f64)
                        .move_to(seg_x - seg_w, self.image_height as f64 + h)
                        .line_to(seg_x + seg_w, self.image_height as f64 + h)
                        .move_to(seg_x        , self.image_height as f64)
                        .line_to(seg_x        , self.image_height as f64 + h)
                        .finalize();
                    let path   = shapes::draw_path(&mut self.document, path, Some(color::insertion()), None);
                    path.set_attribute(AttributeId::StrokeWidth, 2);
                    g_insertions.append(&path);
                }
                else if ! segment.is_aligned() {
                    let seg_x = match idx == 0 {
                            true => self.xscale.scale(segments[idx + 1].template_offset().expect("Not aligned")),
                            false => self.xscale.scale(segments[idx - 1].template_offset().expect("Not aligned"))
                    };
                    let seg_w = (w as f64) / 4f64;

                    let path = svgdom::types::path::Builder::new()
                        .move_to(seg_x - seg_w, self.image_height as f64)
                        .line_to(seg_x + seg_w, self.image_height as f64)
                        .move_to(seg_x - seg_w, self.image_height as f64 + h)
                        .line_to(seg_x + seg_w, self.image_height as f64 + h)
                        .move_to(seg_x        , self.image_height as f64)
                        .line_to(seg_x        , self.image_height as f64 + h)
                        .finalize();
                    let node = match segment.sequence_length() > 0 {
                        true  => shapes::draw_path(&mut self.document, path, Some(Color::orange()), None), // soft clip
                        false => shapes::draw_path(&mut self.document, path, Some(Color::red()   ), None)  // hard clip
                    };
                    node.set_attribute(AttributeId::StrokeWidth, 2);
                    g_insertions.append(&node);
                }
                else {
                    panic!("Do not know how to sketch: {:?}", segment);
                }
            }
            self.image_height += FONT_SIZE + (2*PADDING);
        }
    }




    /// Write the generated SVG to the given destination
    pub fn write<W : Write>(&self, dst: &mut W){
        write!(dst, "{}", self);
    }
}

impl<E:SequenceElement, CS: Scale<E, Color>> fmt::Display for SvgOutput<E, CS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.node_svg.set_attribute(AttributeId::Width, self.xscale.scale(self.template_offset + self.template_length));
        self.node_svg.set_attribute(AttributeId::Height, self.image_height as f64);
        write!(f, "{}", self.document.to_string_with_opt(&WriteOptions::default()))
    }
}

