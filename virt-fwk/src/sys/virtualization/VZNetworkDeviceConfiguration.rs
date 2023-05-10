#![allow(non_snake_case)]
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType};

use crate::sys::virtualization::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZNetworkDeviceConfiguration;

    unsafe impl ClassType for VZNetworkDeviceConfiguration {
        type Super = NSObject;
    }
);

#[cfg(feature = "not_implemented_yet")]
unsafe impl NSCopying for VZNetworkDeviceConfiguration {}

unsafe impl NSObjectProtocol for VZNetworkDeviceConfiguration {}

extern_methods!(
    unsafe impl VZNetworkDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other MACAddress)]
        pub unsafe fn MACAddress(&self) -> Id<VZMACAddress, Shared>;

        #[method(setMACAddress:)]
        pub unsafe fn setMACAddress(&self, mac_address: &VZMACAddress);

        #[method_id(@__retain_semantics Other attachment)]
        pub unsafe fn attachment(&self) -> Option<Id<VZNetworkDeviceAttachment, Shared>>;

        #[method(setAttachment:)]
        pub unsafe fn setAttachment(&self, attachment: Option<&VZNetworkDeviceAttachment>);
    }
);
