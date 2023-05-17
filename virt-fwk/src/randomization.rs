use objc2::rc::{Id, Shared};

use crate::sealed::UnsafeGetId;
use crate::sys::virtualization::{
    VZEntropyDeviceConfiguration, VZVirtioEntropyDeviceConfiguration,
};

pub trait EntropyDeviceConfiguration: UnsafeGetId<VZEntropyDeviceConfiguration> {}

#[derive(Debug)]
pub struct VirtioEntropyDeviceConfiguration {
    inner: Id<VZVirtioEntropyDeviceConfiguration, Shared>,
}

impl VirtioEntropyDeviceConfiguration {
    pub fn new() -> Self {
        unsafe {
            VirtioEntropyDeviceConfiguration {
                inner: VZVirtioEntropyDeviceConfiguration::new(),
            }
        }
    }
}

impl EntropyDeviceConfiguration for VirtioEntropyDeviceConfiguration {}

impl Default for VirtioEntropyDeviceConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl UnsafeGetId<VZEntropyDeviceConfiguration> for VirtioEntropyDeviceConfiguration {
    fn id(&self) -> Id<VZEntropyDeviceConfiguration, Shared> {
        unsafe { Id::cast(self.inner.clone()) }
    }
}
