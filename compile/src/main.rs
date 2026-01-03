use std::process::Command;

fn main() {
    let status = Command::new("cargo")
        .args(&[
            "build",
            "--target=wasm32-wasi",
            "--release",
            "--example",
            "variables",
        ])
        .status()
        .unwrap();
    assert!(status.success());
    let status = Command::new("wasm-strip")
        .args(&["target/wasm32-wasi/release/examples/variables.wasm"])
        .status()
        .unwrap();
    assert!(status.success());
    let status = Command::new("wasm-opt")
        .args(&[
            "-Os",
            "target/wasm32-wasi/release/examples/variables.wasm",
            "-o",
            "target/wasm32-wasi/release/examples/variables.wasm",
        ])
        .status()
        .unwrap();
    assert!(status.success());
}
