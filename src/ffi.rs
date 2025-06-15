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
  unsafe {
    let ptr = ext::get_test(get_test_a);
    assert!(!ptr.is_null());

    let po = &*ptr;
    (po.f)();

    // frees memory allocated by C code
    (po.delete)(ptr);
  }
}

pub struct SafePolymorphicObject {
  f: extern "C" fn() -> (),
}

impl SafePolymorphicObject {
  pub fn call(&self) {
    (self.f)();
  }
}

pub fn get_test(get_test_a: bool) -> Result<SafePolymorphicObject, String> {
  unsafe {
    let ptr = ext::get_test(get_test_a);
    if ptr.is_null() {
      return Err(String::from("Allocation failure in C get_test call."));
    }

    let po = &*ptr;
    let safe_object = SafePolymorphicObject { f: po.f };

    // frees memory allocated by C code
    (po.delete)(ptr);

    Ok(safe_object)
  }
}
