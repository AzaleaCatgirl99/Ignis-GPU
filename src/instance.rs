use thiserror::Error;

use crate::{Backend, InstanceFeatures, Version, internal};

pub trait IgnisHasWindowHandle:
    raw_window_handle::HasWindowHandle + core::fmt::Debug + Send + Sync + 'static
{
}
impl<T: raw_window_handle::HasWindowHandle + core::fmt::Debug + Send + Sync + 'static>
    IgnisHasWindowHandle for T
{
}

pub struct ApplicationDescriptor {
    pub name: &'static str,
    pub version: Version,
    pub engine_name: &'static str,
    pub engine_version: Version,
}

pub struct InstanceDescriptor {
    pub backends: Backend,
    pub application: ApplicationDescriptor,
    pub features: InstanceFeatures,
    pub window: Option<Box<dyn IgnisHasWindowHandle>>,
}

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

pub struct Instance {
    handle: Box<dyn internal::Instance>,
}

impl Instance {
    #[inline]
    pub fn new(descriptor: &InstanceDescriptor) -> Result<Self, InstanceCreateError> {
        let handle = if cfg!(target_os = "macos") && descriptor.backends.contains(Backend::METAL) {
            // TODO: implement actual Metal backend...
            Box::new(internal::VulkanInstance::new(descriptor)?)
        } else if descriptor.backends.contains(Backend::VULKAN) {
            Box::new(internal::VulkanInstance::new(descriptor)?)
        } else {
            return Err(InstanceCreateError::UnsupportedBackend);
        };

        Ok(Self { handle })
    }

    #[inline]
    pub fn destroy(&self) {
        self.handle.destroy();
    }

    #[inline]
    pub fn get_backend(&self) -> Backend {
        self.handle.get_backend()
    }

    #[inline]
    pub fn get_backend_str(&self) -> &'static str {
        self.handle.get_backend_str()
    }
}
