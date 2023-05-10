use objc2::rc::{Id, Shared};
use objc2::ClassType;

use crate::sealed::UnsafeGetId;
use crate::sys::foundation::{NSString, NSURL};
use crate::sys::virtualization::{
    VZDiskImageStorageDeviceAttachment, VZStorageDeviceAttachment, VZStorageDeviceConfiguration,
    VZVirtioBlockDeviceConfiguration,
};

pub trait StorageDeviceAttachment: UnsafeGetId<VZStorageDeviceAttachment> {}

#[derive(Debug)]
pub struct DiskImageStorageDeviceAttachment {
    inner: Id<VZDiskImageStorageDeviceAttachment, Shared>,
}

impl DiskImageStorageDeviceAttachment {
    pub fn new(url: &str, read_only: bool) -> Self {
        unsafe {
            let disk_image_url = NSURL::file_url_with_path(url, false)
                .absolute_url()
                .unwrap();
            DiskImageStorageDeviceAttachment {
                inner: VZDiskImageStorageDeviceAttachment::initWithURL_readOnly_error(
                    VZDiskImageStorageDeviceAttachment::alloc(),
                    &disk_image_url,
                    read_only,
                )
                .unwrap(),
            }
        }
    }
}

impl StorageDeviceAttachment for DiskImageStorageDeviceAttachment {}

impl UnsafeGetId<VZStorageDeviceAttachment> for DiskImageStorageDeviceAttachment {
    fn id(&self) -> Id<VZStorageDeviceAttachment, Shared> {
        unsafe { Id::cast(self.inner.clone()) }
    }
}

pub trait StorageDeviceConfiguration: UnsafeGetId<VZStorageDeviceConfiguration> {}

#[derive(Debug)]
pub struct VirtioBlockDeviceConfiguration {
    inner: Id<VZVirtioBlockDeviceConfiguration, Shared>,
}

impl VirtioBlockDeviceConfiguration {
    pub fn new<T: StorageDeviceAttachment>(attachment: T) -> Self {
        unsafe {
            let attachment = attachment.id();
            let inner = VZVirtioBlockDeviceConfiguration::initWithAttachment(
                VZVirtioBlockDeviceConfiguration::alloc(),
                &attachment,
            );

            VirtioBlockDeviceConfiguration { inner }
        }
    }

    pub fn validate_block_device_identifier(block_device_identifier: &str) -> Result<(), String> {
        unsafe {
            let id = NSString::from_str(block_device_identifier);
            let result = VZVirtioBlockDeviceConfiguration::validateBlockDeviceIdentifier_error(&id);

            match result {
                Ok(_) => Ok(()),
                Err(ns_error) => Err(ns_error.localized_description()),
            }
        }
    }

    pub fn set_block_device_identifier(&self, block_device_identifier: &str) {
        unsafe {
            let id = NSString::from_str(block_device_identifier);
            self.inner.setBlockDeviceIdentifier(&id);
        }
    }
}

impl StorageDeviceConfiguration for VirtioBlockDeviceConfiguration {}

impl UnsafeGetId<VZStorageDeviceConfiguration> for VirtioBlockDeviceConfiguration {
    fn id(&self) -> Id<VZStorageDeviceConfiguration, Shared> {
        unsafe { Id::cast(self.inner.clone()) }
    }
}
