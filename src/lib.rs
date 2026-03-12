// src/lib.rs
use serde::{Deserialize};
use serde_json::{Value, json};
use std::cell::Cell;
use std::slice;

// -----------------------------
// Input structure from Node.js
// -----------------------------
#[derive(Deserialize)]
struct Input {
    args: Vec<Value>, // can hold numbers, strings, etc.
    context: Value,   // any JSON object
}

// Thread-local storage to track last output length per call
thread_local! {
    static LAST_LEN: Cell<usize> = Cell::new(0);
}

// -----------------------------
// Core run function
// -----------------------------
#[unsafe(no_mangle)]
pub extern "C" fn run(ptr: *const u8, len: usize) -> *const u8 {
    // Convert raw pointer + length into Rust slice
    let input_bytes = unsafe { slice::from_raw_parts(ptr, len) };

    // Deserialize JSON input
    let input: Input = match serde_json::from_slice(input_bytes) {
        Ok(i) => i,
        Err(_) => {
            // Return error JSON if deserialization fails
            let err = json!({"error": "invalid JSON input"});
            let err_bytes = serde_json::to_vec(&err).unwrap();
            LAST_LEN.with(|cell| cell.set(err_bytes.len()));
            let ptr = err_bytes.as_ptr();
            std::mem::forget(err_bytes);
            return ptr;
        }
    };

    // -----------------------------
    // Example plugin logic: "add" or numeric evaluation
    // -----------------------------
    let result = if input.args.len() >= 3 {
        let left = input.args[0].as_f64().unwrap_or(0.0);
        let cond = input.args[1].as_str().unwrap_or("");
        let right = input.args[2].as_f64().unwrap_or(0.0);

        let res = match cond {
            "lower than" => left < right,
            "higher than" => left > right,
            "equal to" => (left - right).abs() < f64::EPSILON,
            _ => false,
        };
        res
    } else {
        false
    };

    // -----------------------------
    // Build output JSON
    // -----------------------------
    let output = json!({
        "args": input.args,
        "context": input.context,
        "result": result
    });

    // Serialize output JSON into Vec<u8> (heap allocation per call)
    let output_bytes = serde_json::to_vec(&output).unwrap();

    // Save length in thread-local storage for Node.js
    LAST_LEN.with(|cell| cell.set(output_bytes.len()));

    // Return pointer to memory
    let ptr = output_bytes.as_ptr();
    std::mem::forget(output_bytes); // Node reads memory
    ptr
}

// -----------------------------
// Helper for Node.js to get length
// -----------------------------
#[unsafe(no_mangle)]
pub extern "C" fn get_last_output_len() -> u32 {
    LAST_LEN.with(|cell| cell.get() as u32)
}
