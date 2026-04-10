#[cfg(feature = "debug")]
use std::ffi::{CStr, c_void};

use vulkanalia::{
    loader::{LIBRARY, LibloadingLoader},
    vk::{self, ErrorCode, ExtDebugUtilsExtensionInstanceCommands, HasBuilder, InstanceV1_0},
};

use crate::{Backend, InstanceCreateError, InstanceDescriptor, InstanceFeatures};

#[cfg(feature = "debug")]
// Internal debug messenger callback.
extern "system" fn debug_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    type_: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    let data = unsafe { *data };
    let message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();

    println!("({:?}) {}", type_, message);

    // Only crash if the severity is 'ERROR'.
    (severity >= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR) as vk::Bool32
}

pub(crate) trait Instance {
    fn destroy(&self);

    fn get_backend(&self) -> Backend;
    fn get_backend_str(&self) -> &'static str;
}

pub(crate) struct VulkanInstance {
    _entry: vulkanalia::Entry,
    handle: vulkanalia::Instance,
    debug_messenger: Option<vk::DebugUtilsMessengerEXT>,
}

impl Instance for VulkanInstance {
    fn destroy(&self) {
        if self.debug_messenger.is_some() {
            unsafe {
                self.handle
                    .destroy_debug_utils_messenger_ext(self.debug_messenger.unwrap(), None)
            };
        }

        unsafe { self.handle.destroy_instance(None) };
    }

    #[inline]
    fn get_backend(&self) -> Backend {
        Backend::VULKAN
    }

    #[inline]
    fn get_backend_str(&self) -> &'static str {
        "Vulkan"
    }
}

impl VulkanInstance {
    #[cfg(feature = "debug")]
    fn create_debug_messenger(
        handle: &vulkanalia::Instance,
    ) -> Result<vk::DebugUtilsMessengerEXT, InstanceCreateError> {
        let severity = vk::DebugUtilsMessageSeverityFlagsEXT::all();
        let type_flags = vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
            | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
            | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE;

        let create_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(severity)
            .message_type(type_flags)
            .user_callback(Some(debug_callback));

        match unsafe { handle.create_debug_utils_messenger_ext(&create_info, None) } {
            Ok(messenger) => Ok(messenger),
            Err(code) => {
                return Err(match code {
                    ErrorCode::OUT_OF_HOST_MEMORY => InstanceCreateError::OutOfHostMemory,
                    ErrorCode::VALIDATION_FAILED => InstanceCreateError::DebugFailed,
                    _ => InstanceCreateError::Unknown,
                });
            }
        }
    }

    pub fn new(descriptor: &InstanceDescriptor) -> Result<Self, InstanceCreateError> {
        let library_location = LIBRARY;

        // Load the library.
        let loader = match unsafe { LibloadingLoader::new(library_location) } {
            Ok(loader) => loader,
            Err(_) => {
                return Err(InstanceCreateError::UnableToLoadLibrary(
                    library_location.to_string(),
                ));
            }
        };

        // Create the entry.
        let entry = match unsafe { vulkanalia::Entry::new(loader) } {
            Ok(entry) => entry,
            Err(_) => {
                return Err(InstanceCreateError::UnableToLoadLibrary(
                    library_location.to_string(),
                ));
            }
        };

        // 1.3 is used because it is the standard for desktops.
        let vk_api_ver: u32 = vulkanalia::Version::V1_3_0.into();

        let app_name = descriptor.application.name.as_bytes();
        let app_ver = descriptor.application.version.into();
        let engine_name = descriptor.application.engine_name.as_bytes();
        let engine_ver = descriptor.application.engine_version.into();

        let app_info = vk::ApplicationInfo::builder()
            .application_name(app_name)
            .application_version(app_ver)
            .engine_name(engine_name)
            .engine_version(engine_ver)
            .api_version(vk_api_ver)
            .build();

        let mut layers = vec![];
        let mut extensions = vec![];

        // Add window extensions if there is a window available.
        if descriptor.window.is_some() {
            let window_handle = descriptor.window.as_ref().unwrap().window_handle().unwrap();

            extensions = vulkanalia::window::get_required_instance_extensions(&window_handle)
                .to_vec()
                .iter()
                .map(|e| e.as_ptr())
                .collect();
        }

        // Add internal validation if the backend debugger feature is enabled.
        if cfg!(feature = "debug") && descriptor.features.contains(InstanceFeatures::DEBUG) {
            layers.push(c"VK_LAYER_KHRONOS_validation".as_ptr());
            extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());
        }

        let flags = vk::InstanceCreateFlags::empty();

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_layer_names(&layers)
            .enabled_extension_names(&extensions)
            .flags(flags)
            .build();

        // Create the main handle.
        let handle = match unsafe { entry.create_instance(&create_info, None) } {
            Ok(handle) => handle,
            Err(code) => {
                return Err(match code {
                    ErrorCode::EXTENSION_NOT_PRESENT => {
                        InstanceCreateError::WindowRequirementsNotPresent
                    }
                    ErrorCode::INCOMPATIBLE_DRIVER => InstanceCreateError::IncompatibleDriver,
                    ErrorCode::INITIALIZATION_FAILED => InstanceCreateError::InitializationFailure,
                    ErrorCode::LAYER_NOT_PRESENT => {
                        InstanceCreateError::DebugRequirementsNotPresent
                    }
                    ErrorCode::OUT_OF_DEVICE_MEMORY => InstanceCreateError::OutOfDeviceMemory,
                    ErrorCode::OUT_OF_HOST_MEMORY => InstanceCreateError::OutOfHostMemory,
                    ErrorCode::VALIDATION_FAILED => InstanceCreateError::DebugFailed,
                    _ => InstanceCreateError::Unknown,
                });
            }
        };

        // Create the debug messenger if the features are enabled.
        let debug_messenger =
            if cfg!(feature = "debug") && descriptor.features.contains(InstanceFeatures::DEBUG) {
                Some(Self::create_debug_messenger(&handle)?)
            } else {
                None
            };

        Ok(Self {
            _entry: entry,
            handle,
            debug_messenger,
        })
    }
}
