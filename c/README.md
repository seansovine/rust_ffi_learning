# Test C Library

A simple C library with a function that returns a function pointer.
We will use this as an example to create a Rust C FFI interface that
receives a function pointer from a C function and then calls that
pointer.

## Makefile

Has targets for configure, build, test, checks, etc.

## Struct `PolymorphicObject`

The `get_test` function returns the following struct:

```c
typedef struct PolymorphicObject {
  TestFn f;
  FreeFn delete;
} PolymorphicObject;
```

The `f` function pointer will vary depending on the `get_test` arguments,
and the `delete` member points to a function that wraps the libc `free`
function. This is needed so that the memory allocated for the struct can
be freed easily on the Rust side of the FFI interface.
