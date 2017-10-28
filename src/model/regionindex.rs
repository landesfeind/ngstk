


pub trait RegionIndex {

	fn file_offset_for_template<S: ToString>(s: &S) -> Option<usize>;

	fn file_offset_for_region<R: Region>(r: &R) -> Option<usize>;

}