// run.js
import fs from 'fs';

// -----------------------------
// Preload WASM plugin once
// -----------------------------
async function loadWasm(path) {
  const wasmBytes = fs.readFileSync(path);
  const { instance } = await WebAssembly.instantiate(wasmBytes, {});
  return instance;
}

// -----------------------------
// Call WASM plugin with any JSON input
// -----------------------------
async function runWasm(instance, inputJson, funcName = 'run') {
  const memory = instance.exports.memory;

  // Encode input JSON into WASM memory at offset 0
  const encoder = new TextEncoder();
  const inputBytes = encoder.encode(JSON.stringify(inputJson));
  new Uint8Array(memory.buffer).set(inputBytes, 0);

  // Call Rust run(ptr, len) function
  const ptr = instance.exports[funcName](0, inputBytes.length);

  // Get length of output JSON from Rust
  const len = instance.exports.get_last_output_len();

  // Read JSON output from WASM memory
  const outputBytes = new Uint8Array(memory.buffer, ptr, len);
  const outputStr = new TextDecoder().decode(outputBytes);

  // Parse JSON
  let output;
  try {
    output = JSON.parse(outputStr);
  } catch (err) {
    console.error('[WASM] Failed to parse JSON:', outputStr);
    throw err;
  }

  // -----------------------------
  // Debug info (optional)
  // -----------------------------
  console.debug('[DEBUG] Input JSON:', JSON.stringify(inputJson));
  console.debug('[DEBUG] Output pointer:', ptr, 'length:', len);
  console.debug('[DEBUG] Output JSON string:', outputStr);

  return output;
}

// -----------------------------
// Example usage
// -----------------------------
(async () => {
  // Preload WASM once per worker
  const instance = await loadWasm('build/rust_plugin.wasm');

  // Example 1
  const input1 = { args: [10, 'lower than', 5], context: { set_context: 'prev' } };
  const out1 = await runWasm(instance, input1);
  console.log('Rust plugin eval 1:', out1);

  // Example 2
  const input2 = { args: [7, 'higher than', 3], context: { set_context: 'prev' } };
  const out2 = await runWasm(instance, input2);
  console.log('Rust plugin eval 2:', out2);
})();
