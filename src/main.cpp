#include "rusty_bridge/lib.h"
#include <iostream>

int main() {
  auto myType = new_type();
  std::cout << myType->give_me_42();

  return 0;
}