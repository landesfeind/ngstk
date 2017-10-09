mod record;
mod reader;
mod stream;
mod index;
mod writer;

pub use self::record::FastaRecord;
pub use self::reader::FastaReader;
pub use self::index::IndexedFastaFile;
pub use self::stream::FastaStream;
pub use self::writer::FastaWriter;

use std::io::Read;

pub fn read_stream<R: Read>(input: R) -> FastaStream<R> {
    FastaStream::from(input)
}

