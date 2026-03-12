use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
struct Input {
    args: Vec<Value>,
    context: Value,
}


// -----------------------------
// Core run function
// -----------------------------
#[unsafe(no_mangle)]
pub extern "C" fn run(ptr: u32, len: u32, out_ptr: u32) {
    // Safety: read input from ptr..ptr+len in WASM memory
    let input_bytes = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };

    let input: Input = match serde_json::from_slice(input_bytes) {
        Ok(i) => i,
        Err(_) => return write_output(out_ptr as usize, json!({"error": "invalid JSON input"})),
    };

    // Plugin logic
    let result = if input.args.len() >= 3 {
        let left = input.args[0].as_f64().unwrap_or(0.0);
        let cond = input.args[1].as_str().unwrap_or("");
        let right = input.args[2].as_f64().unwrap_or(0.0);
        match cond {
            "lower than" => left < right,
            "higher than" => left > right,
            "equal to" => (left - right).abs() < 1e-9,
            _ => false,
        }
    } else {
        false
    };

    let output_json = json!({
        "args": input.args,
        "context": input.context,
        "result": result
    });

    write_output(out_ptr as usize, output_json);
}

// -----------------------------
// Write output JSON into WASM memory (after out_ptr)
// -----------------------------
fn write_output(out_ptr: usize, value: Value) {
    let bytes = serde_json::to_vec(&value).unwrap();

    // Use memory starting at out_ptr + 16 for output JSON
    let offset = out_ptr + 16;
    let len = bytes.len();

    // Safety: write directly into linear memory
    let mem = unsafe { std::slice::from_raw_parts_mut(offset as *mut u8, len) };
    mem.copy_from_slice(&bytes);

    // Write OutputStruct (offset + len) at out_ptr
    let out_mem = unsafe { std::slice::from_raw_parts_mut(out_ptr as *mut u8, 8) };
    out_mem[0..4].copy_from_slice(&(offset as u32).to_le_bytes());
    out_mem[4..8].copy_from_slice(&(len as u32).to_le_bytes());
}

