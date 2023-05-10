#![allow(non_snake_case)]
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType};

use crate::sys::foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZMACAddress;

    unsafe impl ClassType for VZMACAddress {
        type Super = NSObject;
    }
);

#[cfg(feature = "not_implemented_yet")]
unsafe impl NSCopying for VZMACAddress {}

unsafe impl NSObjectProtocol for VZMACAddress {}

extern_methods!(
    unsafe impl VZMACAddress {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "not_implemented_yet")]
        #[method_id(@__retain_semantics Init initWithEthernetAddress:)]
        pub unsafe fn initWithEthernetAddress(
            this: Option<Allocated<Self>>,
            ethernet_address: ether_addr_t,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithString:)]
        pub unsafe fn initWithString(
            this: Option<Allocated<Self>>,
            string: &NSString,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other randomLocallyAdministeredAddress)]
        pub unsafe fn randomLocallyAdministeredAddress() -> Id<Self, Shared>;

        #[cfg(feature = "not_implemented_yet")]
        #[method(ethernetAddress)]
        pub unsafe fn ethernetAddress(&self) -> ether_addr_t;

        #[method_id(@__retain_semantics Other string)]
        pub unsafe fn string(&self) -> Id<NSString, Shared>;

        #[method(isBroadcastAddress)]
        pub unsafe fn isBroadcastAddress(&self) -> bool;

        #[method(isMulticastAddress)]
        pub unsafe fn isMulticastAddress(&self) -> bool;

        #[method(isUnicastAddress)]
        pub unsafe fn isUnicastAddress(&self) -> bool;

        #[method(isLocallyAdministeredAddress)]
        pub unsafe fn isLocallyAdministeredAddress(&self) -> bool;

        #[method(isUniversallyAdministeredAddress)]
        pub unsafe fn isUniversallyAdministeredAddress(&self) -> bool;
    }
);
