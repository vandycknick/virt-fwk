#![allow(non_snake_case)]
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType};

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZMemoryBalloonDeviceConfiguration;

    unsafe impl ClassType for VZMemoryBalloonDeviceConfiguration {
        type Super = NSObject;
    }
);

#[cfg(feature = "not_implemented_yet")]
unsafe impl NSCopying for VZMemoryBalloonDeviceConfiguration {}

unsafe impl NSObjectProtocol for VZMemoryBalloonDeviceConfiguration {}

extern_methods!(
    unsafe impl VZMemoryBalloonDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
