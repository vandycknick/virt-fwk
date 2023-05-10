#![allow(non_snake_case)]
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType};

use crate::sys::virtualization::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZSerialPortConfiguration;

    unsafe impl ClassType for VZSerialPortConfiguration {
        type Super = NSObject;
    }
);

#[cfg(feature = "not_implemented_yet")]
unsafe impl NSCopying for VZSerialPortConfiguration {}

unsafe impl NSObjectProtocol for VZSerialPortConfiguration {}

extern_methods!(
    unsafe impl VZSerialPortConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other attachment)]
        pub unsafe fn attachment(&self) -> Option<Id<VZSerialPortAttachment, Shared>>;

        #[method(setAttachment:)]
        pub unsafe fn setAttachment(&self, attachment: Option<&VZSerialPortAttachment>);
    }
);
