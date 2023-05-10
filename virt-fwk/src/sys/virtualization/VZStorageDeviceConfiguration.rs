#![allow(non_snake_case)]
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType};

use crate::sys::virtualization::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZStorageDeviceConfiguration;

    unsafe impl ClassType for VZStorageDeviceConfiguration {
        type Super = NSObject;
    }
);

#[cfg(feature = "not_implemented_yet")]
unsafe impl NSCopying for VZStorageDeviceConfiguration {}

unsafe impl NSObjectProtocol for VZStorageDeviceConfiguration {}

extern_methods!(
    unsafe impl VZStorageDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other attachment)]
        pub unsafe fn attachment(&self) -> Id<VZStorageDeviceAttachment, Shared>;
    }
);
