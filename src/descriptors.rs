use crate::{AdapterType, Backend, IgnisHasWindowHandle, InstanceFeatures, Version};

pub struct ApplicationDescriptor {
    pub name: &'static str,
    pub version: Version,
    pub engine_name: &'static str,
    pub engine_version: Version,
}

pub struct InstanceDescriptor {
    pub application: ApplicationDescriptor,
    pub features: InstanceFeatures,
    pub window: Option<Box<dyn IgnisHasWindowHandle>>,
}

pub struct BackendInfo {
    pub name: &'static str,
    pub version: Version,
}

// TODO: implement device limits...
pub struct AdapterInfo {
    pub device_name: String,
    pub device_type: AdapterType,
    pub device_id: u32,
    pub vendor_id: u32,
}
