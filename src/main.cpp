#include "rusty_bridge/lib.h"
#include <iostream>

int main() {
  auto myType = new_type();
  std::cout << myType->give_me_42();

  auto mysender = new_modbus_sender();

  const auto myBytes = ::rust::Vec<uint8_t>{};
  mysender->send(myBytes);

  return 0;
}
