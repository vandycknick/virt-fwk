use objc2::rc::{Id, Shared};
use objc2::ClassType;

use crate::sealed::UnsafeGetId;
use crate::sys::virtualization::{
    VZMemoryBalloonDeviceConfiguration, VZVirtioTraditionalMemoryBalloonDeviceConfiguration,
};

pub trait MemoryBalloonDeviceConfiguration:
    UnsafeGetId<VZMemoryBalloonDeviceConfiguration>
{
}

#[derive(Debug)]
pub struct VirtioTraditionalMemoryBalloonDeviceConfiguration {
    inner: Id<VZVirtioTraditionalMemoryBalloonDeviceConfiguration, Shared>,
}

impl VirtioTraditionalMemoryBalloonDeviceConfiguration {
    pub fn new() -> Self {
        unsafe {
            VirtioTraditionalMemoryBalloonDeviceConfiguration {
                inner: VZVirtioTraditionalMemoryBalloonDeviceConfiguration::init(
                    VZVirtioTraditionalMemoryBalloonDeviceConfiguration::alloc(),
                ),
            }
        }
    }
}

impl MemoryBalloonDeviceConfiguration for VirtioTraditionalMemoryBalloonDeviceConfiguration {}

impl Default for VirtioTraditionalMemoryBalloonDeviceConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl UnsafeGetId<VZMemoryBalloonDeviceConfiguration>
    for VirtioTraditionalMemoryBalloonDeviceConfiguration
{
    fn id(&self) -> Id<VZMemoryBalloonDeviceConfiguration, Shared> {
        unsafe { Id::cast(self.inner.clone()) }
    }
}
