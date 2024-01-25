# PDQ

This is a node.js library wrapping [PDQ Hash](https://github.com/darwinium-com/pdqhash/tree/main).

## How to use

```
import { pdq } from "pdq";

const buffer = fs.readFileSync("./test_data/bridge-1-original.jpg");
const data = Uint8Array.from(buffer);
const hash = pdq(data);
```
