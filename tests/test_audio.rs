use mixr;

#[test]
fn test_wav() {
    let mut format = mixr::system::AudioFormat {
        sample_rate: None,
        channels: None
    };

    let device = mixr::system::AudioSystem::new(&mut format);

    println!("{:#?}", device.format);
}