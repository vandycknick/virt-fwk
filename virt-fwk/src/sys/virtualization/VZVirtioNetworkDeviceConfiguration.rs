#![allow(non_snake_case)]
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType};

use crate::sys::virtualization::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct VZVirtioNetworkDeviceConfiguration;

    unsafe impl ClassType for VZVirtioNetworkDeviceConfiguration {
        #[inherits(NSObject)]
        type Super = VZNetworkDeviceConfiguration;
    }
);

#[cfg(feature = "not_implemented_yet")]
unsafe impl NSCopying for VZVirtioNetworkDeviceConfiguration {}

unsafe impl NSObjectProtocol for VZVirtioNetworkDeviceConfiguration {}

extern_methods!(
    unsafe impl VZVirtioNetworkDeviceConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
