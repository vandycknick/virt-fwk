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
