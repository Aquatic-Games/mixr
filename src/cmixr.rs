use crate::{system::{AudioSystem}, AudioFormat};

/*#[repr(C)]
pub struct CPCM {
    pub data: *const u8,
    pub data_length: usize,
    pub format: CAudioFormat
}*/

#[no_mangle]
pub extern "C" fn mxCreateSystem(format: AudioFormat, channels: u16) -> *const AudioSystem {
    Box::into_raw(Box::new(AudioSystem::new(Some(format), channels)))
}

#[no_mangle]
pub extern "C" fn mxDeleteSystem(system: *mut AudioSystem) {
    unsafe { Box::from_raw(system) };
}

#[no_mangle]
pub extern "C" fn mxCreateBuffer(system: &mut AudioSystem) -> i32 {
    system.create_buffer()
}
    

#[no_mangle]
pub extern "C" fn mxUpdateBuffer(system: &mut AudioSystem, buffer: i32, data: *const u8, data_length: usize, format: AudioFormat) {
    let data = unsafe { std::slice::from_raw_parts(data, data_length).to_vec() };

    assert_eq!(data_length, data.len(), "data_length, {}, does not equal the converted data length, {}", data_length, data.len());

    system.update_buffer(buffer, &data, format);
}

#[no_mangle]
pub extern "C" fn mxPlayBuffer(system: &mut AudioSystem, channel: u16, buffer: i32, volume: f64, speed: f64, panning: f64) {
    system.play_buffer(channel, buffer, volume, speed, panning);
}

#[no_mangle]
pub extern "C" fn mxAdvance(system: &mut AudioSystem) -> i16 {
    system.advance()
}

/*#[no_mangle]
pub unsafe extern "C" fn mxPCMLoadWav(path: *const i8) -> CPCM {
    let pcm = PCM::load_wav(CStr::from_ptr(path).to_str().unwrap()).unwrap();
    
    CPCM {
        data: Box::into_raw(Box::new(pcm.data.as_ptr().clone())) as *const u8,
        data_length: pcm.data.len(),
        format: CAudioFormat { 
            channels: pcm.format.channels.unwrap(), 
            sample_rate: pcm.format.sample_rate.unwrap(), 
            bits_per_sample: pcm.format.bits_per_sample.unwrap() 
        }
    }
}*/