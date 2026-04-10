#include <iostream>

#include <demo-rust-lib.h>

void test_service() {
  auto *service = service_new();
  service_set_number(service, 42);
  service_set_string(service, "Hello");

  std::cout << "Number from service: " << service_get_number(service)
            << std::endl;
  std::cout << "String from service: " << service_get_string(service)
            << std::endl;
  service_print_string(service);

  service_delete(service);
}

void test_slice() {
  auto slice = string_slice_of_two_new("test1", "test2");
  std::cout << "Slice of size: " << slice.size << std::endl;
  for (auto i = 0; i < slice.size; ++i) {
    const auto *item = *(slice.items + i);
    std::cout << "Slice at " << i << ": " << item << std::endl;
  }
  string_slice_delete(slice);
}

int main() {
  test_service();
  test_slice();
  return 0;
}
