fn main() {
    println!("cargo:rerun-if-changed=../unyt/workdir/unyt.happ");
    tauri_build::build()
}
