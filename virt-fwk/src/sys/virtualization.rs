#[link(name = "Virtualization", kind = "framework")]
extern "C" {}

mod vz_bootloader;
mod vz_configuration;
mod vz_memory_configuration;
mod vz_memory_devices;
mod vz_network_attachments;
mod vz_network_configuration;
mod vz_network_device;
mod vz_randomization_configurations;
mod vz_runtime;
mod vz_serialports_attachments;
mod vz_serialports_configurations;
mod vz_storage_attachments;
mod vz_storage_configurations;

pub(crate) use self::vz_bootloader::*;
pub(crate) use self::vz_configuration::*;
pub(crate) use self::vz_memory_configuration::*;
// pub(crate) use self::vz_memory_devices::*;
pub(crate) use self::vz_network_attachments::*;
pub(crate) use self::vz_network_configuration::*;
pub(crate) use self::vz_network_device::*;
pub(crate) use self::vz_randomization_configurations::*;
pub(crate) use self::vz_runtime::*;
pub(crate) use self::vz_serialports_attachments::*;
pub(crate) use self::vz_serialports_configurations::*;
pub(crate) use self::vz_storage_attachments::*;
pub(crate) use self::vz_storage_configurations::*;
