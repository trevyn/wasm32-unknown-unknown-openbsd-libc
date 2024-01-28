use std::env;

fn main() {
    if !env::var("TARGET").map_or(false, |t| t.starts_with("wasm")) {
        return;
    }

    // Apple clang doesn't support wasm32, so use Homebrew clang by default.
    if env::var("HOST") == Ok("x86_64-apple-darwin".to_string()) {
        if env::var("CC").is_err() {
            std::env::set_var("CC", "/usr/local/opt/llvm/bin/clang");
        }
        if env::var("AR").is_err() {
            std::env::set_var("AR", "/usr/local/opt/llvm/bin/llvm-ar");
        }
    } else if env::var("HOST") == Ok("aarch64-apple-darwin".to_string()) {
        if env::var("CC").is_err() {
            std::env::set_var("CC", "/opt/homebrew/opt/llvm/bin/clang");
        }
        if env::var("AR").is_err() {
            std::env::set_var("AR", "/opt/homebrew/opt/llvm/bin/llvm-ar");
        }
    }

    let sources = [
        "stdlib/heapsort.c",
        "stdlib/qsort.c",
        "string/bcmp.c",
        "string/bcopy.c",
        "string/bzero.c",
        "string/explicit_bzero.c",
        "string/ffs.c",
        "string/memccpy.c",
        "string/memchr.c",
        "string/memcmp.c",
        // "string/memcpy.c",
        "string/memmem.c",
        // "string/memmove.c",
        "string/memrchr.c",
        // "string/memset.c",
        "string/stpcpy.c",
        "string/stpncpy.c",
        "string/strcasecmp.c",
        "string/strcasecmp_l.c",
        "string/strcasestr.c",
        "string/strcat.c",
        "string/strchr.c",
        "string/strcmp.c",
        "string/strcoll.c",
        "string/strcpy.c",
        "string/strcspn.c",
        "string/strdup.c",
        "string/strerror.c",
        // "string/strerror_l.c",
        // "string/strerror_r.c",
        "string/strlcat.c",
        "string/strlcpy.c",
        "string/strlen.c",
        "string/strmode.c",
        "string/strncat.c",
        "string/strncmp.c",
        "string/strncpy.c",
        "string/strndup.c",
        "string/strnlen.c",
        "string/strpbrk.c",
        "string/strrchr.c",
        "string/strsep.c",
        // "string/strsignal.c",
        "string/strspn.c",
        "string/strstr.c",
        "string/strtok.c",
        "string/strxfrm.c",
        "string/strxfrm_l.c",
        "string/swab.c",
        "string/timingsafe_bcmp.c",
        "string/timingsafe_memcmp.c",
    ];

    let sources = sources
        .iter()
        .map(|f| format!("openbsd-src/lib/libc/{}", f))
        .collect::<Vec<_>>();

    cc::Build::new()
        .include("include")
        .files(sources)
        .file("src/errno.c")
        .flag("-w")
        .compile("wasm32-unknown-unknown-openbsd-libc");

    println!(
        "cargo:include={}/include",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );
}
