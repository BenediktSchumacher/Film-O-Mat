pub mod download;

extern crate curl;
extern crate flate2;

use download::*;
use download::decompressor::*;

fn main() {
    println!("Hello, world!");
}
