# Rust FFI Example Project

An example project for learning the Rust C FFI with an emphasis
on calling C code from Rust.

## Rust example

The file[`src/main.rs`](src/main.rs) has a basic example of calling a C
function through the Rust FFI that returns a struct containing a function
pointer that depends on arguments of the function call. It then calls the C
function pointer from within Rust. The struct that is returned from the C
function includes a pointer to a function that calls the libc `free` function
to delete the memory allocated for the struct. This allows memory allocated
by the C code to be conveniently freed from within Rust. Rust wrappers for
the FFI-linked C functions are defined in [`src/ffi.rs`](src/ffi.rs).

## Example C library

The simple example C library that is used for this example is in the [`c/`](c/)
subdirectory. See [`c/include/test.h`](c/include/test.h) for the definitions of
functions and types in the public interface of this library.
