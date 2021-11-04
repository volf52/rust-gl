# Testing Rust with WebGLs

---

- To link the wasm output (`pkg`) with the TS app (as `rust-gl`), run `yarn link` from inside `pkg` to register it, and then `yarn link rust-gl` in the base dir.
