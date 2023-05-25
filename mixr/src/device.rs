use crate::AudioSystem;

pub struct AudioDevice<'a> {
    system: &'a mut AudioSystem
}