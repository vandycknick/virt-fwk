#![allow(non_snake_case)]
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType};

use crate::sys::virtualization::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct VZNATNetworkDeviceAttachment;

    unsafe impl ClassType for VZNATNetworkDeviceAttachment {
        #[inherits(NSObject)]
        type Super = VZNetworkDeviceAttachment;
    }
);

unsafe impl NSObjectProtocol for VZNATNetworkDeviceAttachment {}

extern_methods!(
    unsafe impl VZNATNetworkDeviceAttachment {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
