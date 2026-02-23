fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    compile_blueprint(&manifest_dir, "window");
    compile_blueprint(&manifest_dir, "navbar");
}

fn compile_blueprint(manifest_dir: &str, name: &str) {
    let blp = format!("{}/data/{}.blp", manifest_dir, name);
    let ui = format!("{}/data/{}.ui", manifest_dir, name);

    println!("cargo:rerun-if-changed={}", blp);

    let status = std::process::Command::new("blueprint-compiler")
        .args(["compile", "--output", &ui, &blp])
        .status()
        .expect("blueprint-compiler no encontrado");

    if !status.success() {
        panic!("Error compilando {}", blp);
    }
}
