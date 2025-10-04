import { eventHandler } from 'h3';
import * as rust from 'rust-wasm';

export default eventHandler(async () => {
  try {
    return rust.hello_world();
  } catch (error) {
    console.error('Error loading or executing WASM:', error);

    return `
      <h1>Error: Could not load Rust WASM module</h1>
      <p>Make sure to build the Rust WASM module first: <code>cd rust && wasm-pack build --target bundler --out-dir pkg</code></p>
      <p>Error details: ${error instanceof Error ? error.message : 'Unknown error'}</p>
    `;
  }
});
