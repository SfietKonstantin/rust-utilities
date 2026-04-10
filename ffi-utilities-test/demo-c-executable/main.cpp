#include <iostream>

#include <demo-rust-lib.h>

int main() {
  auto *service = service_new();
  service_set_number(service, 42);
  service_set_string(service, "Hello");

  std::cout << "Number from service: " << service_get_number(service)
            << std::endl;
  std::cout << "String from service: " << service_get_string(service)
            << std::endl;
  service_print_string(service);

  service_delete(service);
  return 0;
}
