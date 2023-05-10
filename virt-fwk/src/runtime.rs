use std::ffi::c_void;

use block2::ConcreteBlock;
use crossbeam_channel::{bounded, Receiver, Sender};
use objc2::rc::{autoreleasepool, Id, Shared};
use objc2::runtime::{NSObject, NSObjectProtocol, Object};
use objc2::ClassType;
use objc2::{declare_class, msg_send_id};

use crate::sealed::UnsafeGetId;
use crate::sys::foundation::{NSArray, NSError, NSString};
use crate::sys::virtualization::{
    NSKeyValueObservingOptions, VZEntropyDeviceConfiguration, VZMemoryBalloonDeviceConfiguration,
    VZNetworkDeviceConfiguration, VZSerialPortConfiguration, VZStorageDeviceConfiguration,
    VZVirtualMachine, VZVirtualMachineConfiguration,
};

use crate::boot_loader::BootLoader;
use crate::entropy_device::EntropyDeviceConfiguration;
use crate::memory_device::MemoryBalloonDeviceConfiguration;
use crate::network_device::NetworkDeviceConfiguration;
use crate::serial_port::SerialPortConfiguration;
use crate::storage_device::StorageDeviceConfiguration;
use crate::sys::queue::{Queue, QueueAttribute};

pub struct VirtualMachineConfiguration {
    inner: Id<VZVirtualMachineConfiguration, Shared>,
}

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

#[derive(Debug)]
pub enum VirtualMachineState {
    Stopped = 0,
    Running = 1,
    Paused = 2,
    Error = 3,
    Starting = 4,
    Pausing = 5,
    Resuming = 6,
    Stopping = 7,
    Unknown = -1,
}

#[derive(Debug)]
pub struct VirtualMachine {
    machine: Id<VZVirtualMachine, Shared>,
    queue: Queue,
    observer: Id<VirtualMachineStateObserver, Shared>,

    notifier: Sender<VirtualMachineState>,
    pub state_notifications: Receiver<VirtualMachineState>,
}

impl VirtualMachine {
    pub fn new(config: &VirtualMachineConfiguration) -> Self {
        unsafe {
            let queue = Queue::create("com.vz.fwk.vm.start", QueueAttribute::Serial);
            let machine = VZVirtualMachine::initWithConfiguration_queue(
                VZVirtualMachine::alloc(),
                &config.id(),
                queue.ptr,
            );

            let (sender, receiver) = bounded(1);
            let observer = VirtualMachineStateObserver::new();

            let key = NSString::from_str("state");

            let mut vm = VirtualMachine {
                machine,
                queue,
                observer,
                notifier: sender,
                state_notifications: receiver,
            };
            let vm_ptr: *mut c_void = &mut vm as *mut _ as *mut c_void;

            vm.machine.addObserver_forKeyPath_options_context(
                &vm.observer,
                &key,
                NSKeyValueObservingOptions::NSKeyValueObservingOptionNew,
                vm_ptr,
            );

            vm
        }
    }

    pub fn get_state_channel(&self) -> Receiver<VirtualMachineState> {
        self.state_notifications.clone()
    }

    pub fn supported() -> bool {
        unsafe { VZVirtualMachine::isSupported() }
    }

    pub fn start(&self) -> Result<(), String> {
        unsafe {
            let (tx, rx) = std::sync::mpsc::channel();
            let dispatch_block = ConcreteBlock::new(move || {
                let inner_tx = tx.clone();
                let completion_handler = ConcreteBlock::new(move |err: *mut NSError| {
                    if err.is_null() {
                        inner_tx.send(Ok(())).unwrap();
                    } else {
                        inner_tx
                            .send(Err(err.as_mut().unwrap().localized_description()))
                            .unwrap();
                    }
                });

                let completion_handler = completion_handler.copy();
                self.machine.startWithCompletionHandler(&completion_handler);
            });

            let dispatch_block_clone = dispatch_block.clone();
            self.queue.exec_block_async(&dispatch_block_clone);

            let result = rx.recv();

            if let Err(_) = result {
                return Err("TODO: implement better error handling here!".into());
            }

            if let Err(nse) = result.unwrap() {
                return Err(nse);
            }

            return Ok(());
        }
    }

