import test from "ava";
import fs from "fs";

import { pdq } from "../index.js";

test("get pdq hash", (t) => {
  const buffer = fs.readFileSync("./test_data/bridge-1-original.jpg");
  const data = Uint8Array.from(buffer);
  // t.is(pdq(data), "f8f8f0cee0f4a84f06370a22038f63f0b36e2ed596621e1d33e6b39c4e9c9b22");
  t.is(pdq(data), "1111100011111000111100001100111011100000111101001010100001001111000001100011011100001010001000100000001110001111011000111111000010110011011011100010111011010101100101100110001000011110000111010011001111100110101100111001110001001110100111001001101100100010");
});

test("get pdq hash for invalid image", (t) => {
  const buffer = fs.readFileSync("./test_data/xml-as-img.png");
  const data = Uint8Array.from(buffer);
  t.is(pdq(data), "ERR");
});
