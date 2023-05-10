#[link(name = "Virtualization", kind = "framework")]
extern "C" {}

#[path = "virtualization/mod.rs"]
mod virtualization;

pub(crate) use self::virtualization::VZBootLoader::*;
pub(crate) use self::virtualization::VZBridgedNetworkInterface::*;
pub(crate) use self::virtualization::VZDiskImageStorageDeviceAttachment::*;
pub(crate) use self::virtualization::VZEntropyDeviceConfiguration::*;
pub(crate) use self::virtualization::VZFileHandleSerialPortAttachment::*;
pub(crate) use self::virtualization::VZLinuxBootLoader::*;
pub(crate) use self::virtualization::VZMACAddress::*;
pub(crate) use self::virtualization::VZMemoryBalloonDeviceConfiguration::*;
pub(crate) use self::virtualization::VZNATNetworkDeviceAttachment::*;
pub(crate) use self::virtualization::VZNetworkDevice::*;
pub(crate) use self::virtualization::VZNetworkDeviceAttachment::*;
pub(crate) use self::virtualization::VZNetworkDeviceConfiguration::*;
pub(crate) use self::virtualization::VZSerialPortAttachment::*;
pub(crate) use self::virtualization::VZSerialPortConfiguration::*;
pub(crate) use self::virtualization::VZStorageDeviceAttachment::*;
pub(crate) use self::virtualization::VZStorageDeviceConfiguration::*;
pub(crate) use self::virtualization::VZVirtioBlockDeviceConfiguration::*;
pub(crate) use self::virtualization::VZVirtioConsoleDeviceSerialPortConfiguration::*;
pub(crate) use self::virtualization::VZVirtioEntropyDeviceConfiguration::*;
pub(crate) use self::virtualization::VZVirtioNetworkDeviceConfiguration::*;
pub(crate) use self::virtualization::VZVirtioTraditionalMemoryBalloonDeviceConfiguration::*;
pub(crate) use self::virtualization::VZVirtualMachine::*;
pub(crate) use self::virtualization::VZVirtualMachineConfiguration::*;
pub(crate) use self::virtualization::VZVirtualMachineDelegate::*;
