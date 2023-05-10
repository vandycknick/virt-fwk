#![allow(non_snake_case)]
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType};

use crate::sys::foundation::{NSArray, NSString};

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
