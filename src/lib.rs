pub extern crate fmod_sys;
pub mod bank;
pub mod error;
pub mod event_description;
pub mod event_instance;
pub mod system;
use error::FMODStudioError;
use fmod_sys::*;

pub type FMODStudioResult<T> = Result<T, FMODStudioError>;

pub trait AsResult {
    fn as_result(&self) -> FMODStudioResult<()>;
}

pub trait AsResultWith<T> {
    fn as_result_with<F: FnOnce() -> T>(&self, if_success: F) -> FMODStudioResult<T>;
}

impl AsResult for FMOD_RESULT {
    fn as_result(&self) -> FMODStudioResult<()> {
        self.as_result_with(|| ())
    }
}

impl<T> AsResultWith<T> for FMOD_RESULT {
    fn as_result_with<F: FnOnce() -> T>(&self, if_success: F) -> FMODStudioResult<T> {
        match *self {
            FMOD_RESULT_FMOD_OK => Ok(if_success()),
            _ => Err(FMODStudioError::from(*self)),
        }
    }
}

mod internal_prelude {
    pub use super::{error::FMODStudioError, AsResult, AsResultWith, FMODStudioResult};
}
