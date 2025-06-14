#include "test.h"

#include <stdio.h>
#include <stdlib.h>

void test_a() { printf("Test A running!\n"); }

void test_b() { printf("Test B running!\n"); }

void delete(PolymorphicObject *po) { free(po); }

PolymorphicObject *get_test(bool get_test_a) {
  PolymorphicObject *po = malloc(sizeof(PolymorphicObject));
  po->delete = delete;

  if (get_test_a) {
    po->f = test_a;
  } else {
    po->f = test_b;
  }

  return po;
}
