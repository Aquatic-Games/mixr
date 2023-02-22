fn main() {
    println!("cargo:rerun-if-changed=src/loaders/stb_vorbis.c");
    cc::Build::new()
        .file("src/loaders/stb_vorbis.c")
        .compile("loaders");
}