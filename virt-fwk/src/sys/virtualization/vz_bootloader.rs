#![allow(non_snake_case)]
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType};

use crate::sys::foundation::*;
use crate::sys::virtualization::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZBootLoader;

    unsafe impl ClassType for VZBootLoader {
        type Super = NSObject;
    }
);

#[cfg(feature = "not_implemented_yet")]
unsafe impl NSCopying for VZBootLoader {}

unsafe impl NSObjectProtocol for VZBootLoader {}

extern_methods!(
    unsafe impl VZBootLoader {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct VZLinuxBootLoader;

    unsafe impl ClassType for VZLinuxBootLoader {
        #[inherits(NSObject)]
        type Super = VZBootLoader;
    }
);

#[cfg(feature = "not_implemented_yet")]
unsafe impl NSCopying for VZLinuxBootLoader {}

unsafe impl NSObjectProtocol for VZLinuxBootLoader {}

extern_methods!(
    unsafe impl VZLinuxBootLoader {
        #[method_id(@__retain_semantics Init initWithKernelURL:)]
        pub(crate) unsafe fn initWithKernelURL(
            this: Option<Allocated<Self>>,
            kernel_url: &NSURL,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other kernelURL)]
        pub(crate) unsafe fn kernelURL(&self) -> Id<NSURL, Shared>;

        #[method(setKernelURL:)]
        pub(crate) unsafe fn setKernelURL(&self, kernel_url: &NSURL);

        #[method_id(@__retain_semantics Other commandLine)]
        pub(crate) unsafe fn commandLine(&self) -> Id<NSString, Shared>;

        #[method(setCommandLine:)]
        pub(crate) unsafe fn setCommandLine(&self, command_line: &NSString);

        #[method_id(@__retain_semantics Other initialRamdiskURL)]
        pub(crate) unsafe fn initialRamdiskURL(&self) -> Option<Id<NSURL, Shared>>;

        #[method(setInitialRamdiskURL:)]
        pub(crate) unsafe fn setInitialRamdiskURL(&self, initial_ramdisk_url: Option<&NSURL>);
    }
);
