
use io::bed::BedRecord;
use region::Region;
use sketch::Canvas;
use sketch::Color;
use sketch::Decorator;

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

    pub fn add_record(&mut self, record: BedRecord) {
        self.records.push(record)
    }

    pub fn region(&self) -> Region {
        self.region.clone()
    }


    fn default_block_color(&self) -> Color {
        Color::blue().lighten_by(50u8)
    }
}


impl Decorator for BedRecordDecorator {
    fn draw<C: Canvas>(&self, canvas: &mut C, offset_y: f64) -> f64 {
        let font_size = self.font_size();
        let bg_height = 2.0 * self.font_padding() + font_size;
        let element_width = self.element_width(canvas, &self.region);

        let mut offset_y_here = 0.0;

        for record in &self.records {
            debug!("Appending BedRecord: {:?}", record);

            let start = element_width *
                (record.chrom_start() - self.region.offset().unwrap()) as f64;
            let end = element_width * (record.chrom_end() - self.region.offset().unwrap()) as f64;

            let block_background = match record.item_rgb() {
                Some(c) => c,
                None => self.default_block_color(),
            };

            canvas.draw_rect(
                start,
                offset_y + offset_y_here,
                end - start,
                bg_height,
                Some(block_background),
            );

            if record.has_strand() {
                let num_arrows = record.length() / 3;
                for 0 .. num_arrows {
                    
                }
            }

            if record.has_name() {
                let text_x = start + self.font_padding() as f64;
                let text_y = (offset_y + offset_y_here + self.font_padding() + font_size) as f64;
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

            // FIXME:
            // - thick 
            // - blocks

            offset_y_here += bg_height;
        }

        offset_y_here
    }
}
