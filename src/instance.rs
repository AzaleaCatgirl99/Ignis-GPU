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

pub struct Instance {
    handle: Box<dyn internal::Instance>,
}

impl Instance {
    #[inline]
    pub fn new(descriptor: &InstanceDescriptor) -> Result<Self, InstanceCreateError> {
        let handle = if cfg!(target_os = "macos") && descriptor.backends.contains(Backend::METAL) {
            // TODO: implement actual Metal backend...
            Box::new(internal::vulkan::Instance::new(descriptor)?)
        } else if descriptor.backends.contains(Backend::VULKAN) {
            Box::new(internal::vulkan::Instance::new(descriptor)?)
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
