#include "test.h"

#include <stdio.h>
#include <stdlib.h>

void test_a() { printf("Test A running!\n"); }

void test_b() { printf("Test B running!\n"); }

// A wrapper around libc free for use on the other side
// of the FFI interface.
void delete(PolymorphicObject *po) { free(po); }

// Returns an object containing a pointer to either
// test_a or test_b, depending on the parameter value.
PolymorphicObject *get_test(bool get_test_a) {
  PolymorphicObject *po = malloc(sizeof(PolymorphicObject));
  if (!po) {
    return (PolymorphicObject *)0;
  }

  po->delete = delete;

  if (get_test_a) {
    po->f = test_a;
  } else {
    po->f = test_b;
  }

  return po;
}
