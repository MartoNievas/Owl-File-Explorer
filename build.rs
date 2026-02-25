fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    compile_blueprint(&manifest_dir, "window");
    compile_blueprint(&manifest_dir, "navbar");
    compile_blueprint(&manifest_dir, "side_panel");
    compile_blueprint(&manifest_dir, "content_panel");
}

fn compile_blueprint(manifest_dir: &str, name: &str) {
    let blp = format!("{}/data/{}.blp", manifest_dir, name);
    let ui = format!("{}/data/{}.ui", manifest_dir, name);
    println!("cargo:rerun-if-changed={}", blp);

    let output = std::process::Command::new("blueprint-compiler")
        .args(["compile", "--output", &ui, &blp])
        .output()
        .unwrap_or_else(|_| panic!("blueprint-compiler no encontrado"));

    if !output.status.success() {
        panic!(
            "Error compilando {}:\n{}",
            blp,
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
