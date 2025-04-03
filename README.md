# WSON WASM
WSON for the Web, powered by Rust + WebAssembly.
Parse and validate WSON data directly in the browser or any WASM-compatible environment.

WSON (Wave Serialized Object Notation) is a flexible and human-friendly data format that supports comments, version numbers, dates, and much more—while maintaining full compatibility with JSON syntax.

This crate allows you to use WSON in frontend applications by compiling the core WSON parser into WebAssembly.

## Features
* ✅ Validate WSON strings (wson_validate)

* ✅ Parse WSON strings into normalized WSON text (wson_parse)

* ✅ Compatible with both WSON and standard JSON input

* ✅ Fast and lightweight

* ✅ Built with Rust and compiled to WASM using wasm-bindgen

## Example Usage (in JavaScript)
After compiling with `wasm-pack`:

Import the module and use the functions in any modern browser or frontend project.

```js
import init, { wson_validate, wson_parse } from './pkg/wson_wasm.js';

await init();

const input = `
{
  "name": "Luna",
  "age": 20,
  "tags": ["dev", "wave"],
  "is_active": true
}
`;

if (wson_validate(input)) {
  const output = wson_parse(input);
  console.log("Valid WSON:");
  console.log(output);
} else {
  console.error("Invalid WSON format");
}
```

## Build Instructions
To compile the library to WebAssembly:

1. Install wasm-pack if you haven't already:

    `cargo install wasm-pack`

2. Build the project:

    `wasm-pack build --target web`

The output will be available in the `pkg/` directory and can be directly used in frontend projects.