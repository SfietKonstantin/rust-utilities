#include <iostream>

#include <demo-rust-lib.h>

int main() {
  auto *opaque = service_new();
  std::cout << "Value from service " << service_get_value(opaque) << std::endl;
  service_delete(opaque);
  return 0;
}
