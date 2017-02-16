//! Decompresses a gz-stream to a string.
use flate2::read::GzDecoder;
use std::io::prelude::*;
use std::io;

fn decompress_gz(encoded: &[u8]) -> io::Result<Vec<u8>> {
    let mut decoded = Vec::new();
    try!(try!(GzDecoder::new(encoded)).read_to_end(&mut decoded));
    Ok(decoded)
}

pub fn decompress(compressed: &[u8]) -> io::Result<String> {
    let decompressed = try!(decompress_gz(compressed));
    let mut string = String::new();
    for c in decompressed {
        string.push(c as char);
    }
    Ok(string)
}
