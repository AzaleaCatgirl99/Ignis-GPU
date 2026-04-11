pub enum Backend {
    Vulkan,
    Metal,
}

bitflags::bitflags! {
    #[derive(PartialEq, Clone, Copy)]
    pub struct InstanceFeatures: u8 {
        const DEBUG = 1 << 0;
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum AdapterType {
    Dedicated,
    Integrated,
    Virtual,
    Cpu,
    Unknown,
}

bitflags::bitflags! {
    #[derive(PartialEq, Clone, Copy)]
    pub struct AdapterFeatures: u8 {
        const MULTI_DRAW_INDIRECT = 1 << 0;
        const DRAW_INDIRECT_COUNT = 1 << 1;
    }
}
