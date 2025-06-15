//! FFI interface to the TestLib C library.

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

    pub fn get_test(get_test_a: bool) -> *const PolymorphicObject;
  }
}

// Safe wrappers around foreign types.

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

/// Retrieves and calls test_(a|b) depending on parameter.
/// The C get_test call has a memory allocation, which might
/// fail. In that case this function call will panic.
pub fn call_test(get_test_a: bool) {
  #[allow(unused_assignments)]
  unsafe {
    let mut ptr = ext::get_test(get_test_a);
    assert!(!ptr.is_null());

    let po = &*ptr;
    (po.f)();

    // frees memory allocated by C code
    (po.delete)(ptr);
    // set null in case additional code is added below
    ptr = std::ptr::null();
  }
}

/// Safe in the sense that it doesn't need to free any
/// memory allocated by outside code. On the other hand,
/// there is no guarantee the pointer returned by the C
/// code, which `f` refers to, is valid, except to trust
/// that the C API is functioning as expected.
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
  unsafe {
    let mut ptr = ext::get_test(get_test_a);
    if ptr.is_null() {
      return Err(String::from("Allocation failure in C get_test call."));
    }

    let po = &*ptr;
    let safe_object = SafePolymorphicObject { f: po.f };

    // frees memory allocated by C code
    (po.delete)(ptr);
    // set null in case additional code is added below
    ptr = std::ptr::null();

    Ok(safe_object)
  }
}
