# Rust to WASM
Simple Rust to WASM interface (v0.1) that works well with NodeJS/Bun.

Goal: One simple fast interface for using WASM (created by Rust) in NodeJS/Bun backends/engines.

Status: Experimental (awaiting community feedback) 

## Usage:
```
bash build_and_run.sh 
```

Should output something like:
```
    Finished `release` profile [optimized] target(s) in 0.02s
[DEBUG] Input JSON: {"args":[10,"lower than",5],"context":{"set_context":"prev"}}
[DEBUG] Output pointer: 1115696 length: 76
[DEBUG] Output JSON string: {"args":[10,"lower than",5],"context":{"set_context":"prev"},"result":false}
Rust plugin eval 1: {
  args: [ 10, 'lower than', 5 ],
  context: { set_context: 'prev' },
  result: false
}
[DEBUG] Input JSON: {"args":[7,"higher than",3],"context":{"set_context":"prev"}}
[DEBUG] Output pointer: 1115832 length: 75
[DEBUG] Output JSON string: {"args":[7,"higher than",3],"context":{"set_context":"prev"},"result":true}
Rust plugin eval 2: {
  args: [ 7, 'higher than', 3 ],
  context: { set_context: 'prev' },
  result: true
}
```
