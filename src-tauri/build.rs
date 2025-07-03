fn main() {
    println!("cargo:rerun-if-changed=../domino/workdir/domino.happ");
    tauri_build::build()
}
