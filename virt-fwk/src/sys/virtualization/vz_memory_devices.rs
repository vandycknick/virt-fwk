#![allow(non_snake_case)]
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType};

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZMemoryBalloonDevice;

    unsafe impl ClassType for VZMemoryBalloonDevice {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for VZMemoryBalloonDevice {}

extern_methods!(
    unsafe impl VZMemoryBalloonDevice {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZVirtioTraditionalMemoryBalloonDevice;

    unsafe impl ClassType for VZVirtioTraditionalMemoryBalloonDevice {
        #[inherits(NSObject)]
        type Super = VZMemoryBalloonDevice;
    }
);

unsafe impl NSObjectProtocol for VZVirtioTraditionalMemoryBalloonDevice {}

extern_methods!(
    unsafe impl VZVirtioTraditionalMemoryBalloonDevice {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method(targetVirtualMachineMemorySize)]
        pub unsafe fn targetVirtualMachineMemorySize(&self) -> u64;

        #[method(setTargetVirtualMachineMemorySize:)]
        pub unsafe fn setTargetVirtualMachineMemorySize(
            &self,
            target_virtual_machine_memory_size: u64,
        );
    }
);