    pub fn stop(&self) -> Result<(), String> {
        let (tx, rx) = std::sync::mpsc::channel();
        let dispatch_block = ConcreteBlock::new(move || {
            let inner_tx = tx.clone();
            unsafe {
                let completion_handler = ConcreteBlock::new(move |err: *mut NSError| {
                    if err.is_null() {
                        inner_tx.send(Ok(())).unwrap();
                    } else {
                        inner_tx
                            .send(Err(err.as_mut().unwrap().localized_description()))
                            .unwrap();
                    }
                });

                let completion_handler = completion_handler.copy();
                self.machine.stopWithCompletionHandler(&completion_handler);
            }
        });

        let dispatch_block_clone = dispatch_block.clone();
        self.queue.exec_block_async(&dispatch_block_clone);

        let result = rx.recv();

        if let Err(_) = result {
            return Err("TODO: implement better error handling here!".into());
        }

        if let Err(nse) = result.unwrap() {
            return Err(nse);
        }

        return Ok(());
    }

    pub fn can_start(&self) -> bool {
        self.queue
            .exec_sync(move || unsafe { self.machine.canStart() })
    }

    pub fn can_stop(&self) -> bool {
        self.queue
            .exec_sync(move || unsafe { self.machine.canRequestStop() })
    }

    pub fn can_pause(&self) -> bool {
        self.queue
            .exec_sync(move || unsafe { self.machine.canPause() })
    }

    pub fn can_resume(&self) -> bool {
        self.queue
            .exec_sync(move || unsafe { self.machine.canResume() })
    }

    pub fn can_request_stop(&self) -> bool {
        self.queue
            .exec_sync(move || unsafe { self.machine.canRequestStop() })
    }

    pub fn state(&self) -> VirtualMachineState {
        unsafe {
            match self.machine.state() {
                0 => VirtualMachineState::Stopped,
                1 => VirtualMachineState::Running,
                2 => VirtualMachineState::Paused,
                3 => VirtualMachineState::Error,
                4 => VirtualMachineState::Starting,
                5 => VirtualMachineState::Pausing,
                6 => VirtualMachineState::Resuming,
                7 => VirtualMachineState::Stopping,
                _ => VirtualMachineState::Unknown,
            }
        }
    }
}

impl Drop for VirtualMachine {
    fn drop(&mut self) {
        let key_path = NSString::from_str("state");

        let vm_ptr: *mut c_void = self as *mut _ as *mut c_void;

        unsafe {
            self.machine
                .removeObserver_forKeyPath_context(&self.observer, &key_path, vm_ptr);
        }
    }
}

declare_class!(
    #[derive(Debug)]
    struct VirtualMachineStateObserver;

    unsafe impl ClassType for VirtualMachineStateObserver {
        type Super = NSObject;
        const NAME: &'static str = "VirtualMachineStateObserver";
    }

    unsafe impl VirtualMachineStateObserver {
        #[method(observeValueForKeyPath:ofObject:change:context:)]
        unsafe fn observe_value_for_key_path(
            &self,
            key_path: Option<&NSString>,
            _object: Option<&NSObject>,
            _change: Option<&Object>,
            context: *mut c_void,
        ) {
            if let Some(msg) = key_path {
                let key = autoreleasepool(|pool| msg.as_str(pool).to_owned());

                if key == "state" {
                    let vm: &mut VirtualMachine = &mut *(context as *mut VirtualMachine);
                    let _ = vm.state_notifications.try_recv();
                    vm.notifier.send(vm.state()).unwrap();
                }
            }
        }
    }
);

unsafe impl NSObjectProtocol for VirtualMachineStateObserver {}

unsafe impl Send for VirtualMachineStateObserver {}

unsafe impl Sync for VirtualMachineStateObserver {}

impl VirtualMachineStateObserver {
    pub fn new() -> Id<Self, Shared> {
        unsafe { msg_send_id![Self::alloc(), init] }
    }
}
