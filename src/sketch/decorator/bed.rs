

use io::bed::BedRecord;
use region::Region;
use sketch::Canvas;
use sketch::Color;
use sketch::Decorator;
use sketch::canvas::DrawOperation;
use std::collections::BTreeMap;

pub struct BedRecordDecorator {
    records: Vec<BedRecord>,
    region: Region,
}

impl BedRecordDecorator {
    pub fn new(region: Region) -> Self {
        BedRecordDecorator {
            records: Vec::new(),
            region: region,
        }
    }

    pub fn with_region(mut self, region: Region) -> Self {
        self.region = region;
        self
    }

    pub fn with_records(mut self, records: Vec<BedRecord>) -> Self {
        self.records = records;
        self
    }

    pub fn region(&self) -> Region {
        self.region.clone()
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
        let mut offsets: BTreeMap<usize, Region> = BTreeMap::new();

        let font_size = self.font_size();
        let bg_height = 2.0 * self.font_padding() + font_size;
        let element_width = self.element_width(canvas, &self.region);

        for record in &self.records {
            debug!("Appending BedRecord: {:?}", record);
            let row = self.find_offset_row(&mut offsets, Region::from(record)) as f64;
            debug!(" -> will be placed in row {}", row);

            let start = element_width *
                (record.chrom_start() - self.region.offset().unwrap()) as f64;
            let end = element_width * (record.chrom_end() - self.region.offset().unwrap()) as f64;

            let offset_y_here = offset_y + row * bg_height;

            let block_background = match record.item_rgb() {
                Some(c) => c,
                None => self.default_bg_color(),
            };

            // Draw the full block
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
