#![allow(non_snake_case)]
use crate::sys::foundation::*;
use crate::sys::virtualization::VZStorageDeviceAttachment;
use objc2::ffi::NSInteger;
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType};

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct VZDiskImageStorageDeviceAttachment;

    unsafe impl ClassType for VZDiskImageStorageDeviceAttachment {
        #[inherits(NSObject)]
        type Super = VZStorageDeviceAttachment;
    }
);

unsafe impl NSObjectProtocol for VZDiskImageStorageDeviceAttachment {}

extern_methods!(
    unsafe impl VZDiskImageStorageDeviceAttachment {
        #[method_id(@__retain_semantics Init initWithURL:readOnly:error:_)]
        pub unsafe fn initWithURL_readOnly_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            read_only: bool,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[cfg(feature = "not_implemented_yet")]
        #[method_id(@__retain_semantics Init initWithURL:readOnly:cachingMode:synchronizationMode:error:_)]
        pub unsafe fn initWithURL_readOnly_cachingMode_synchronizationMode_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            read_only: bool,
            caching_mode: VZDiskImageCachingMode,
            synchronization_mode: VZDiskImageSynchronizationMode,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Id<NSURL, Shared>;

        #[method(isReadOnly)]
        pub unsafe fn isReadOnly(&self) -> bool;

        #[method(cachingMode)]
        pub unsafe fn cachingMode(&self) -> NSInteger;

        #[method(synchronizationMode)]
        pub unsafe fn synchronizationMode(&self) -> NSInteger;
    }
);
