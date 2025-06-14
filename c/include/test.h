// test library for FFI.

#include <stdbool.h>

typedef struct PolymorphicObject PolymorphicObject;

typedef void (*TestFn)(void);
typedef void (*FreeFn)(PolymorphicObject *);

typedef struct PolymorphicObject {
  TestFn f;
  FreeFn delete;
} PolymorphicObject;

void test_a();

void test_b();

PolymorphicObject *get_test(bool get_test_a);
