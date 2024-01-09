pub fn includes() -> Vec<impl AsRef<std::path::Path>> {
    let dir = env!("CARGO_MANIFEST_DIR");
    vec![
        format!("{}/openbsd-src/sys", dir),
        format!("{}/openbsd-src/include", dir),
        format!("{}/include", dir),
    ]
}
