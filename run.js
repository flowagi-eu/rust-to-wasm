import fs from "fs";

// -----------------------------
// Load WASM
// -----------------------------
async function loadWasm(path) {
  const wasmBytes = fs.readFileSync(path);
  const { instance } = await WebAssembly.instantiate(wasmBytes, {});
  return instance;
}

// -----------------------------
// Run WASM plugin
// -----------------------------
async function runWasm(instance, inputJson) {
  const memory = instance.exports.memory;
  const memBuffer = new Uint8Array(memory.buffer);

  const encoder = new TextEncoder();
  const inputBytes = encoder.encode(JSON.stringify(inputJson));

  // Write input JSON at offset 0
  memBuffer.set(inputBytes, 0);

  // Reserve 16 bytes for Output struct
  const outPtr = inputBytes.length + 16;

  // Call run(ptr, len, out_ptr)
  instance.exports.run(0, inputBytes.length, outPtr);

  // Read Output struct (offset + len) from memory
  const view = new DataView(memory.buffer, outPtr, 8);
  const offset = view.getUint32(0, true);
  const len = view.getUint32(4, true);

  console.debug("[DEBUG] Input JSON:", JSON.stringify(inputJson));
  console.debug("[DEBUG] Output offset:", offset, "length:", len);

  const outputBytes = new Uint8Array(memory.buffer, offset, len);
  const outputStr = new TextDecoder().decode(outputBytes);

  console.debug("[DEBUG] Output JSON string:", outputStr);

  const parsed = JSON.parse(outputStr);

  return parsed;
}

// -----------------------------
// Example usage
// -----------------------------
(async () => {
  const instance = await loadWasm("build/rust_plugin.wasm");

  const input1 = { args: [10, "lower than", 5], context: { set_context: "prev" } };
  const out1 = await runWasm(instance, input1);
  console.log("Rust plugin eval 1:", out1);

  const input2 = { args: [7, "higher than", 3], context: { set_context: "prev" } };
  const out2 = await runWasm(instance, input2);
  console.log("Rust plugin eval 2:", out2);
})();
