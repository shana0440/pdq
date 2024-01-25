#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::Uint8Array;
use pdqhash;

#[napi]
pub fn pdq(buf: Uint8Array) -> String {
  let hash = pdqhash::generate_pdq_full_size(&pdqhash::image::load_from_memory(&buf).unwrap()).0;

  let mut in_bin = "".to_string();

  for p in hash.clone() {
    in_bin += &format!("{:08b}", p);
  }

  in_bin
}