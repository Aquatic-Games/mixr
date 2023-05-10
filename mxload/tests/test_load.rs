use mxload::{stream::Wav, AudioStream};

#[test]
fn test_wav() {
    let mut wav = Wav::from_file("/home/ollie/Music/necros_-_introspection.wav").unwrap();

    println!("{:#?}", wav.format());

    let mut full_buffer = Vec::new();

    let mut buf: Vec<u8> = std::iter::repeat(0).take(24000).collect();

    while wav.get_buffer(&mut buf).is_ok() {
        full_buffer.append(&mut buf);
    }
}