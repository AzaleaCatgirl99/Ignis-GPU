use bitflags::bitflags;

pub(crate) mod internal;

mod instance;
pub use instance::*;

#[inline]
const fn version_packed(major: u8, minor: u8, patch: u8) -> u32 {
    (major as u32) << 22u32 | (minor as u32) << 12u32 | (patch as u32)
}

#[inline]
const fn version_major(packed: u32) -> u8 {
    (packed >> 22u32) as u8
}

#[inline]
const fn version_minor(packed: u32) -> u8 {
    (packed >> 12u32) as u8
}

#[inline]
const fn version_patch(packed: u32) -> u8 {
    packed as u8
}

#[repr(packed(32))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl Version {
    #[inline]
    pub const fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl Default for Version {
    #[inline]
    fn default() -> Self {
        Self {
            major: 1,
            minor: 0,
            patch: 0,
        }
    }
}

impl From<u32> for Version {
    #[inline]
    fn from(packed: u32) -> Self {
        Self {
            major: version_major(packed),
            minor: version_minor(packed),
            patch: version_patch(packed),
        }
    }
}

impl From<Version> for u32 {
    #[inline]
    fn from(version: Version) -> Self {
        version_packed(version.major, version.minor, version.patch)
    }
}

impl From<(u8, u8, u8)> for Version {
    #[inline]
    fn from(nums: (u8, u8, u8)) -> Self {
        Self {
            major: nums.0,
            minor: nums.1,
            patch: nums.2,
        }
    }
}

impl From<Version> for (u8, u8, u8) {
    #[inline]
    fn from(version: Version) -> Self {
        (version.major, version.minor, version.patch)
    }
}

impl From<[u8; 3]> for Version {
    #[inline]
    fn from(array: [u8; 3]) -> Self {
        Self {
            major: array[0],
            minor: array[1],
            patch: array[2],
        }
    }
}

impl From<Version> for [u8; 3] {
    fn from(version: Version) -> Self {
        [version.major, version.minor, version.patch]
    }
}

pub const API_VERSION: Version = Version::new(0, 1, 0);

bitflags! {
    #[derive(PartialEq, Clone, Copy)]
    pub struct Backend: u8 {
        const VULKAN = 1 << 0;
        const METAL = 1 << 1;
    }
}

bitflags! {
    #[derive(PartialEq, Clone, Copy)]
    pub struct InstanceFeatures: u8 {
        const DEBUG = 1 << 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_test() -> anyhow::Result<()> {
        let app_name = "Ignis Main Test";
        let engine_name = "Ignis Main Test Engine";
        let version = Version::default();

        let application_descriptor = ApplicationDescriptor {
            name: app_name,
            version: version,
            engine_name: engine_name,
            engine_version: version,
        };

        let descriptor = InstanceDescriptor {
            backends: Backend::VULKAN,
            application: application_descriptor,
            features: InstanceFeatures::DEBUG,
            window: None,
        };

        let instance = Instance::new(&descriptor)?;

        println!("Backend: {}", instance.get_backend_str());

        instance.destroy();

        Ok(())
    }
}
