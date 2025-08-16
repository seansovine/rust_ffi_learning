// A simple example program for demonstrating the use of the C FFI.

mod ffi;

fn main() -> Result<(), String> {
    println!("Calling test functions directly:");

    ffi::test_a();
    ffi::test_b();

    println!("\nCalling test functions via call_test:");

    ffi::call_test(true);
    ffi::call_test(false);

    println!("\nCalling functions via wrapper structs:");

    let safe_object = ffi::get_test(true)?;
    safe_object.call();

    let safe_object = ffi::get_test(false)?;
    safe_object.call();

    Ok(())
}
