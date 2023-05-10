use objc2::rc::{Id, Shared};
use objc2::ClassType;
use std::os::fd::AsRawFd;

use crate::sealed::UnsafeGetId;
use crate::sys::foundation::NSFileHandle;
use crate::sys::virtualization::{
    VZFileHandleSerialPortAttachment, VZSerialPortAttachment, VZSerialPortConfiguration,
    VZVirtioConsoleDeviceSerialPortConfiguration,
};

pub trait SerialPortAttachment: UnsafeGetId<VZSerialPortAttachment> {}

#[derive(Debug)]
pub struct FileHandleSerialPortAttachment {
    inner: Id<VZFileHandleSerialPortAttachment, Shared>,
}

impl FileHandleSerialPortAttachment {
    pub fn new<T: AsRawFd, U: AsRawFd>(reader: &T, writer: &U) -> Self {
        unsafe {
            let file_handle_for_reading =
                NSFileHandle::initWithFileDescriptor(NSFileHandle::alloc(), reader.as_raw_fd());

            let file_handle_for_writing =
                NSFileHandle::initWithFileDescriptor(NSFileHandle::alloc(), writer.as_raw_fd());

            let attachment =
                VZFileHandleSerialPortAttachment::initWithFileHandleForReading_fileHandleForWriting(
                    VZFileHandleSerialPortAttachment::alloc(),
                    Some(&file_handle_for_reading),
                    Some(&file_handle_for_writing),
                );
            FileHandleSerialPortAttachment { inner: attachment }
        }
    }
}

impl SerialPortAttachment for FileHandleSerialPortAttachment {}

impl UnsafeGetId<VZSerialPortAttachment> for FileHandleSerialPortAttachment {
    fn id(&self) -> Id<VZSerialPortAttachment, Shared> {
        unsafe { Id::cast(self.inner.clone()) }
    }
}

pub trait SerialPortConfiguration: UnsafeGetId<VZSerialPortConfiguration> {
    fn set_attachment<T: SerialPortAttachment>(&self, attachment: T);
}

#[derive(Debug)]
pub struct VirtioConsoleDeviceSerialPortConfiguration {
    inner: Id<VZVirtioConsoleDeviceSerialPortConfiguration, Shared>,
}

impl VirtioConsoleDeviceSerialPortConfiguration {
    pub fn new() -> Self {
        VirtioConsoleDeviceSerialPortConfiguration {
            inner: VZVirtioConsoleDeviceSerialPortConfiguration::new(),
        }
    }

    pub fn new_with_attachment<T: SerialPortAttachment>(attachment: T) -> Self {
        let config = Self::new();
        config.set_attachment(attachment);
        config
    }
}

impl SerialPortConfiguration for VirtioConsoleDeviceSerialPortConfiguration {
    fn set_attachment<T: SerialPortAttachment>(&self, attachment: T) {
        unsafe {
            let attachment_id = attachment.id();
            self.inner.setAttachment(Some(&attachment_id));
        }
    }
}

impl UnsafeGetId<VZSerialPortConfiguration> for VirtioConsoleDeviceSerialPortConfiguration {
    fn id(&self) -> Id<VZSerialPortConfiguration, Shared> {
        unsafe { Id::cast(self.inner.clone()) }
    }
}
