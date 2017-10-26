use model::Region;
use io::bed::BedRecord;
use sketch::Canvas;
use sketch::Color;
use sketch::Decorator;
use sketch::canvas::DrawOperation;
use std::collections::BTreeMap;

pub struct BedRecordDecorator {
    records: Vec<BedRecord>
}

impl BedRecordDecorator {
    pub fn new() -> Self {
        BedRecordDecorator {
            records: Vec::new()
        }
    }

    pub fn with_records(mut self, records: Vec<BedRecord>) -> Self {
        self.records = records;
        self
    }

    /// Defines the default color per BedRecord
    fn default_bg_color(&self) -> Color {
        Color::blue().lighten_by(50u8)
    }

    /// Defines the default color for BedRecord sub-blocks
    fn default_block_color(&self) -> Color {
        Color::blue().lighten_by(80u8)
    }
}


impl Decorator for BedRecordDecorator {

    fn draw<C: Canvas>(&self, canvas: &mut C, offset_y: f64) -> f64 {
        let mut offsets: BTreeMap<usize, usize> = BTreeMap::new();

        let font_size = self.font_size();
        let bg_height = 2.0 * self.font_padding() + font_size;

        for record in &self.records {
            debug!("Appending BedRecord: {:?}", record);
            let row = self.find_offset_row(&mut offsets, record) as f64;
            
            let postion_opt = canvas.scale_position_x(record);
            if ! postion_opt.is_some() {
                continue
            }
            let (start, end) = postion_opt.unwrap();
            let offset_y_here = offset_y + row * bg_height;

            let block_background = match record.item_rgb() {
                Some(c) => c,
                None => self.default_bg_color(),
            };

            canvas.draw_rect(
                    start,
                    offset_y_here,
                    end - start,
                    bg_height,
                    Some(block_background),
                );            

            // FIXME:
            // - thick
            // - blocks

            let element_width = (end - start) / record.length() as f64;

            // Draw "arrows" showing the direction
            if record.has_strand() {
                let num_arrows = record.length() / 3;
                for i in 0..num_arrows {
                    let arrow_min_x = start + (3 * i + 1) as f64 * element_width +
                        self.font_padding();
                    let arrow_max_x = start + (3 * i + 2) as f64 * element_width -
                        self.font_padding();

                    let mut path = Vec::new();
                    if record.is_forward_strand().unwrap() {
                        path.push(DrawOperation::MoveTo(
                            arrow_min_x,
                            offset_y_here + self.font_padding(),
                        ));
                        path.push(DrawOperation::LineTo(
                            arrow_max_x,
                            offset_y_here + bg_height / 2.0,
                        ));
                        path.push(DrawOperation::LineTo(
                            arrow_min_x,
                            offset_y_here + bg_height - self.font_padding(),
                        ));
                    } else {
                        path.push(DrawOperation::MoveTo(
                            arrow_max_x,
                            offset_y_here + self.font_padding(),
                        ));
                        path.push(DrawOperation::LineTo(
                            arrow_min_x,
                            offset_y_here + bg_height / 2.0,
                        ));
                        path.push(DrawOperation::LineTo(
                            arrow_max_x,
                            offset_y_here + bg_height - self.font_padding(),
                        ));
                    }

                    canvas.draw_path(path, Some(Color::gray()), Some(Color::transparent()));
                }
            }

            // If the record has a name, it is drawn
            if record.has_name() {
                let text_x = start + self.font_padding() as f64;
                let text_y = (offset_y_here + self.font_padding() + font_size) as f64;
                canvas.draw_text(
                    record.name().unwrap(),
                    text_x,
                    text_y,
                    self.font_size(),
                    "normal",
                    false,
                    true,
                    Some(Color::black()),
                );
            }
        }

        match offsets.keys().max() {
            Some(row) => (row + 1) as f64 * bg_height,
            None => 0f64,
        }
    }
}
