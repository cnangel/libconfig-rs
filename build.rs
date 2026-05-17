use std::env;
use std::path::PathBuf;

fn main() {
    // Register custom cfg flags for Rust 1.80+ check-cfg
    println!("cargo::rustc-check-cfg=cfg(libconfig_1_1)");
    println!("cargo::rustc-check-cfg=cfg(libconfig_1_4)");
    println!("cargo::rustc-check-cfg=cfg(libconfig_1_5)");
    println!("cargo::rustc-check-cfg=cfg(libconfig_1_8)");
    println!("cargo::rustc-check-cfg=cfg(libconfig_ver_major, values(\"1\"))");
    println!("cargo::rustc-check-cfg=cfg(libconfig_ver_minor, values(any()))");
    println!("cargo::rustc-check-cfg=cfg(libconfig_ver, values(any()))");

    let lib = match pkg_config::Config::new()
        .atleast_version("1.1")
        .probe("libconfig")
    {
        Ok(lib) => lib,
        Err(e) => {
            println!("cargo:warning=libconfig not found via pkg-config: {}", e);
            println!("cargo:warning=Building with minimal API (no version-gated features)");
            return;
        }
    };

    let version = &lib.version;
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() < 2 {
        println!("cargo:warning=Cannot parse libconfig version: {}", version);
        return;
    }

    let major: u32 = parts[0].parse().unwrap_or(1);
    let minor: u32 = parts[1].parse().unwrap_or(0);

    println!("cargo:rustc-cfg=libconfig_ver_major=\"{}\"", major);
    println!("cargo:rustc-cfg=libconfig_ver_minor=\"{}\"", minor);

    // Emit cumulative cfg flags for each minor version
    for m in 1..=minor {
        println!("cargo:rustc-cfg=libconfig_{}_{}", major, m);
    }

    // Emit the full version for runtime checks
    println!("cargo:rustc-cfg=libconfig_ver=\"{}\"", version);

    // Build the C wrapper that exposes macro-only functions
    let wrapper_src = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src")
        .join("wrapper.c");

    let mut build = cc::Build::new();
    build.file(&wrapper_src);

    // Add libconfig include paths from pkg-config
    for path in &lib.include_paths {
        build.include(path);
    }

    build.compile("wrapper");

    // Re-run build.rs if the wrapper sources change
    println!("cargo:rerun-if-changed=src/wrapper.c");
    println!("cargo:rerun-if-changed=src/wrapper.h");
    for path in &lib.link_paths {
        println!("cargo:rerun-if-changed={}", path.display());
    }
}