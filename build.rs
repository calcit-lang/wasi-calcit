fn main() {
  let target = std::env::var("TARGET").expect("Cargo provides TARGET");
  if target.starts_with("wasm32-wasi") {
    // Recursive preprocessing of the bundled Calcit core exceeds wasm-ld's
    // default 1 MiB shadow stack. This host owns its final binary link flags.
    println!("cargo:rustc-link-arg-bin=wasi-calcit=-zstack-size=4194304");
  }
}
