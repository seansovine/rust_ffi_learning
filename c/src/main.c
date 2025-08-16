#include "test.h"

#include <stddef.h>

int main() {
  PolymorphicObject *po = get_test(true);
  po->f();
  po->delete (po);
  po = NULL;

  po = get_test(false);
  po->f();
  po->delete (po);
  po = NULL;

  return 0;
}
