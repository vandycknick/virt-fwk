use objc2::rc::{Id, Shared};
use objc2::ClassType;

use crate::sealed::UnsafeGetId;
use crate::sys::virtualization::{
    VZMACAddress, VZNATNetworkDeviceAttachment, VZNetworkDeviceAttachment,
    VZNetworkDeviceConfiguration, VZVirtioNetworkDeviceConfiguration,
};

pub trait NetworkDeviceAttachment: UnsafeGetId<VZNetworkDeviceAttachment> {}

#[derive(Debug)]
pub struct NATNetworkDeviceAttachment {
    inner: Id<VZNATNetworkDeviceAttachment, Shared>,
}

impl NATNetworkDeviceAttachment {
    pub fn new() -> Self {
        unsafe {
            NATNetworkDeviceAttachment {
                inner: VZNATNetworkDeviceAttachment::init(VZNATNetworkDeviceAttachment::alloc()),
            }
        }
    }
}

impl NetworkDeviceAttachment for NATNetworkDeviceAttachment {}

impl Default for NATNetworkDeviceAttachment {
    fn default() -> Self {
        Self::new()
    }
}

impl UnsafeGetId<VZNetworkDeviceAttachment> for NATNetworkDeviceAttachment {
    fn id(&self) -> Id<VZNetworkDeviceAttachment, Shared> {
        unsafe { Id::cast(self.inner.clone()) }
    }
}

#[derive(Debug)]
pub struct MACAddress {
    inner: Id<VZMACAddress, Shared>,
}

impl MACAddress {
    pub fn new() -> Self {
        unsafe {
            MACAddress {
                inner: VZMACAddress::init(VZMACAddress::alloc()),
            }
        }
    }

    pub fn new_with_random_locally_administered_address() -> Self {
        unsafe {
            MACAddress {
                inner: VZMACAddress::randomLocallyAdministeredAddress(),
            }
        }
    }
}

impl Default for MACAddress {
    fn default() -> Self {
        Self::new()
    }
}

impl UnsafeGetId<VZMACAddress> for MACAddress {
    fn id(&self) -> Id<VZMACAddress, Shared> {
        unsafe { Id::cast(self.inner.clone()) }
    }
}

pub trait NetworkDeviceConfiguration: UnsafeGetId<VZNetworkDeviceConfiguration> {}

#[derive(Debug)]
pub struct VirtioNetworkDeviceConfiguration {
    inner: Id<VZVirtioNetworkDeviceConfiguration, Shared>,
}

impl VirtioNetworkDeviceConfiguration {
    pub fn new() -> Self {
        unsafe {
            VirtioNetworkDeviceConfiguration {
                inner: VZVirtioNetworkDeviceConfiguration::init(
                    VZVirtioNetworkDeviceConfiguration::alloc(),
                ),
            }
        }
    }

    pub fn new_with_attachment<T: NetworkDeviceAttachment>(attachment: T) -> Self {
        let config = Self::new();
        config.set_attachment(attachment);
        config
    }

    pub fn set_attachment<T: NetworkDeviceAttachment>(&self, attachment: T) {
        unsafe {
            let id = attachment.id();
            self.inner.setAttachment(Some(&id));
        }
    }

    pub fn set_mac_address(&self, addresss: MACAddress) {
        unsafe {
            let id = addresss.id();
            self.inner.setMACAddress(&id);
        }
    }
}

impl NetworkDeviceConfiguration for VirtioNetworkDeviceConfiguration {}

impl Default for VirtioNetworkDeviceConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl UnsafeGetId<VZNetworkDeviceConfiguration> for VirtioNetworkDeviceConfiguration {
    fn id(&self) -> Id<VZNetworkDeviceConfiguration, Shared> {
        unsafe { Id::cast(self.inner.clone()) }
    }
}
