#![allow(non_snake_case)]
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType};

use crate::sys::foundation::*;
use crate::sys::virtualization::*;

// region: VZStorageDeviceConfiguration
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
// endregion

// region: VZVirtioBlockDeviceConfiguration
extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct VZVirtioBlockDeviceConfiguration;

    unsafe impl ClassType for VZVirtioBlockDeviceConfiguration {
        #[inherits(NSObject)]
        type Super = VZStorageDeviceConfiguration;
    }
);

#[cfg(feature = "not_implemented_yet")]
unsafe impl NSCopying for VZVirtioBlockDeviceConfiguration {}

unsafe impl NSObjectProtocol for VZVirtioBlockDeviceConfiguration {}

extern_methods!(
    unsafe impl VZVirtioBlockDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithAttachment:)]
        pub unsafe fn initWithAttachment(
            this: Option<Allocated<Self>>,
            attachment: &VZStorageDeviceAttachment,
        ) -> Id<Self, Shared>;

        #[method(validateBlockDeviceIdentifier:error:_)]
        pub unsafe fn validateBlockDeviceIdentifier_error(
            block_device_identifier: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other blockDeviceIdentifier)]
        pub unsafe fn blockDeviceIdentifier(&self) -> Id<NSString, Shared>;

        #[method(setBlockDeviceIdentifier:)]
        pub unsafe fn setBlockDeviceIdentifier(&self, block_device_identifier: &NSString);
    }
);
// endregion
