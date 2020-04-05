use fmod_sys::*;
use thiserror::Error;

// TODO Copy/paste error descriptions in from documentation file:///Volumes/FMOD%20Programmers%20API%20Mac/FMOD%20Programmers%20API/doc/FMOD%20API%20User%20Manual/core-api-common.html#fmod_result
#[derive(Error, Debug)]
pub enum FMODStudioError {
    #[error("")]
    BadCommand,
    #[error("")]
    ChannelAlloc,
    #[error("")]
    ChannelStolen,
    #[error("")]
    DMA,
    #[error("")]
    DspConnection,
    #[error("")]
    DspDontProcess,
    #[error("")]
    DspFormat,
    #[error("")]
    DspInUse,
    #[error("")]
    DspNotFound,
    #[error("")]
    DspReserved,
    #[error("")]
    DspSilence,
    #[error("")]
    DspType,
    #[error("")]
    FileBad,
    #[error("")]
    FileCouldNotSeek,
    #[error("")]
    FileDiskEjected,
    #[error("")]
    FileEof,
    #[error("")]
    FileEndOfData,
    #[error("")]
    FileNotFound,
    #[error("")]
    Format,
    #[error("")]
    HeaderMismatch,
    #[error("")]
    Http,
    #[error("")]
    HttpAccess,
    #[error("")]
    HttpProxyAuth,
    #[error("")]
    HttpServerError,
    #[error("")]
    HttpTimeout,
    #[error("")]
    Initialization,
    #[error("")]
    Initialized,
    #[error("")]
    Internal,
    #[error("")]
    InvalidFloat,
    #[error("")]
    InvalidHandle,
    #[error("")]
    InvalidParam,
    #[error("")]
    InvalidPosition,
    #[error("")]
    InvalidSpeaker,
    #[error("")]
    InvalidSyncPoint,
    #[error("")]
    InvalidThread,
    #[error("")]
    InvalidVector,
    #[error("")]
    MaxAudible,
    #[error("")]
    Memory,
    #[error("")]
    MemoryCantPoint,
    #[error("")]
    Needs3d,
    #[error("")]
    NeedsHardware,
    #[error("")]
    NetConnect,
    #[error("")]
    NetSocketError,
    #[error("")]
    NetUrl,
    #[error("")]
    NetWouldBlock,
    #[error("")]
    NotReady,
    #[error("")]
    OutputAllocated,
    #[error("")]
    OutputCreateBuffer,
    #[error("")]
    OutputDriverCall,
    #[error("")]
    OutputFormat,
    #[error("")]
    OutputInit,
    #[error("")]
    OutputNoDrivers,
    #[error("")]
    Plugin,
    #[error("")]
    PluginMissing,
    #[error("")]
    PluginResource,
    #[error("")]
    PluginVersion,
    #[error("")]
    Record,
    #[error("")]
    ReverbChannelGroup,
    #[error("")]
    ReverbInstance,
    #[error("")]
    SubSounds,
    #[error("")]
    SubsoundAllocated,
    #[error("")]
    SubsoundCantMove,
    #[error("")]
    TagNotFound,
    #[error("")]
    TooManyChannels,
    #[error("")]
    Truncated,
    #[error("")]
    Unimplemented,
    #[error("")]
    Unintialized,
    #[error("")]
    Unsupported,
    #[error("")]
    Version,
    #[error("")]
    EventAlreadyLoaded,
    #[error("")]
    EventLiveUpdateBusy,
    #[error("")]
    EventLiveUpdateMismatch,
    #[error("")]
    EventLiveUpdateTimeout,
    #[error("")]
    EventNotFound,
    #[error("")]
    StudioUninitialized,
    #[error("")]
    StudioNotLoaded,
    #[error("")]
    InvalidString,
    #[error("")]
    AlreadyLocked,
    #[error("")]
    NotLocked,
    #[error("")]
    RecordDisconnected,
    #[error("")]
    TooManySamples,
    #[error("FMOD Error code: {0}")]
    Unknown(FMOD_RESULT),
}

