import test from "ava";
import fs from "fs";

import { pdq } from "../index.js";

test("get pdq hash", (t) => {
  const buffer = fs.readFileSync("./test_data/bridge-1-original.jpg");
  const data = Uint8Array.from(buffer);
  t.is(pdq(data), "f8f8f0cee0f4a84f06370a22038f63f0b36e2ed596621e1d33e6b39c4e9c9b22");
});
