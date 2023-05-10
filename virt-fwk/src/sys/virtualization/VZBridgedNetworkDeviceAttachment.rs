#![allow(non_snake_case)]

use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType};

use crate::sys::virtualization::{VZBridgedNetworkInterface, VZNetworkDeviceAttachment};

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZBridgedNetworkDeviceAttachment;

    unsafe impl ClassType for VZBridgedNetworkDeviceAttachment {
        #[inherits(NSObject)]
        type Super = VZNetworkDeviceAttachment;
    }
);

unsafe impl NSObjectProtocol for VZBridgedNetworkDeviceAttachment {}

extern_methods!(
    unsafe impl VZBridgedNetworkDeviceAttachment {
        #[method_id(@__retain_semantics Init initWithInterface:)]
        pub unsafe fn initWithInterface(
            this: Option<Allocated<Self>>,
            interface: &VZBridgedNetworkInterface,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other interface)]
        pub unsafe fn interface(&self) -> Id<VZBridgedNetworkInterface, Shared>;
    }
);
