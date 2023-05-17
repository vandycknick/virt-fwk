#![allow(non_snake_case)]
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType};

// region: VZEntropyDeviceConfiguration
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
// endregion

// region: VZVirtioEntropyDeviceConfiguration
extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZVirtioEntropyDeviceConfiguration;

    unsafe impl ClassType for VZVirtioEntropyDeviceConfiguration {
        #[inherits(NSObject)]
        type Super = VZEntropyDeviceConfiguration;
    }
);

#[cfg(feature = "not_implemented_yet")]
unsafe impl NSCopying for VZVirtioEntropyDeviceConfiguration {}

unsafe impl NSObjectProtocol for VZVirtioEntropyDeviceConfiguration {}

extern_methods!(
    unsafe impl VZVirtioEntropyDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
// endregion
