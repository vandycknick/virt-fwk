use objc2::rc::{Id, Shared};
use objc2::ClassType;

use crate::sealed::UnsafeGetId;
use crate::sys::foundation::{NSString, NSURL};
use crate::sys::virtualization::{VZBootLoader, VZLinuxBootLoader};

pub trait BootLoader: UnsafeGetId<VZBootLoader> {}

pub struct LinuxBootLoader {
    inner: Id<VZLinuxBootLoader, Shared>,
}

impl LinuxBootLoader {
    pub fn new(kernel_url: &str, initrd_url: &str, command_line: &str) -> Self {
        let boot_loader = Self::new_with_kernel(kernel_url);
        boot_loader.set_initrd(initrd_url);
        boot_loader.set_command_line(command_line);
        boot_loader
    }

    pub fn new_with_kernel(kernel_url: &str) -> Self {
        unsafe {
            let kernel_url = NSURL::file_url_with_path(kernel_url, false)
                .absolute_url()
                .unwrap();

            LinuxBootLoader {
                inner: VZLinuxBootLoader::initWithKernelURL(
                    VZLinuxBootLoader::alloc(),
                    &kernel_url,
                ),
            }
        }
    }

    pub fn set_initrd(&self, initrd_url: &str) {
        unsafe {
            let initrd_url = NSURL::file_url_with_path(initrd_url, false)
                .absolute_url()
                .unwrap();

            self.inner.setInitialRamdiskURL(Some(&initrd_url));
        }
    }

    pub fn set_command_line(&self, command_line: &str) {
        unsafe {
            let command_line = NSString::from_str(command_line);
            self.inner.setCommandLine(&command_line);
        }
    }
}

impl BootLoader for LinuxBootLoader {}

impl UnsafeGetId<VZBootLoader> for LinuxBootLoader {
    fn id(&self) -> Id<VZBootLoader, Shared> {
        unsafe { Id::cast(self.inner.clone()) }
    }
}
