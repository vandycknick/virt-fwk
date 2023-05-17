#![allow(non_snake_case)]
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType};

use crate::sys::foundation::{NSArray, NSString};

// region: VZNetworkDeviceAttachment
extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZNetworkDeviceAttachment;

    unsafe impl ClassType for VZNetworkDeviceAttachment {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for VZNetworkDeviceAttachment {}

extern_methods!(
    unsafe impl VZNetworkDeviceAttachment {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
// endregion

// region: VZNATNetworkDeviceAttachment
extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct VZNATNetworkDeviceAttachment;

    unsafe impl ClassType for VZNATNetworkDeviceAttachment {
        #[inherits(NSObject)]
        type Super = VZNetworkDeviceAttachment;
    }
);

unsafe impl NSObjectProtocol for VZNATNetworkDeviceAttachment {}

extern_methods!(
    unsafe impl VZNATNetworkDeviceAttachment {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
// endregion

// region: VZBridgedNetworkInterface
extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZBridgedNetworkInterface;

    unsafe impl ClassType for VZBridgedNetworkInterface {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for VZBridgedNetworkInterface {}

extern_methods!(
    unsafe impl VZBridgedNetworkInterface {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other networkInterfaces)]
        pub unsafe fn networkInterfaces() -> Id<NSArray<VZBridgedNetworkInterface>, Shared>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other localizedDisplayName)]
        pub unsafe fn localizedDisplayName(&self) -> Option<Id<NSString, Shared>>;
    }
);
// endregion

// region: VZBridgedNetworkDeviceAttachment
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
// endregion
