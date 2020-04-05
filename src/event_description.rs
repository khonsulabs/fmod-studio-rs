use super::{event_instance::EventInstance, internal_prelude::*};
use fmod_sys::*;
use std::ptr;

pub struct EventDescription {
    pub description: *mut FMOD_STUDIO_EVENTDESCRIPTION,
}

impl EventDescription {
    pub fn create_instance(&self) -> FMODStudioResult<EventInstance> {
        let mut instance: *mut FMOD_STUDIO_EVENTINSTANCE = ptr::null_mut();
        unsafe { FMOD_Studio_EventDescription_CreateInstance(self.description, &mut instance) }
            .as_result_with(|| EventInstance { instance })
    }
}
