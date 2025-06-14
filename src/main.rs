// An example program for the C FFI.

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

mod ffi {
  use crate::*;

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

  pub fn get_test(get_test_a: bool) {
    unsafe {
      let ptr = ext::get_test(get_test_a);
      let po = &*ptr;
      (po.f)();
      (po.delete)(ptr);
    }
  }
}

fn main() {
  ffi::test_a();
  ffi::test_b();

  ffi::get_test(true);
  ffi::get_test(false);
}
