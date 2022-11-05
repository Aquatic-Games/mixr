use mixr;

#[test]
pub fn load_wave() {
    let mut reader = mixr::binary_reader::BinaryReader::new("/home/ollie/Music/Always There.wav");
    println!("{}", reader.read_string(4));
    println!("{}", reader.read_i32());
    println!("{}", reader.read_string(4));
    println!("{}", reader.read_string(4));
}