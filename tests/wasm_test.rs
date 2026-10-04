use std::fs;
use std::path::Path;

#[test]
fn test_cargo_toml_wasm_configuration() {
    let cargo_toml = fs::read_to_string("Cargo.toml").expect("Cargo.toml readable");
    assert!(
        cargo_toml.contains("cdylib"),
        "Cargo.toml must configure cdylib in crate-type for WASM"
    );
    assert!(
        cargo_toml.contains("wasm = ["),
        "Cargo.toml must define the 'wasm' feature"
    );
    assert!(
        cargo_toml.contains("wasm-bindgen"),
        "Cargo.toml must include wasm-bindgen dependency"
    );
}

#[test]
fn test_wasm_artifacts_and_html_integration() {
    let wasm_js = Path::new("site/pkg/ayvu.js");
    let wasm_bg = Path::new("site/pkg/ayvu_bg.wasm");

    assert!(wasm_js.is_file(), "site/pkg/ayvu.js should exist");
    assert!(wasm_bg.is_file(), "site/pkg/ayvu_bg.wasm should exist");

    let html = fs::read_to_string("site/index.html").expect("site/index.html readable");
    assert!(
        html.contains("render_comic") || html.contains("./pkg/ayvu.js"),
        "site/index.html should integrate WASM module"
    );
}
