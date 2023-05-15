pub struct AudioSystem {
    system: mixr::AudioSystem
}

#[no_mangle]
pub extern "C" fn mxCreateSystem(sample_rate: u32, voices: u16) -> *mut AudioSystem {
    let system = mixr::AudioSystem::new(sample_rate, voices);

    Box::into_raw(Box::new(AudioSystem { system }))
}

#[no_mangle]
pub extern "C" fn mxDestroySystem(system: &mut AudioSystem) {
    std::mem::drop(system)
}