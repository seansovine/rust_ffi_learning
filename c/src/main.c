#include "test.h"

int main() {
  PolymorphicObject *po = get_test(true);
  po->f();
  po->delete (po);

  po = get_test(false);
  po->f();
  po->delete (po);

  return 0;
}
