use objc2::rc::{Id, Shared};

use crate::sealed::UnsafeGetId;
use crate::sys::foundation::NSArray;
use crate::sys::virtualization::{
    VZEntropyDeviceConfiguration, VZMemoryBalloonDeviceConfiguration, VZNetworkDeviceConfiguration,
    VZSerialPortConfiguration, VZStorageDeviceConfiguration, VZVirtualMachineConfiguration,
};

use crate::bootloader::BootLoader;
use crate::memory::MemoryBalloonDeviceConfiguration;
use crate::network::NetworkDeviceConfiguration;
use crate::randomization::EntropyDeviceConfiguration;
use crate::serial_port::SerialPortConfiguration;
use crate::storage::StorageDeviceConfiguration;

pub struct VirtualMachineConfiguration {
    inner: Id<VZVirtualMachineConfiguration, Shared>,
}

/// The environment attributes and list of devices to use during the configuration of macOS or Linux VMs.
///
/// **Support**: macOS 11.0+
///
/// Use a VirtualMachineConfiguration object to configure the environment for a macOS or Linux VM.
/// This configuration object contains information about the VM environment, including the devices that the VM exposes to the guest operating system.
/// For example, use the configuration object to specify the network interfaces and storage devices that the operating system may access.
impl VirtualMachineConfiguration {
    pub fn default() -> Self {
        VirtualMachineConfiguration {
            inner: VZVirtualMachineConfiguration::new(),
        }
    }

    pub fn new<T: BootLoader>(boot_loader: T, cpus: usize, memory: u64) -> Self {
        let config = Self::default();
        config.set_boot_loader(boot_loader);
        config.set_cpu_count(cpus);
        config.set_memory_size(memory);
        config
    }

    pub fn set_cpu_count(&self, cpus: usize) {
        unsafe {
            self.inner.setCPUCount(cpus);
        }
    }

    pub fn set_memory_size(&self, memory: u64) {
        unsafe {
            self.inner.setMemorySize(memory);
        }
    }

    pub fn set_boot_loader<T: BootLoader>(&self, boot_loader: T) {
        unsafe {
            let boot_loader = boot_loader.id();
            self.inner.setBootLoader(Some(&boot_loader));
        }
    }

    pub fn set_entropy_devices<T>(&self, devices: Vec<T>)
    where
        T: EntropyDeviceConfiguration,
    {
        let entropy_devices: Vec<Id<VZEntropyDeviceConfiguration, Shared>> =
            devices.iter().map(|d| d.id()).collect();

        let entropy_devices = NSArray::from_vec(entropy_devices);
        unsafe {
            self.inner.setEntropyDevices(&entropy_devices);
        }
    }

    pub fn set_serial_ports<T: SerialPortConfiguration>(&self, serial_ports: Vec<T>) {
        let serial_ports: Vec<Id<VZSerialPortConfiguration, Shared>> =
            serial_ports.iter().map(|s| s.id()).collect();

        let serial_ports = NSArray::from_vec(serial_ports);
        unsafe {
            self.inner.setSerialPorts(&serial_ports);
        }
    }

    pub fn set_memory_balloon_devices<T: MemoryBalloonDeviceConfiguration>(&self, devices: Vec<T>) {
        let devices: Vec<Id<VZMemoryBalloonDeviceConfiguration, Shared>> =
            devices.iter().map(|d| d.id()).collect();

        let entropy_devices = NSArray::from_vec(devices);
        unsafe {
            self.inner.setMemoryBalloonDevices(&entropy_devices);
        }
    }

    pub fn set_storage_devices<T: StorageDeviceConfiguration>(&self, devices: Vec<T>) {
        let devices: Vec<Id<VZStorageDeviceConfiguration, Shared>> =
            devices.iter().map(|d| d.id()).collect();

        let entropy_devices = NSArray::from_vec(devices);
        unsafe {
            self.inner.setStorageDevices(&entropy_devices);
        }
    }

    pub fn set_network_devices<T: NetworkDeviceConfiguration>(&self, devices: Vec<T>) {
        let devices: Vec<Id<VZNetworkDeviceConfiguration, Shared>> =
            devices.iter().map(|d| d.id()).collect();

        let network_devices = NSArray::from_vec(devices);
        unsafe {
            self.inner.setNetworkDevices(&network_devices);
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        unsafe {
            let result = self.inner.validateWithError();
            match result {
                Ok(_) => Ok(()),
                Err(ns_error) => Err(ns_error.localized_description()),
            }
        }
    }
}

impl UnsafeGetId<VZVirtualMachineConfiguration> for VirtualMachineConfiguration {
    fn id(&self) -> Id<VZVirtualMachineConfiguration, Shared> {
        self.inner.clone()
    }
}
