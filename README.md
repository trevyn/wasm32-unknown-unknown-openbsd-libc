# wasm32-unknown-unknown-openbsd-libc

A small chunk of OpenBSD's libc, conveniently packaged as a Rust crate.

Created for compiling C code that relies on a reasonable subset of libc to `wasm32-unknown-unknown`.

```toml
[dependencies]
wasm32-unknown-unknown-openbsd-libc = "0.1"

[build-dependencies]
wasm32-unknown-unknown-openbsd-libc = "0.1"
```

In your `build.rs` using `cc`:

```rust
cfg.includes(wasm32_unknown_unknown_openbsd_libc::includes());

println!("cargo:rustc-link-lib=wasm32-unknown-unknown-openbsd-libc");
```
