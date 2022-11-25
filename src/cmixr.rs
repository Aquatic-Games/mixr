use crate::{system::{AudioSystem}, AudioFormat, ChannelProperties};

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
pub extern "C" fn mxSetBufferFinishedCallback(system: &mut AudioSystem, callback: extern "C" fn(u16, i32)) {
    system.set_buffer_finished_callback(unsafe { std::mem::transmute(callback) });
}

#[no_mangle]
pub extern "C" fn mxCreateBuffer(system: &mut AudioSystem) -> i32 {
    system.create_buffer()
}
    

#[no_mangle]
pub extern "C" fn mxUpdateBuffer(system: &mut AudioSystem, buffer: i32, data: *const u8, data_length: usize, format: AudioFormat) {
    let data = unsafe { std::slice::from_raw_parts(data, data_length) };

    assert_eq!(data_length, data.len(), "data_length, {}, does not equal the converted data length, {}", data_length, data.len());

    system.update_buffer(buffer, &data, format);
}

#[no_mangle]
pub extern "C" fn mxPlayBuffer(system: &mut AudioSystem, buffer: i32, channel: u16, properties: ChannelProperties) {
    system.play_buffer(buffer, channel, properties);
}

#[no_mangle]
pub extern "C" fn mxQueueBuffer(system: &mut AudioSystem, buffer: i32, channel: u16) {
    system.queue_buffer(buffer, channel);
}

#[no_mangle]
pub extern "C" fn mxSetChannelProperties(system: &mut AudioSystem, channel: u16, properties: ChannelProperties) {
    system.set_channel_properties(channel, properties);
}

#[no_mangle]
pub extern "C" fn mxPlay(system: &mut AudioSystem, channel: u16) {
    system.play(channel);
}

#[no_mangle]
pub extern "C" fn mxPause(system: &mut AudioSystem, channel: u16) {
    system.pause(channel);
}

#[no_mangle]
pub extern "C" fn mxStop(system: &mut AudioSystem, channel: u16) {
    system.stop(channel);
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