#![allow(non_snake_case)]
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType};

use crate::sys::foundation::*;
use crate::sys::virtualization::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZFileHandleSerialPortAttachment;

    unsafe impl ClassType for VZFileHandleSerialPortAttachment {
        #[inherits(NSObject)]
        type Super = VZSerialPortAttachment;
    }
);

unsafe impl NSObjectProtocol for VZFileHandleSerialPortAttachment {}

extern_methods!(
    unsafe impl VZFileHandleSerialPortAttachment {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithFileHandleForReading:fileHandleForWriting:)]
        pub unsafe fn initWithFileHandleForReading_fileHandleForWriting(
            this: Option<Allocated<Self>>,
            file_handle_for_reading: Option<&NSFileHandle>,
            file_handle_for_writing: Option<&NSFileHandle>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other fileHandleForReading)]
        pub unsafe fn fileHandleForReading(&self) -> Option<Id<NSFileHandle, Shared>>;

        #[method_id(@__retain_semantics Other fileHandleForWriting)]
        pub unsafe fn fileHandleForWriting(&self) -> Option<Id<NSFileHandle, Shared>>;
    }
);
