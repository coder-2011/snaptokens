use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

const TOKENDAGGER_REVISION: &str = "1eed815d1ef3a23a73b0d6554f99fc0be6ff4164";

fn bridge_build(pcre2_includes: &[PathBuf]) -> cc::Build {
    let mut build = cc::Build::new();
    build.cpp(true);
    build.std("c++17");
    build.warnings(false);
    build.include("native");
    for include in pcre2_includes {
        build.include(include);
    }
    build
}

fn emit_compiler_metadata(build: &cc::Build) {
    let compiler = build.get_compiler();
    let compiler_path = resolve_executable(compiler.path());
    let output = Command::new(&compiler_path)
        .arg("--version")
        .output()
        .expect("failed to query the C++ compiler");
    assert!(output.status.success(), "C++ compiler --version failed");
    let version = String::from_utf8_lossy(&output.stdout)
        .lines()
        .next()
        .unwrap_or("unknown")
        .to_owned();
    let arguments = compiler
        .args()
        .iter()
        .map(|argument| argument.to_string_lossy())
        .collect::<Vec<_>>()
        .join(" ");
    println!(
        "cargo:rustc-env=TOKENDAGGER_CXX_PATH={}",
        compiler_path.display()
    );
    println!("cargo:rustc-env=TOKENDAGGER_CXX_VERSION={version}");
    println!("cargo:rustc-env=TOKENDAGGER_CXX_ARGS={arguments}");
}

fn resolve_executable(path: &Path) -> PathBuf {
    if path.components().count() > 1 {
        return fs::canonicalize(path).unwrap_or_else(|_| path.to_owned());
    }
    if let Some(paths) = env::var_os("PATH") {
        for directory in env::split_paths(&paths) {
            let candidate = directory.join(path);
            if candidate.is_file() {
                return fs::canonicalize(&candidate).unwrap_or(candidate);
            }
        }
    }
    path.to_owned()
}

fn compile_tokendagger(pcre2_includes: &[PathBuf]) {
    let source = format!("vendor/tokendagger-{TOKENDAGGER_REVISION}/src/tiktoken");
    let mut build = bridge_build(pcre2_includes);
    build.file("native/tokendagger_bridge.cpp");
    build.include(source);
    build.opt_level(3);
    match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("x86") | Ok("x86_64") => {
            build.flag("-march=native");
            build.flag("-mtune=native");
        }
        Ok("aarch64") => {
            build.flag("-mcpu=native");
        }
        _ => {}
    }
    emit_compiler_metadata(&build);
    build.compile("snaptokens_tokendagger_bridge");
}

fn pcre2_includes() -> Vec<PathBuf> {
    let mut prefixes = Vec::new();
    if let Some(prefix) = env::var_os("PCRE2_DIR") {
        prefixes.push(PathBuf::from(prefix));
    }
    prefixes.push(PathBuf::from("/opt/homebrew/opt/pcre2"));
    prefixes.push(PathBuf::from("/usr/local/opt/pcre2"));
    for prefix in prefixes {
        let header = prefix.join("include/pcre2.h");
        if header.is_file() {
            println!(
                "cargo:rustc-link-search=native={}",
                prefix.join("lib").display()
            );
            return vec![prefix.join("include")];
        }
    }
    Vec::new()
}

fn main() {
    let pcre2_includes = pcre2_includes();
    println!("cargo:rustc-link-lib=pcre2-8");
    compile_tokendagger(&pcre2_includes);

    println!("cargo:rerun-if-changed=native");
    println!("cargo:rerun-if-changed=vendor");
    println!("cargo:rerun-if-env-changed=PCRE2_DIR");
    println!("cargo:rustc-env=TOKENDAGGER_REVISION={TOKENDAGGER_REVISION}");
}
