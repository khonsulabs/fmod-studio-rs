use super::{bank::Bank, internal_prelude::*};
use fmod_sys::*;
use std::{
    ffi::{c_void, CString},
    ptr,
    sync::{
        atomic::{AtomicBool, AtomicPtr, Ordering},
        Arc,
    },
    thread, time,
};

struct SystemHandle {
    system: AtomicPtr<FMOD_STUDIO_SYSTEM>,
    shutdown: AtomicBool,
}

impl Drop for SystemHandle {
    fn drop(&mut self) {
        let result = unsafe { FMOD_Studio_System_Release(self.system.load(Ordering::Relaxed)) };
        assert_eq!(result, FMOD_RESULT_FMOD_OK);
    }
}

#[derive(Default)]
pub struct CoreInitialization {
    pub righthanded_3d: bool,
    pub channel_lowpass: bool,
    pub channel_distancefilter: bool,
    pub profile_enable: bool,
    pub vol0_becomes_virtual: bool,
    pub geometry_use_closest: bool,
    pub prefer_dolby_downmix: bool,
    pub profile_meter_all: bool,
    pub memory_tracking: bool,
}

impl CoreInitialization {
    pub fn flags(&self) -> FMOD_INITFLAGS {
        let mut flags = FMOD_INIT_NORMAL;

        if self.righthanded_3d {
            flags |= FMOD_INIT_3D_RIGHTHANDED;
        }

        if self.channel_lowpass {
            flags |= FMOD_INIT_CHANNEL_LOWPASS;
        }

        if self.channel_distancefilter {
            flags |= FMOD_INIT_CHANNEL_DISTANCEFILTER;
        }

        if self.profile_enable {
            flags |= FMOD_INIT_PROFILE_ENABLE;
        }

        if self.vol0_becomes_virtual {
            flags |= FMOD_INIT_VOL0_BECOMES_VIRTUAL;
        }

        if self.geometry_use_closest {
            flags |= FMOD_INIT_GEOMETRY_USECLOSEST;
        }

        if self.prefer_dolby_downmix {
            flags |= FMOD_INIT_PREFER_DOLBY_DOWNMIX;
        }

        if self.profile_meter_all {
            flags |= FMOD_INIT_PROFILE_METER_ALL;
        }

        if self.memory_tracking {
            flags |= FMOD_INIT_MEMORY_TRACKING;
        }

        flags
    }
}

#[derive(Default)]
pub struct StudioInitialization {
    pub max_channels: Option<usize>,
    pub spawn_update_thread: bool,
    pub live_update: bool,
    pub allow_missing_plugins: bool,
    pub memory_tracking: bool,
    pub core: CoreInitialization,
}

impl StudioInitialization {
    pub fn flags(&self) -> (FMOD_STUDIO_INITFLAGS, FMOD_INITFLAGS) {
        let mut flags = FMOD_STUDIO_INIT_NORMAL;

        if self.live_update {
            flags |= FMOD_STUDIO_INIT_LIVEUPDATE;
        }

        if self.allow_missing_plugins {
            flags |= FMOD_STUDIO_INIT_ALLOW_MISSING_PLUGINS;
        }

        if self.memory_tracking {
            flags |= FMOD_STUDIO_INIT_MEMORY_TRACKING;
        }

        (flags, self.core.flags())
    }
}

#[derive(Clone)]
pub struct System {
    handle: Arc<SystemHandle>,
}

const F144HZ_AS_NANOS: u128 = 6_944_444;

impl System {
    pub fn new() -> FMODStudioResult<System> {
        Self::initialized(StudioInitialization::default())
    }

    pub fn initialized(initialization: StudioInitialization) -> FMODStudioResult<System> {
        let mut system: *mut FMOD_STUDIO_SYSTEM = ptr::null_mut();
        let result = unsafe { FMOD_Studio_System_Create(&mut system, FMOD_VERSION) };
        let handle = result.as_result_with(|| {
            Arc::new(SystemHandle {
                system: system.into(),
                shutdown: AtomicBool::new(false),
            })
        })?;

        let (studio_init_flags, core_init_flags) = initialization.flags();
        let result = unsafe {
            FMOD_Studio_System_Initialize(
                system,
                initialization.max_channels.unwrap_or(512) as i32,
                studio_init_flags,
                core_init_flags,
                ptr::null_mut() as *mut c_void,
            )
        };
        let system = result.as_result_with(|| System { handle })?;

        if initialization.spawn_update_thread {
            let thread_self = system.clone();
            thread::spawn(move || thread_self.update_thread_main());
        }

        Ok(system)
    }

    pub fn load_bank_from_file<S: Into<String>>(
        &self,
        path: S,
        decompress_samples: bool,
    ) -> FMODStudioResult<Bank> {
        let mut flags = FMOD_STUDIO_LOAD_BANK_NORMAL;
        if decompress_samples {
            flags |= FMOD_STUDIO_LOAD_BANK_DECOMPRESS_SAMPLES;
        }
        let path_cstr = CString::new(path.into().as_bytes()).unwrap();
        let mut bank: *mut FMOD_STUDIO_BANK = ptr::null_mut();
        let result = unsafe {
            FMOD_Studio_System_LoadBankFile(self.system(), path_cstr.as_ptr(), flags, &mut bank)
        };

        result.as_result_with(|| Bank { bank })
    }

    pub fn system(&self) -> *mut FMOD_STUDIO_SYSTEM {
        self.handle.system.load(Ordering::Relaxed)
    }

    pub fn update(&self) -> FMODStudioResult<()> {
        unsafe { FMOD_Studio_System_Update(self.system()) }.as_result()
    }

    pub fn shutdown(&self) {
        self.handle.shutdown.store(true, Ordering::Release);
    }

    fn update_thread_main(self) {
        let mut last_update_time = time::Instant::now();
        while !self.handle.shutdown.load(Ordering::Acquire) {
            if last_update_time.elapsed().as_nanos() > F144HZ_AS_NANOS {
                self.update().unwrap();
                last_update_time = time::Instant::now();
            }
        }
    }
}
