#![allow(non_snake_case)]
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType};

use crate::sys::virtualization::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZVirtioTraditionalMemoryBalloonDeviceConfiguration;

    unsafe impl ClassType for VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
        #[inherits(NSObject)]
        type Super = VZMemoryBalloonDeviceConfiguration;
    }
);

#[cfg(feature = "not_implemented_yet")]
unsafe impl NSCopying for VZVirtioTraditionalMemoryBalloonDeviceConfiguration {}

unsafe impl NSObjectProtocol for VZVirtioTraditionalMemoryBalloonDeviceConfiguration {}

extern_methods!(
    unsafe impl VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
