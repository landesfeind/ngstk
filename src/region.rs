use std::fmt::Display;
use std::fmt::Debug;
use std::cmp::PartialEq;

pub use sequence::SequenceElement;

pub trait RegionIdentifier : Clone + Debug + Ord + Display {}

pub trait Region<T: RegionIdentifier, E : SequenceElement> {

    /// Returns the reference sequence name
    fn reference(&self) -> T;

    /// Returns the offset position of the genomic
    /// region (indexing starts with 0, inclusive)
    fn offset(&self) -> usize;

    /// Returns the end position of the genomic
    /// region (indexing starts with 0, exclusive)
    fn end(&self) -> usize;

    /// Returns the number of sequence elements
    /// that are covered by this genomic region.
    fn length(&self) -> usize {
        self.end() - self.offset()
    }

    /// Returns true if two genomic regions are on the same template sequence
    fn is_located_on_same_template(&self, other: &Self) -> bool {
        self.reference() == other.reference()
    }

    /// Intersects two genomic regions and return the start and end positions
    /// of the intersection. In case the regions do not overlap, `None` is returned.
    fn intersect(&self, other: &Self) -> Option<(usize,usize)> {
        if ! self.is_located_on_same_template(other) {
            return None
        }
        let s = if self.offset() < other.offset() { // self starts before other
            if self.end() < other.offset() { // self also ends before other
                None
            }
            else {
                other.offset()
            }
        }
        else { // other starts before self
            if other.end() < self.offset() {
                None
            }
            else {
                self.offset()
            }
        };

        let e = if self.end() < other.end() { // self starts before other
            if self.end() < other.offset() { // self also ends before other
                None
            }
            else {
                other.end()
            }
        }
        else { // other starts before self
            if other.end() < self.offset() {
                None
            }
            else {
                self.end()
            }
        };

        if s.is_some() && e.is_some() {
            (s.unwrap(), e.unwrap())
        }
        else {
            None
        }
    }

    /// Returns the lower bound of the union, that is the 
    /// base where an union of two genomic regions does start.
    /// Returns `None` if the regions do not overlap.
    fn union(&self, other: &Self) -> Option<(usize, usize)> {
        if ! self.is_located_on_same_template(other) {
            return None
        }
        
        let s = if self.offset() < other.offset() { // self starts before other
            if self.end() < other.offset() { // self also ends before other
                None
            }
            else {
                self.offset()
            }
        }
        else { // other starts before self
            if other.end() < self.offset() {
                None
            }
            else {
                other.offset()
            }
        };

        let e = if self.end() < other.end() { // self starts before other
            if self.end() < other.offset() { // self also ends before other
                None
            }
            else {
                other.end()
            }
        }
        else { // other starts before self
            if other.end() < self.offset() {
                None
            }
            else {
                self.end()
            }
        };

        if s.is_some() && e.is_some() {
            (s.unwrap(), e.unwrap())
        }
        else {
            None
        }
    }
}
