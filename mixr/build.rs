fn main() {
    cc::Build::new()
        .file("../stb/stb_vorbis.c")
        .compile("stb");
}