fn main() {
    // Read the HOLOCHAIN_ARC_FACTOR environment variable
    let arc_factor = std::env::var("HOLOCHAIN_ARC_FACTOR").unwrap_or_else(|_| "".to_string());

    // Generate a Rust file with the arc factor value
    let code = format!(
        r#"
        pub const HOLOCHAIN_ARC_FACTOR: &str = "{}";
        "#,
        arc_factor
    );

    // Write to a generated file
    std::fs::write("src/generated_arc_factor.rs", code).expect("Failed to write generated file");

    // Tell Cargo to rerun this build script if the environment variable changes
    println!("cargo:rerun-if-env-changed=HOLOCHAIN_ARC_FACTOR");

    // This is essential for Tauri to work properly
    tauri_build::build()
}
