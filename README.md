# Rust FFI Example Project

An example project for learning the Rust C FFI with an emphasis
on calling C code from Rust.

## Rust example

[`src/main.rs`](src/main.rs) currently contains a basic example of
calling a C function that returns a struct containing a function pointer
that depends on arguments of the function call, and then calling the C
function pointer from within Rust. The struct that is returned from
the C function includes a pointer to a function that calls the libc
`free` function to delete the memory allocated for the struct, so that
memory allocated by the C code can be freed from within Rust.

## Example C library

The C library that is used for this example is in the `c/` subdirectory.
See [`c/include/test.h`](c/include/test.h) for the definitions.
