import * as wasm from "../dist/rust_wasm.js";

(async () => {
  const message = "🦀";
  const bytes = wasm.string_to_bytes(message);
  console.log(bytes);
})().catch((e) => {
  console.error(e);
});
