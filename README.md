# rust-wasm

#### How to build
- `npm i`
- `npm run build`

#### Usage
```js
import wasmInit, * as wasm from "./dist/rust_wasm.js";

(async () => {
  await wasmInit();

  const message = "🦀";
  const bytes = wasm.string_to_bytes(message);
  console.log(bytes);
})().catch((e) => {
  console.error(e);
});
```
