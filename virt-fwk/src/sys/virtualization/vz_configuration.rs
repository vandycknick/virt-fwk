#![allow(non_snake_case)]
use objc2::ffi::NSUInteger;
use objc2::rc::{Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_class, extern_methods, ClassType};

use crate::sys::foundation::*;
use crate::sys::virtualization::*;

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub(crate) struct VZVirtualMachineConfiguration;

    unsafe impl ClassType for VZVirtualMachineConfiguration {
        type Super = NSObject;
    }
);

#[cfg(feature = "not_implemented_yet")]
unsafe impl NSCopying for VZVirtualMachineConfiguration {}

unsafe impl NSObjectProtocol for VZVirtualMachineConfiguration {}

extern_methods!(
    unsafe impl VZVirtualMachineConfiguration {
        #[method_id(new)]
        pub fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other bootLoader)]
        pub unsafe fn bootLoader(&self) -> Option<Id<VZBootLoader, Shared>>;

        #[method(setBootLoader:)]
        pub unsafe fn setBootLoader(&self, boot_loader: Option<&VZBootLoader>);

        #[method(memorySize)]
        pub unsafe fn memorySize(&self) -> u64;

        #[method(setMemorySize:)]
        pub unsafe fn setMemorySize(&self, memory_size: u64);

        #[method(CPUCount)]
        pub unsafe fn CPUCount(&self) -> NSUInteger;

        #[method(setCPUCount:)]
        pub unsafe fn setCPUCount(&self, cpu_count: NSUInteger);

        #[cfg(feature = "not_implemented_yet")]
        #[method_id(@__retain_semantics Other platform)]
        pub unsafe fn platform(&self) -> Id<VZPlatformConfiguration>;

        #[cfg(feature = "not_implemented_yet")]
        #[method(setPlatform:)]
        pub unsafe fn setPlatform(&self, platform: &VZPlatformConfiguration);

        #[cfg(feature = "not_implemented_yet")]
        #[method_id(@__retain_semantics Other audioDevices)]
        pub unsafe fn audioDevices(&self) -> Id<NSArray<VZAudioDeviceConfiguration>>;

        #[cfg(feature = "not_implemented_yet")]
        #[method(setAudioDevices:)]
        pub unsafe fn setAudioDevices(&self, audio_devices: &NSArray<VZAudioDeviceConfiguration>);

        #[cfg(feature = "not_implemented_yet")]
        #[method_id(@__retain_semantics Other consoleDevices)]
        pub unsafe fn consoleDevices(&self) -> Id<NSArray<VZConsoleDeviceConfiguration>>;

        #[cfg(feature = "not_implemented_yet")]
        #[method(setConsoleDevices:)]
        pub unsafe fn setConsoleDevices(
            &self,
            console_devices: &NSArray<VZConsoleDeviceConfiguration>,
        );

        #[cfg(feature = "not_implemented_yet")]
        #[method_id(@__retain_semantics Other directorySharingDevices)]
        pub unsafe fn directorySharingDevices(
            &self,
        ) -> Id<NSArray<VZDirectorySharingDeviceConfiguration>>;

        #[cfg(feature = "not_implemented_yet")]
        #[method(setDirectorySharingDevices:)]
        pub unsafe fn setDirectorySharingDevices(
            &self,
            directory_sharing_devices: &NSArray<VZDirectorySharingDeviceConfiguration>,
        );

        #[method_id(@__retain_semantics Other entropyDevices)]
        pub unsafe fn entropyDevices(&self) -> Id<NSArray<VZEntropyDeviceConfiguration>, Shared>;

        #[method(setEntropyDevices:)]
        pub unsafe fn setEntropyDevices(
            &self,
            entropy_devices: &NSArray<VZEntropyDeviceConfiguration>,
        );

        #[method_id(@__retain_semantics Other memoryBalloonDevices)]
        pub unsafe fn memoryBalloonDevices(
            &self,
        ) -> Id<NSArray<VZMemoryBalloonDeviceConfiguration>, Shared>;

        #[method(setMemoryBalloonDevices:)]
        pub unsafe fn setMemoryBalloonDevices(
            &self,
            memory_balloon_devices: &NSArray<VZMemoryBalloonDeviceConfiguration>,
        );

        #[method_id(@__retain_semantics Other networkDevices)]
        pub unsafe fn networkDevices(&self) -> Id<NSArray<VZNetworkDeviceConfiguration>, Shared>;

        #[method(setNetworkDevices:)]
        pub unsafe fn setNetworkDevices(
            &self,
            network_devices: &NSArray<VZNetworkDeviceConfiguration>,
        );

        #[method_id(@__retain_semantics Other serialPorts)]
        pub unsafe fn serialPorts(&self) -> Id<NSArray<VZSerialPortConfiguration>, Shared>;

        #[method(setSerialPorts:)]
        pub unsafe fn setSerialPorts(&self, serial_ports: &NSArray<VZSerialPortConfiguration>);

        #[cfg(feature = "not_implemented_yet")]
        #[method_id(@__retain_semantics Other socketDevices)]
        pub unsafe fn socketDevices(&self) -> Id<NSArray<VZSocketDeviceConfiguration>>;

        #[cfg(feature = "not_implemented_yet")]
        #[method(setSocketDevices:)]
        pub unsafe fn setSocketDevices(
            &self,
            socket_devices: &NSArray<VZSocketDeviceConfiguration>,
        );

        #[method_id(@__retain_semantics Other storageDevices)]
        pub unsafe fn storageDevices(&self) -> Id<NSArray<VZStorageDeviceConfiguration>, Shared>;

        #[method(setStorageDevices:)]
        pub unsafe fn setStorageDevices(
            &self,
            storage_devices: &NSArray<VZStorageDeviceConfiguration>,
        );

        #[cfg(feature = "not_implemented_yet")]
        #[method_id(@__retain_semantics Other keyboards)]
        pub unsafe fn keyboards(&self) -> Id<NSArray<VZKeyboardConfiguration>>;

        #[cfg(feature = "not_implemented_yet")]
        #[method(setKeyboards:)]
        pub unsafe fn setKeyboards(&self, keyboards: &NSArray<VZKeyboardConfiguration>);

        #[cfg(feature = "not_implemented_yet")]
        #[method_id(@__retain_semantics Other pointingDevices)]
        pub unsafe fn pointingDevices(&self) -> Id<NSArray<VZPointingDeviceConfiguration>>;

        #[cfg(feature = "not_implemented_yet")]
        #[method(setPointingDevices:)]
        pub unsafe fn setPointingDevices(
            &self,
            pointing_devices: &NSArray<VZPointingDeviceConfiguration>,
        );

        #[cfg(feature = "not_implemented_yet")]
        #[method_id(@__retain_semantics Other graphicsDevices)]
        pub unsafe fn graphicsDevices(&self) -> Id<NSArray<VZGraphicsDeviceConfiguration>>;

        #[cfg(feature = "not_implemented_yet")]
        #[method(setGraphicsDevices:)]
        pub unsafe fn setGraphicsDevices(
            &self,
            graphics_devices: &NSArray<VZGraphicsDeviceConfiguration>,
        );
    }
);

extern_methods!(
    unsafe impl VZVirtualMachineConfiguration {
        #[method(validateWithError:_)]
        pub unsafe fn validateWithError(&self) -> Result<(), Id<NSError, Shared>>;

        #[method(minimumAllowedMemorySize)]
        pub unsafe fn minimumAllowedMemorySize() -> u64;

        #[method(maximumAllowedMemorySize)]
        pub unsafe fn maximumAllowedMemorySize() -> u64;

        #[method(minimumAllowedCPUCount)]
        pub unsafe fn minimumAllowedCPUCount() -> NSUInteger;

        #[method(maximumAllowedCPUCount)]
        pub unsafe fn maximumAllowedCPUCount() -> NSUInteger;
    }
);
