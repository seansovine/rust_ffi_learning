//! FFI interface to the TestLib C library.

pub use wrappers::*;

/// Foreign types and functions.
mod ext {
    #[repr(C)]
    pub struct PolymorphicObject {
        pub f: extern "C" fn() -> (),
        pub delete: extern "C" fn(po: *const PolymorphicObject) -> (),
    }

    #[link(name = "TestLib")]
    unsafe extern "C" {
        pub fn test_a();
        pub fn test_b();

        /// The bool parameter selects whether `test_a` or `test_b`
        /// function will be pointed to by `PolymorphicObject::f`.
        pub fn get_test(get_test_a: bool) -> *const PolymorphicObject;
    }
}

// Note on poninter constness:
//  We are not directly modifying the pointed-to memory through these
//  pointers, and don't intend to, so semantically I think it makes
//  sense to use the const pointer for them. However, the `delete`
//  function will _free_ the memory that is pointed to. I believe this
//  won't cause a problem, but we should be very careful with such
//  functions, as it would be UB if we allowed a reference continue
//  to refer to a pointed-to location _after_ it's memory is freed.

/// Safe wrappers around foreign types.
mod wrappers {
    use super::*;
    pub fn test_a() {
        unsafe {
            ext::test_a();
        }
    }

    pub fn test_b() {
        unsafe {
            ext::test_b();
        }
    }

    /// Retrieves and calls test_(a|b) depending on parameter. The C
    /// get_test call has a memory allocation, which might fail. In
    /// that case this function call will panic.
    ///
    pub fn call_test(get_test_a: bool) {
        #[allow(unused_assignments)]
        {
            let mut ptr = unsafe { ext::get_test(get_test_a) };
            assert!(!ptr.is_null());

            // Convert raw pointer to reference.
            let po = unsafe { ptr.as_ref().unwrap() };
            (po.f)();

            // Frees memory allocated by C code.
            (po.delete)(ptr);
            // Set pointer null in case additional code is added below.
            ptr = std::ptr::null();
        }
    }

    /// Safe in the sense that it doesn't need to free any memory allocated by
    /// outside code, which could fail if this code used a different allocator
    /// than the one the C code was compiled with.
    ///
    /// On the other hand, there is no guarantee the pointer returned by the C
    /// code, which `f` refers to, is valid, except to trust that the C API is
    /// functioning as expected.
    ///
    pub struct SafePolymorphicObject {
        f: extern "C" fn() -> (),
    }

    impl SafePolymorphicObject {
        pub fn call(&self) {
            (self.f)();
        }
    }

    pub fn get_test(get_test_a: bool) -> Result<SafePolymorphicObject, String> {
        #[allow(unused_assignments)]
        {
            let mut ptr = unsafe { ext::get_test(get_test_a) };
            if ptr.is_null() {
                return Err(String::from("Allocation failure in C get_test call."));
            }

            // Convert raw pointer to reference (unsafe operation).
            let po = unsafe { ptr.as_ref().unwrap() };
            let safe_object = SafePolymorphicObject { f: po.f };

            // Frees memory allocated by C code.
            (po.delete)(ptr);
            // Set pointer null in case additional code is added below.
            ptr = std::ptr::null();

            Ok(safe_object)
        }
    }
}
