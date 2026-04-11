use thiserror::Error;

#[derive(Error, Debug)]
pub enum InstanceCreateError {
    #[error("current driver does not support one of the selected available backends")]
    UnsupportedBackend,
    #[error("could not load library at location '{0}'")]
    UnableToLoadLibrary(String),
    #[error("internal backend does not have support for current driver")]
    IncompatibleDriver,
    #[error("initialization of internal backend handle failed")]
    InitializationFailure,
    #[error("Vulkan extension is not present for driver")]
    WindowRequirementsNotPresent,
    #[error("device does not have enough memory to succeed")]
    OutOfDeviceMemory,
    #[error("host does not have enough memory to succeed")]
    OutOfHostMemory,
    #[error("missing backend debug validation requirements")]
    DebugRequirementsNotPresent,
    #[error("failed to initialize debugging")]
    DebugFailed,
    #[error("unknown error detected")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum AdapterRequestError {
    #[error("no available adapters")]
    NoneAvailable,
    #[error("initialization of internal backend handle failed")]
    InitializationFailure,
    #[error("device does not have enough memory to succeed")]
    OutOfDeviceMemory,
    #[error("host does not have enough memory to succeed")]
    OutOfHostMemory,
    #[error("failed to debug")]
    DebugFailed,
    #[error("unknown error detected")]
    Unknown,
}
