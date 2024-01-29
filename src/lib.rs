#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::Uint8Array;
use pdqhash;

#[napi]
pub fn pdq(buf: Uint8Array) -> String {
  let img_result = &pdqhash::image::load_from_memory(&buf);

  let str: String = match img_result {
    Ok(img) => {
      let hash = pdqhash::generate_pdq_full_size(img).0;
    
      let mut in_bin = "".to_string();
    
      for p in hash.clone() {
        in_bin += &format!("{:08b}", p);
      }
    
      in_bin

    },
    Err(error) => {
      eprintln!("Failed to load image: {}", error);

      "ERR".to_string()
    }
  };

  str
}