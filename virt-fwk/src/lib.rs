//! A rust interface into Apple's virtualization framework.
//!
//! This rust library exposes a safe set of high-level API that directly interfaces with Apple's Virtualization framework.
//! These API's can be used to create and manage virtual machines (VM) on Apple silicon and Intel-based Mac computers.
//! Use this framework to boot and run macOS or Linux-based operating systems in custom environments that you define.
//! The framework supports the Virtual I/O Device (VIRTIO) specification, which defines standard interfaces for many device types,
//! including network, socket, serial port, storage, entropy, and memory-balloon devices.
//!
//! ## Basic Usage
//!
//! For more in depth examples have a look at the [examples](https://github.com/vandycknick/virt-fwk/tree/main/examples) folder.
//! - [simple-vm](https://github.com/vandycknick/virt-fwk/tree/main/examples/simple-vm): This example is inspired of the [SimpleVM](https://github.com/KhaosT/SimpleVM) project and creates a basic Linux VM.
//!
#![cfg_attr(feature = "apple", doc = "```")]
#![cfg_attr(not(feature = "apple"), doc = "```no_run")]
//! use virt_fwk::{LinuxBootLoader, VirtualMachine}
//!
//! // Detect support for the Apple's virtualization API's
//! if !VirtualMachine::supported() {
//!     println!("VirtualMachine not supported");
//!     process::exit(1);
//! }
//!
//! // Creating a boot loader for a Linux VM
//! let boot_loader = LinuxBootLoader::new("/path/to/linux_kernel", "/path/to/initramfs", "console=hvc0");
//!
//! // Create new VM Configuration
//! let config = VirtualMachineConfiguration::new(boot_loader, 2,  2 * 1024 * 1024 * 1024);
//!
//! // Attach network, sockets, serial port, storage, entropy and other devices to the VM configuration.
//! // ...
//!
//! // Verify if the configuration is valid.
//! if let Err(msg) = config.validate() {
//!     println!("Invalid Configuration: {}", msg);
//!     process::exit(1);
//! }
//!
//! // Create and start the virtual machine.
//! let vm = VirtualMachine::new(&config);
//! vm.start()?;
//!
//! // Keep running until the VM stops.
//! loop {
//!     if let Ok(vz::VirtualMachineState::Stopped) = state_changes.recv() {
//!         process::exit(0);
//!     }
//! }
//! ```
//!
//! ## Notes
//!
//! ### Adding the Virtualization Entitlement to Your Project
//! To use the Virtualization APIs, a process must have the [`com.apple.security.virtualization`](https://developer.apple.com/documentation/bundleresources/entitlements/com_apple_security_virtualization) entitlement.
//! To add this create an entitlements file with the following contents:
//! ```toml
//! <?xml version="1.0" encoding="UTF-8"?>
//! <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
//! <plist version="1.0">
//! <dict>
//!     <key>com.apple.security.virtualization</key>
//!     <true/>
//! </dict>
//! </plist>
//! ```
//! And then sign you program with this entitlement using the following command:
//!
//! ```sh
//! $ codesign --entitlements <FILENAME>.entitlements -s - <YOUR BINARY PATH>
//! ```
//!
//! If you want to use [`VZBridgedNetworkDeviceAttachment`](https://developer.apple.com/documentation/virtualization/vzbridgednetworkdeviceattachment?language=objc), you need to add also [`com.apple.vm.networking`](https://developer.apple.com/documentation/bundleresources/entitlements/com_apple_vm_networking) entitlement.

mod boot_loader;
mod entropy_device;
mod memory_device;
mod network_device;
mod runtime;
mod serial_port;
mod storage_device;
mod sys;

pub use crate::boot_loader::*;
pub use crate::entropy_device::*;
pub use crate::memory_device::*;
pub use crate::network_device::*;
pub use crate::runtime::*;
pub use crate::serial_port::*;
pub use crate::storage_device::*;

pub(crate) mod sealed {
    use objc2::rc::Id;
    use objc2::rc::Shared;

    pub trait UnsafeGetId<T> {
        fn id(&self) -> Id<T, Shared>;
    }
}
