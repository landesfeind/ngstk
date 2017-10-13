use std::marker::PhantomData;

use sequence::*;
use sketch::Canvas;
use sketch::Style;
use sketch::decorator::Decorator;

pub struct SequenceDecorator<E: SequenceElement, S: Sequence<E>> {
    style: Option<Style>,
    sequence: S,
    _phantom_e: PhantomData<E>
}

impl<E: SequenceElement, S: Sequence<E>> SequenceDecorator<E, S>{
    pub fn new(sequence: S) -> Self {
        SequenceDecorator {
            style: None, sequence: sequence, _phantom_e: PhantomData
        }
    }

}

impl<E: SequenceElement, S: Sequence<E>> Decorator for SequenceDecorator<E,S> {

    fn with_style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    fn style(&self) -> Style {
        match self.style.clone() {
            Some(s) => s,
            None => Style::default()
        }
    }

    fn draw<C: Canvas>(&self, canvas: &mut C, offset_y: u64) -> u64 {
        let box_width = self.style().image_width() as f64 / self.sequence.length() as f64;
        let box_height = (self.style().font_size() + 2*self.style().font_padding()) as f64;


        for (i,e) in self.sequence.iterator().enumerate() {
            canvas.draw_rect(
                    box_width * i as f64, // x
                    offset_y as f64, //y
                    box_width,
                    box_height,
                    None
                );
            canvas.draw_text(
                    e,
                    box_width * i as f64 * box_width/2f64,
                    (offset_y + self.style().font_size() * self.style().font_padding()) as f64,
                    self.style().font_size(),
                    true,
                    true,
                    Some(self.style().font_color())
                )

        }


        0u64
    }
}