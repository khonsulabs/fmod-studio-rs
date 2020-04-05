use super::internal_prelude::*;
use fmod_sys::*;

pub struct EventInstance {
    pub instance: *mut FMOD_STUDIO_EVENTINSTANCE,
}

impl EventInstance {
    pub fn start(&self) -> FMODStudioResult<()> {
        unsafe { FMOD_Studio_EventInstance_Start(self.instance) }.as_result()
    }
}
