#![allow(non_snake_case)]
use core::ffi::{c_int, c_void};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::str;

use objc2::ffi::{NSInteger, NSUInteger};
use objc2::rc::{autoreleasepool, Allocated, AutoreleasePool, Id, Ownership, Shared};
use objc2::runtime::{NSObject, Object, __nsstring::nsstring_to_str};
use objc2::{__inner_extern_class, extern_class, extern_methods, ClassType};
use objc2::{msg_send, Message};

const UTF8_ENCODING: usize = 4;

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSString;

    unsafe impl ClassType for NSString {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSString {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);

impl NSString {
    pub fn as_str<'r, 's: 'r, 'p: 'r>(&'s self, pool: &'p AutoreleasePool) -> &'r str {
        // SAFETY: This is an instance of `NSString`
        unsafe { nsstring_to_str(self, pool) }
    }

    pub fn from_str(string: &str) -> Id<Self, Shared> {
        unsafe {
            let bytes: *const c_void = string.as_ptr().cast();
            let obj: *mut Object = msg_send![Self::class(), alloc];
            let obj: *mut Object = msg_send![
                obj,
                initWithBytes: bytes,
                length: string.len(),
                encoding: UTF8_ENCODING,
            ];
            Id::new(obj.cast()).unwrap()
        }
    }
}

impl Debug for NSString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // The call to `to_owned` is unfortunate, but is required to work
        // around `f` not being AutoreleaseSafe.
        // TODO: Fix this!
        let s = autoreleasepool(|pool| self.as_str(pool).to_owned());
        std::fmt::Debug::fmt(&s, f)
    }
}

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub(crate) struct NSURL;

    unsafe impl ClassType for NSURL {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSURL {
        #[method_id(@__retain_semantics Init initFileURLWithPath:isDirectory:)]
        pub(crate) unsafe fn init_file_url_with_path(
            this: Option<Allocated<Self>>,
            path: &NSString,
            is_dir: bool,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other absoluteString)]
        pub(crate) fn absolute_string(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other absoluteURL)]
        pub(crate) fn absolute_url(&self) -> Option<Id<NSURL, Shared>>;
    }
);

impl NSURL {
    pub fn file_url_with_path(path: &str, is_directory: bool) -> Id<Self, Shared> {
        let path_nsstring = NSString::from_str(path);
        unsafe {
            let url = Self::init_file_url_with_path(Self::alloc(), &path_nsstring, is_directory);
            url
        }
    }
}

impl Debug for NSURL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NSURL")
            .field("__inner", &self.__inner)
            .finish()
    }
}

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSArray<ObjectType: Message = Object, ObjectTypeOwnership: Ownership = Shared> {
        _inner0: PhantomData<*mut (ObjectType, ObjectTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> ClassType
        for NSArray<ObjectType, ObjectTypeOwnership>
    {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSArray<ObjectType, ObjectTypeOwnership>
    {
        #[method(count)]
        pub fn count(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other objectAtIndex:)]
        pub unsafe fn object_at_index(
            &self,
            index: NSUInteger,
        ) -> Id<ObjectType, ObjectTypeOwnership>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithObjects:count:)]
        pub unsafe fn init_with_objects_count(
            this: Option<Allocated<Self>>,
            objects: *mut NonNull<ObjectType>,
            cnt: NSUInteger,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Creation methods.
    unsafe impl<T: Message> NSArray<T> {
        // https://github.com/madsmtm/objc2/blob/dd6c804d1e189fd8c9b313d8c1901ee9e7286551/crates/icrate/src/Foundation/additions/array.rs#L23
        pub fn from_vec(mut vec: Vec<Id<T, Shared>>) -> Id<Self, Shared> {
            // We intentionally extract the length before we access the
            // pointer as mutable, to not invalidate that mutable pointer.
            let len = vec.len();
            // SAFETY: `Id<T>` has the same memory layout as `NonNull<T>`, and
            // stronger guarantees.
            let ptr: *mut NonNull<T> = vec.as_mut_ptr().cast();
            // SAFETY: We've consumed the `Id<T>`s, which means that we can
            // now safely take ownership (even if `T` is mutable).
            unsafe { Self::init_with_objects_count(Self::alloc(), ptr, len) }
            // The drop of `Vec` here would invalidate our mutable pointer,
            // except for the fact that we're using `UnsafeCell` in `Object`.
        }
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct NSError;

    unsafe impl ClassType for NSError {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSError {
        #[method(code)]
        pub fn code(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other localizedDescription)]
        pub fn localizedDescription(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other localizedFailureReason)]
        pub unsafe fn localizedFailureReason(&self) -> Option<Id<NSString, Shared>>;
    }
);

impl NSError {
    pub fn localized_description(&self) -> String {
        let description = self.localizedDescription();
        autoreleasepool(|pool| description.as_str(pool).to_owned())
    }
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFileHandle;

    unsafe impl ClassType for NSFileHandle {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSFileHandle {
        #[method_id(@__retain_semantics Init initWithFileDescriptor:)]
        pub unsafe fn initWithFileDescriptor(
            this: Option<Allocated<Self>>,
            fd: c_int,
        ) -> Id<Self, Shared>;

        #[method(fileDescriptor)]
        pub unsafe fn fileDescriptor(&self) -> c_int;
    }
);
