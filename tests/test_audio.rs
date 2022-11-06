use mixr;

#[test]
fn test_wav() {
    let mut format = mixr::AudioFormat {
        sample_rate: None,
        channels: None
    };

    let mut system = mixr::system::AudioSystem::new(&mut format);

    let pcm = mixr::loaders::PCM::load_wav("/home/ollie/Music/Always There.wav").unwrap();

    let buffer = system.create_buffer();
    system.update_buffer(&buffer, pcm.data, pcm.format);
}