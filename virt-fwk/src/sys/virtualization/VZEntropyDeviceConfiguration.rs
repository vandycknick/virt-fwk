#![allow(non_snake_case)]
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType};

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZEntropyDeviceConfiguration;

    unsafe impl ClassType for VZEntropyDeviceConfiguration {
        type Super = NSObject;
    }
);

#[cfg(feature = "not_implemented_yet")]
unsafe impl NSCopying for VZEntropyDeviceConfiguration {}

unsafe impl NSObjectProtocol for VZEntropyDeviceConfiguration {}

extern_methods!(
    unsafe impl VZEntropyDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
