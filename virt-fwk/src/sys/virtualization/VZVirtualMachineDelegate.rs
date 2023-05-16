#![allow(non_snake_case)]
use objc2::runtime::NSObjectProtocol;
use objc2::{extern_protocol, ProtocolType};

use crate::sys::foundation::*;
use crate::sys::virtualization::*;

extern_protocol!(
    /// # Safety
    ///
    /// This delegate should never leak outside this crate.
    pub(crate) unsafe trait VZVirtualMachineDelegate: NSObjectProtocol {
        #[optional]
        #[method(guestDidStopVirtualMachine:)]
        unsafe fn guestDidStopVirtualMachine(&self, virtual_machine: &VZVirtualMachine);

        #[optional]
        #[method(virtualMachine:didStopWithError:)]
        unsafe fn virtualMachine_didStopWithError(
            &self,
            virtual_machine: &VZVirtualMachine,
            error: &NSError,
        );

        #[optional]
        #[method(virtualMachine:networkDevice:attachmentWasDisconnectedWithError:)]
        unsafe fn virtualMachine_networkDevice_attachmentWasDisconnectedWithError(
            &self,
            virtual_machine: &VZVirtualMachine,
            network_device: &VZNetworkDevice,
            error: &NSError,
        );
    }

    unsafe impl ProtocolType for dyn VZVirtualMachineDelegate {}
);
