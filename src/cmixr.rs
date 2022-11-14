use std::{ffi::{c_void, CString, CStr}, mem::transmute};

use crate::{system::{AudioSystem}, AudioFormat, loaders::PCM};

#[repr(C)]
pub struct CAudioFormat {
    pub channels: u8,
    pub sample_rate: i32,
    pub bits_per_sample: u8
}

#[repr(C)]
pub struct CPCM {
    pub data: *const u8,
    pub data_length: usize,
    pub format: CAudioFormat
}

#[no_mangle]
pub extern "C" fn mxCreateSystem(format: Option<&CAudioFormat>, channels: u16) -> *const AudioSystem {
    let mut i_fmt = if format.is_none() { 
        AudioFormat { channels: None, sample_rate: None, bits_per_sample: None } 
    } 
    else { 
        let fmt = format.unwrap();
        AudioFormat { channels: Some(fmt.channels), sample_rate: Some(fmt.sample_rate), bits_per_sample: Some(fmt.bits_per_sample) } 
    };

    Box::into_raw(Box::new(AudioSystem::new(&mut i_fmt, channels)))
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
pub extern "C" fn mxUpdateBuffer(system: &mut AudioSystem, buffer: i32, data: *const u8, data_length: usize, format: &CAudioFormat) {
    println!("Got system");
    let data = unsafe { std::slice::from_raw_parts(data, data_length).to_vec() };
    println!("Got data");

    assert_eq!(data_length, data.len(), "data_length, {}, does not equal the converted data length, {}", data_length, data.len());

    let format = AudioFormat {
        channels: Some(format.channels),
        sample_rate: Some(format.sample_rate),
        bits_per_sample: Some(format.bits_per_sample)
    };

    println!("Got format");

    system.update_buffer(buffer, &data, &format);
    println!("Updated buffer");
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