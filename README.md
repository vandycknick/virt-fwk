# virt-fwk - Rust bindings for Apple's [Virtualization Framework](https://developer.apple.com/documentation/virtualization?language=objc)

virt-fwk exposes a safe set of API's unlocking the capabilities of Apple's Virtualization.framework in rust. To explain Virtualization.framework I refer to the following quote directly from Apple's documentation.

> The Virtualization framework provides high-level APIs for creating and managing virtual machines (VM) on Apple silicon and Intel-based Mac computers. Use this framework to boot and run macOS or Linux-based operating systems in custom environments that you define. The framework supports the Virtual I/O Device (VIRTIO) specification, which defines standard interfaces for many device types, including network, socket, serial port, storage, entropy, and memory-balloon devices.

## Requirements.

This is still very much work in progress. As such the library is currently only with the latest macOS and Rust versions:

-   Higher or equal to macOS Ventura (13).
-   Lates version of rust 1.69.0, although it's very likely that earlier versions will work as well. Only the latest version has been tested.

## Getting started

Add the following lines to your `Cargo.toml` file or run `cargo add virt-fwk`.

```toml
[dependencies]
virt-fw = "0.1.0"
```

Use the following if you prefer to use the package via the global `vz` module name. This allows you to use the package in a similar fashion as to how the `vz` [golang library](https://github.com/Code-Hex/vz) works:

```toml
[dependencies]
vz = { package="virt-fwk", version="0.1.0" }
```

### Important

In order to use the macOS virtualization api's your program should have the `com.apple.security.virtualization` entitlement. To add this create an entitlements file with the following contents:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.security.virtualization</key>
    <true/>
</dict>
</plist>
```

And then sign you program with this entitlement using the following command:

```sh
$ codesign --entitlements <FILENAME>.entitlements -s - <YOUR BINARY PATH>
```

If you want to use [`VZBridgedNetworkDeviceAttachment`](https://developer.apple.com/documentation/virtualization/vzbridgednetworkdeviceattachment?language=objc), you need to add also `com.apple.vm.networking` entitlement.
