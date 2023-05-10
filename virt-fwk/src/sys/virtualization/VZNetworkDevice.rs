#![allow(non_snake_case)]
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType};
use std::fmt::Debug;

use crate::sys::virtualization::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZNetworkDevice;

    unsafe impl ClassType for VZNetworkDevice {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for VZNetworkDevice {}

extern_methods!(
    unsafe impl VZNetworkDevice {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other attachment)]
        pub unsafe fn attachment(&self) -> Option<Id<VZNetworkDeviceAttachment, Shared>>;

        #[method(setAttachment:)]
        pub unsafe fn setAttachment(&self, attachment: Option<&VZNetworkDeviceAttachment>);
    }
);
