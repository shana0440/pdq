#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::Uint8Array;
use pdqhash;
use hex;

#[napi]
pub fn pdq(buf: Uint8Array) -> String {
  let hash = pdqhash::generate_pdq_full_size(&pdqhash::image::load_from_memory(&buf).unwrap()).0;
  hex::encode(hash)
}