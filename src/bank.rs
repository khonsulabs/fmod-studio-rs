use super::{event_description::EventDescription, internal_prelude::*};
use fmod_sys::*;
use std::ptr;

pub struct Bank {
    pub bank: *mut FMOD_STUDIO_BANK,
}

pub enum LoadingState {
    Unloading,
    Unloaded,
    Loading,
    Loaded,
    Error,
}

impl From<FMOD_STUDIO_LOADING_STATE> for LoadingState {
    fn from(state: FMOD_STUDIO_LOADING_STATE) -> Self {
        match state {
            FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_UNLOADING => {
                LoadingState::Unloading
            }
            FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_UNLOADED => LoadingState::Unloaded,
            FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_LOADED => LoadingState::Loading,
            FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_LOADING => LoadingState::Loaded,
            FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_ERROR => LoadingState::Error,
            _ => panic!("Unknown state"),
        }
    }
}

impl Bank {
    pub fn get_sample_loading_state(&self) -> FMODStudioResult<LoadingState> {
        let mut state: FMOD_STUDIO_LOADING_STATE = 0;
        let result = unsafe { FMOD_Studio_Bank_GetSampleLoadingState(self.bank, &mut state) };
        result.as_result_with(|| LoadingState::from(state))
    }

    pub fn load_all_sample_data(&self) -> FMODStudioResult<()> {
        unsafe { FMOD_Studio_Bank_LoadSampleData(self.bank) }.as_result()
    }

    pub fn events(&self) -> FMODStudioResult<Vec<EventDescription>> {
        let mut count = 0;
        unsafe { FMOD_Studio_Bank_GetEventCount(self.bank, &mut count) }.as_result()?;
        let mut descriptions =
            Vec::<*mut FMOD_STUDIO_EVENTDESCRIPTION>::with_capacity(count as usize);
        unsafe {
            descriptions.set_len(count as usize);
            FMOD_Studio_Bank_GetEventList(
                self.bank,
                descriptions.as_ptr() as *mut *mut FMOD_STUDIO_EVENTDESCRIPTION,
                count,
                ptr::null_mut::<i32>(),
            )
        }
        .as_result()?;

        Ok(descriptions
            .into_iter()
            .map(|description| EventDescription { description })
            .collect())
    }
}
