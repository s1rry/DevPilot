//! Build script for `devpilot-git`.
//!
//! The vendored libgit2 build uses Win32 SID, token, registry and legacy
//! CryptoAPI functions (all exported from `advapi32.dll`) but does not emit
//! the corresponding link directive, so an MSVC link fails with unresolved
//! externals like `OpenProcessToken` and `CryptAcquireContextA`. Linking
//! `advapi32` explicitly resolves them. The directive is inherited by any
//! binary that depends on this crate.

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        println!("cargo:rustc-link-lib=dylib=advapi32");
    }
}
