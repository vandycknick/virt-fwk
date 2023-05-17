#![allow(non_snake_case)]
use std::ffi::c_void;

use block2::Block;
use objc2::ffi::NSInteger;
use objc2::rc::Owned;
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType, ProtocolObject};

use crate::sys::foundation::*;
use crate::sys::queue::*;
use crate::sys::virtualization::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct VZVirtualMachine;

    unsafe impl ClassType for VZVirtualMachine {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for VZVirtualMachine {}

extern_methods!(
    unsafe impl VZVirtualMachine {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithConfiguration:)]
        pub unsafe fn initWithConfiguration(
            this: Option<Allocated<Self>>,
            configuration: &VZVirtualMachineConfiguration,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithConfiguration:queue:)]
        pub unsafe fn initWithConfiguration_queue(
            this: Option<Allocated<Self>>,
            configuration: &VZVirtualMachineConfiguration,
            queue: dispatch_queue_t,
        ) -> Id<Self, Shared>;

        #[method(isSupported)]
        pub unsafe fn isSupported() -> bool;

        #[method(state)]
        pub unsafe fn state(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn VZVirtualMachineDelegate>, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn VZVirtualMachineDelegate>>,
        );

        #[method(canStart)]
        pub unsafe fn canStart(&self) -> bool;

        #[method(canStop)]
        pub unsafe fn canStop(&self) -> bool;

        #[method(canPause)]
        pub unsafe fn canPause(&self) -> bool;

        #[method(canResume)]
        pub unsafe fn canResume(&self) -> bool;

        #[method(canRequestStop)]
        pub unsafe fn canRequestStop(&self) -> bool;

        #[cfg(feature = "not_implemented_yet")]
        #[method_id(@__retain_semantics Other consoleDevices)]
        pub unsafe fn consoleDevices(&self) -> Id<NSArray<VZConsoleDevice>>;

        #[cfg(feature = "not_implemented_yet")]
        #[method_id(@__retain_semantics Other directorySharingDevices)]
        pub unsafe fn directorySharingDevices(&self) -> Id<NSArray<VZDirectorySharingDevice>>;

        #[cfg(feature = "not_implemented_yet")]
        #[method_id(@__retain_semantics Other memoryBalloonDevices)]
        pub unsafe fn memoryBalloonDevices(&self) -> Id<NSArray<VZMemoryBalloonDevice>>;

        #[cfg(feature = "not_implemented_yet")]
        #[method_id(@__retain_semantics Other networkDevices)]
        pub unsafe fn networkDevices(&self) -> Id<NSArray<VZNetworkDevice>>;

        #[cfg(feature = "not_implemented_yet")]
        #[method_id(@__retain_semantics Other socketDevices)]
        pub unsafe fn socketDevices(&self) -> Id<NSArray<VZSocketDevice>>;

        #[method(startWithCompletionHandler:)]
        pub unsafe fn startWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(feature = "not_implemented_yet")]
        #[method(startWithOptions:completionHandler:)]
        pub unsafe fn startWithOptions_completionHandler(
            &self,
            options: &VZVirtualMachineStartOptions,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[method(stopWithCompletionHandler:)]
        pub unsafe fn stopWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[method(pauseWithCompletionHandler:)]
        pub unsafe fn pauseWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[method(resumeWithCompletionHandler:)]
        pub unsafe fn resumeWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[method(requestStopWithError:_)]
        pub unsafe fn requestStopWithError(&self) -> Result<(), Id<NSError, Shared>>;

        #[method(addObserver:forKeyPath:options:context:)]
        pub unsafe fn addObserver_forKeyPath_options_context(
            &self,
            observer: &NSObject,
            key_path: &NSString,
            options: NSKeyValueObservingOptions,
            context: *mut c_void,
        );

        #[method(removeObserver:forKeyPath:context:)]
        pub unsafe fn removeObserver_forKeyPath_context(
            &self,
            observer: &NSObject,
            key_path: &NSString,
            context: *mut c_void,
        );
    }
);

unsafe impl Send for VZVirtualMachine {}
unsafe impl Sync for VZVirtualMachine {}
