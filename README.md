# Rust FFI Example Project

An example project for learning the Rust C FFI with an emphasis
on calling C code from Rust.

## Rust example

The file[`src/main.rs`](src/main.rs) has a basic example that calls a C
function through the Rust FFI. The function that is called returns a struct, which itself contains a function
pointer that depends on arguments of the function call. The Rust program then procides an API that calls the C function pointer. The struct that is returned from the C
function also includes a pointer to a cleanup function that calls the libc `free` function,
to delete the memory allocated for the struct. This allows memory allocated
by the C code to be effectively freed from within Rust. The Rust wrappers for
FFI-linked C functions are defined in [`src/ffi.rs`](src/ffi.rs).

## Example C library

The simple C library that is used for this example is in the [`c/`](c/)
subdirectory. See [`c/include/test.h`](c/include/test.h) for the definitions of
functions and types in the public interface of this library.
