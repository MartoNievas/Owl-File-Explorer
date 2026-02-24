fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    compile_blueprint(&manifest_dir, "window");
    compile_blueprint(&manifest_dir, "navbar");
    compile_blueprint(&manifest_dir, "side_panel");
}

fn compile_blueprint(manifest_dir: &str, name: &str) {
    let blp = format!("{}/data/{}.blp", manifest_dir, name);
    let ui = format!("{}/data/{}.ui", manifest_dir, name);

    println!("cargo:rerun-if-changed={}", blp);

    // Sintaxis nueva
    let status = std::process::Command::new("blueprint-compiler")
        .args(["compile-file", "--output", &ui, &blp])
        .status()
        .unwrap_or_else(|_| panic!("blueprint-compiler no encontrado"));

    if !status.success() {
        // Si falla con compile-file, probá con batch
        let status2 = std::process::Command::new("blueprint-compiler")
            .args([
                "batch-compile",
                &format!("{}/data", manifest_dir),
                &format!("{}/data", manifest_dir),
                &blp,
            ])
            .status()
            .unwrap_or_else(|_| panic!("blueprint-compiler falló"));

        if !status2.success() {
            panic!("Error compilando {}", blp);
        }
    }
}
