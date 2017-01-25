use flate2::read::GzDecoder;
use std::io::prelude::*;
use std::io;

pub fn decompress_gz(encoded: &[u8]) -> io::Result<Vec<u8>> {
    let mut decoded = Vec::new();
    try!(try!(GzDecoder::new(encoded)).read_to_end(&mut decoded));
    Ok(decoded)
}
