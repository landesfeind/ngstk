
use model::Region;

use sketch::Canvas;
use sketch::Color;
use std::collections::BTreeMap;

mod header;
pub use self::header::HeaderDecorator;
mod dnasequence;
pub use self::dnasequence::DnaSequenceDecorator;
mod bed;
pub use self::bed::BedRecordDecorator;


pub trait Decorator {
    fn draw<C: Canvas>(&self, canvas: &mut C, offset_y: f64) -> f64;

    fn font_size(&self) -> f64 {
        12f64
    }

    fn font_padding(&self) -> f64 {
        0.2 * self.font_size()
    }

    fn font_color(&self) -> Color {
        Color::black().lighten_by(20u8)
    }

    fn find_offset_row<R: Region>(&self, offsets: &mut BTreeMap<usize, usize>, reg: &R) -> usize {
        for (key, val) in offsets.clone().iter() {
            debug!(" ==> {}: {:?}", *key, val);
            if *val < reg.offset() {
                offsets.insert(*key, reg.end());
                return *key;
            }
        }
        let new_key = match offsets.keys().max() {
            Some(e) => e + 1usize,
            None => 0usize,
        };
        offsets.insert(new_key, reg.end());
        return new_key;
    }
}


#[cfg(test)]
mod tests {
    use model::*;
    use sketch::Canvas;
    use sketch::decorator::Decorator;
    use std::collections::BTreeMap;

    struct DecoratorStub {}
    impl Decorator for DecoratorStub {
        fn draw<C: Canvas>(&self, canvas: &mut C, offset_y: f64) -> f64 {
            0.0
        }
    }

    #[test]
    fn test_find_offset_row() {
        let dec = DecoratorStub {};
        let mut map = BTreeMap::new();

        assert_eq!(
            dec.find_offset_row(&mut map, &SimpleRegion::new("ref", 10usize, 6usize)),
            0usize
        );
        assert_eq!(
            dec.find_offset_row(&mut map, &SimpleRegion::new("ref", 15usize, 6usize)),
            1usize
        );
        assert_eq!(
            dec.find_offset_row(&mut map, &SimpleRegion::new("ref", 20usize, 6usize)),
            0usize
        );
    }
}
