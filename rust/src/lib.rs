use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello_world() -> String {
    r#"
        <meta charset="utf-8">
        <h1>This is your brand new Nitro project 🚀</h1>
        <p>Get started by editing the <code>server/routes/index.ts</code> file.</p>
        <p>Learn more from 📖 <a href="https://nitro.build/guide" target="_blank">Nitro Documentation</a></p>
        <p><strong>Now powered by Rust WebAssembly!</strong> 🦀</p>
        <p>This content is generated directly from Rust compiled to WASM.</p>
    "#.to_string()
}