impl From<FMOD_RESULT> for FMODStudioError {
    fn from(result: FMOD_RESULT) -> Self {
        match result {
            FMOD_RESULT_FMOD_ERR_BADCOMMAND => FMODStudioError::BadCommand,
            FMOD_RESULT_FMOD_ERR_CHANNEL_ALLOC => FMODStudioError::ChannelAlloc,
            FMOD_RESULT_FMOD_ERR_CHANNEL_STOLEN => FMODStudioError::ChannelStolen,
            FMOD_RESULT_FMOD_ERR_DMA => FMODStudioError::DMA,
            FMOD_RESULT_FMOD_ERR_DSP_CONNECTION => FMODStudioError::DspConnection,
            FMOD_RESULT_FMOD_ERR_DSP_DONTPROCESS => FMODStudioError::DspDontProcess,
            FMOD_RESULT_FMOD_ERR_DSP_FORMAT => FMODStudioError::DspFormat,
            FMOD_RESULT_FMOD_ERR_DSP_INUSE => FMODStudioError::DspInUse,
            FMOD_RESULT_FMOD_ERR_DSP_NOTFOUND => FMODStudioError::DspNotFound,
            FMOD_RESULT_FMOD_ERR_DSP_RESERVED => FMODStudioError::DspReserved,
            FMOD_RESULT_FMOD_ERR_DSP_SILENCE => FMODStudioError::DspSilence,
            FMOD_RESULT_FMOD_ERR_DSP_TYPE => FMODStudioError::DspType,
            FMOD_RESULT_FMOD_ERR_FILE_BAD => FMODStudioError::FileBad,
            FMOD_RESULT_FMOD_ERR_FILE_COULDNOTSEEK => FMODStudioError::FileCouldNotSeek,
            FMOD_RESULT_FMOD_ERR_FILE_DISKEJECTED => FMODStudioError::FileDiskEjected,
            FMOD_RESULT_FMOD_ERR_FILE_EOF => FMODStudioError::FileEof,
            FMOD_RESULT_FMOD_ERR_FILE_ENDOFDATA => FMODStudioError::FileEndOfData,
            FMOD_RESULT_FMOD_ERR_FILE_NOTFOUND => FMODStudioError::FileNotFound,
            FMOD_RESULT_FMOD_ERR_FORMAT => FMODStudioError::Format,
            FMOD_RESULT_FMOD_ERR_HEADER_MISMATCH => FMODStudioError::HeaderMismatch,
            FMOD_RESULT_FMOD_ERR_HTTP => FMODStudioError::Http,
            FMOD_RESULT_FMOD_ERR_HTTP_ACCESS => FMODStudioError::HttpAccess,
            FMOD_RESULT_FMOD_ERR_HTTP_PROXY_AUTH => FMODStudioError::HttpProxyAuth,
            FMOD_RESULT_FMOD_ERR_HTTP_SERVER_ERROR => FMODStudioError::HttpServerError,
            FMOD_RESULT_FMOD_ERR_HTTP_TIMEOUT => FMODStudioError::HttpTimeout,
            FMOD_RESULT_FMOD_ERR_INITIALIZATION => FMODStudioError::Initialization,
            FMOD_RESULT_FMOD_ERR_INITIALIZED => FMODStudioError::Initialized,
            FMOD_RESULT_FMOD_ERR_INTERNAL => FMODStudioError::Internal,
            FMOD_RESULT_FMOD_ERR_INVALID_FLOAT => FMODStudioError::InvalidFloat,
            FMOD_RESULT_FMOD_ERR_INVALID_HANDLE => FMODStudioError::InvalidHandle,
            FMOD_RESULT_FMOD_ERR_INVALID_PARAM => FMODStudioError::InvalidParam,
            FMOD_RESULT_FMOD_ERR_INVALID_POSITION => FMODStudioError::InvalidPosition,
            FMOD_RESULT_FMOD_ERR_INVALID_SPEAKER => FMODStudioError::InvalidSpeaker,
            FMOD_RESULT_FMOD_ERR_INVALID_SYNCPOINT => FMODStudioError::InvalidSyncPoint,
            FMOD_RESULT_FMOD_ERR_INVALID_THREAD => FMODStudioError::InvalidThread,
            FMOD_RESULT_FMOD_ERR_INVALID_VECTOR => FMODStudioError::InvalidVector,
            FMOD_RESULT_FMOD_ERR_MAXAUDIBLE => FMODStudioError::MaxAudible,
            FMOD_RESULT_FMOD_ERR_MEMORY => FMODStudioError::Memory,
            FMOD_RESULT_FMOD_ERR_MEMORY_CANTPOINT => FMODStudioError::MemoryCantPoint,
            FMOD_RESULT_FMOD_ERR_NEEDS3D => FMODStudioError::Needs3d,
            FMOD_RESULT_FMOD_ERR_NEEDSHARDWARE => FMODStudioError::NeedsHardware,
            FMOD_RESULT_FMOD_ERR_NET_CONNECT => FMODStudioError::NetConnect,
            FMOD_RESULT_FMOD_ERR_NET_SOCKET_ERROR => FMODStudioError::NetSocketError,
            FMOD_RESULT_FMOD_ERR_NET_URL => FMODStudioError::NetUrl,
            FMOD_RESULT_FMOD_ERR_NET_WOULD_BLOCK => FMODStudioError::NetWouldBlock,
            FMOD_RESULT_FMOD_ERR_NOTREADY => FMODStudioError::NotReady,
            FMOD_RESULT_FMOD_ERR_OUTPUT_ALLOCATED => FMODStudioError::OutputAllocated,
            FMOD_RESULT_FMOD_ERR_OUTPUT_CREATEBUFFER => FMODStudioError::OutputCreateBuffer,
            FMOD_RESULT_FMOD_ERR_OUTPUT_DRIVERCALL => FMODStudioError::OutputDriverCall,
            FMOD_RESULT_FMOD_ERR_OUTPUT_FORMAT => FMODStudioError::OutputFormat,
            FMOD_RESULT_FMOD_ERR_OUTPUT_INIT => FMODStudioError::OutputInit,
            FMOD_RESULT_FMOD_ERR_OUTPUT_NODRIVERS => FMODStudioError::OutputNoDrivers,
            FMOD_RESULT_FMOD_ERR_PLUGIN => FMODStudioError::Plugin,
            FMOD_RESULT_FMOD_ERR_PLUGIN_MISSING => FMODStudioError::PluginMissing,
            FMOD_RESULT_FMOD_ERR_PLUGIN_RESOURCE => FMODStudioError::PluginResource,
            FMOD_RESULT_FMOD_ERR_PLUGIN_VERSION => FMODStudioError::PluginVersion,
            FMOD_RESULT_FMOD_ERR_RECORD => FMODStudioError::Record,
            FMOD_RESULT_FMOD_ERR_REVERB_CHANNELGROUP => FMODStudioError::ReverbChannelGroup,
            FMOD_RESULT_FMOD_ERR_REVERB_INSTANCE => FMODStudioError::ReverbInstance,
            FMOD_RESULT_FMOD_ERR_SUBSOUNDS => FMODStudioError::SubSounds,
            FMOD_RESULT_FMOD_ERR_SUBSOUND_ALLOCATED => FMODStudioError::SubsoundAllocated,
            FMOD_RESULT_FMOD_ERR_SUBSOUND_CANTMOVE => FMODStudioError::SubsoundCantMove,
            FMOD_RESULT_FMOD_ERR_TAGNOTFOUND => FMODStudioError::TagNotFound,
            FMOD_RESULT_FMOD_ERR_TOOMANYCHANNELS => FMODStudioError::TooManyChannels,
            FMOD_RESULT_FMOD_ERR_TRUNCATED => FMODStudioError::Truncated,
            FMOD_RESULT_FMOD_ERR_UNIMPLEMENTED => FMODStudioError::Unimplemented,
            FMOD_RESULT_FMOD_ERR_UNINITIALIZED => FMODStudioError::Unintialized,
            FMOD_RESULT_FMOD_ERR_UNSUPPORTED => FMODStudioError::Unsupported,
            FMOD_RESULT_FMOD_ERR_VERSION => FMODStudioError::Version,
            FMOD_RESULT_FMOD_ERR_EVENT_ALREADY_LOADED => FMODStudioError::EventAlreadyLoaded,
            FMOD_RESULT_FMOD_ERR_EVENT_LIVEUPDATE_BUSY => FMODStudioError::EventLiveUpdateBusy,
            FMOD_RESULT_FMOD_ERR_EVENT_LIVEUPDATE_MISMATCH => {
                FMODStudioError::EventLiveUpdateMismatch
            }
            FMOD_RESULT_FMOD_ERR_EVENT_LIVEUPDATE_TIMEOUT => {
                FMODStudioError::EventLiveUpdateTimeout
            }
            FMOD_RESULT_FMOD_ERR_EVENT_NOTFOUND => FMODStudioError::EventNotFound,
            FMOD_RESULT_FMOD_ERR_STUDIO_UNINITIALIZED => FMODStudioError::StudioUninitialized,
            FMOD_RESULT_FMOD_ERR_STUDIO_NOT_LOADED => FMODStudioError::StudioNotLoaded,
            FMOD_RESULT_FMOD_ERR_INVALID_STRING => FMODStudioError::InvalidString,
            FMOD_RESULT_FMOD_ERR_ALREADY_LOCKED => FMODStudioError::AlreadyLocked,
            FMOD_RESULT_FMOD_ERR_NOT_LOCKED => FMODStudioError::NotLocked,
            FMOD_RESULT_FMOD_ERR_RECORD_DISCONNECTED => FMODStudioError::RecordDisconnected,
            FMOD_RESULT_FMOD_ERR_TOOMANYSAMPLES => FMODStudioError::TooManySamples,
            _ => FMODStudioError::Unknown(result),
        }
    }
}
