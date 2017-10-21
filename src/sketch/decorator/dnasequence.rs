use sequence::*;
use sketch::Canvas;
use sketch::Color;
use sketch::decorator::Decorator;
use sketch::scale::Scale;
use sketch::scale::sequences::DnaNucleotideColorScale;

pub struct DnaSequenceDecorator {
    sequence: DnaSequence,
}

impl DnaSequenceDecorator {
    pub fn new(sequence: DnaSequence) -> Self {
        DnaSequenceDecorator { sequence: sequence }
    }

    fn element_to_color(n: &DnaNucleotide) -> Color {
        DnaNucleotideColorScale::default().scale(n.clone())
    }
}

impl Decorator for DnaSequenceDecorator {
    fn draw<C: Canvas>(&self, canvas: &mut C, offset_y: f64) -> f64 {
        let box_width = canvas.image_width() as f64 / self.sequence.length() as f64;
        let box_height = (self.font_size() + 2.0 * self.font_padding()) as f64;


        for (i, e) in self.sequence.iterator().enumerate() {
            canvas.draw_rect(
                box_width * i as f64, // x
                offset_y as f64, //y
                box_width,
                box_height,
                Some(Self::element_to_color(e)),
            );
            canvas.draw_text(
                e,
                (box_width * i as f64) + (box_width / 2f64),
                (offset_y + self.font_size() + self.font_padding()) as f64,
                self.font_size(),
                "normal",
                true,
                true,
                Some(self.font_color()),
            );
        }

        box_height
    }
}
