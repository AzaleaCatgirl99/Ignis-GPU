mod instance;
pub use instance::*;
use vulkanalia::vk;

pub struct Adapter {
    pub(crate) inner: vk::PhysicalDevice,
}
