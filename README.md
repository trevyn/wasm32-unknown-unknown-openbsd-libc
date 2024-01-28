# wasm32-unknown-unknown-openbsd-libc

A small chunk of OpenBSD's libc, conveniently packaged as a Rust crate.

Created for compiling C code that relies on a reasonable subset of libc to `wasm32-unknown-unknown`.

```toml
[target.wasm32-unknown-unknown.dependencies]
wasm32-unknown-unknown-openbsd-libc = "0.2"
```

In your `build.rs` using `cc`:

```rust
cfg.include(std::env::var_os("DEP_WASM32_UNKNOWN_UNKNOWN_OPENBSD_LIBC_INCLUDE").unwrap());

println!("cargo:rustc-link-lib=wasm32-unknown-unknown-openbsd-libc");
```
