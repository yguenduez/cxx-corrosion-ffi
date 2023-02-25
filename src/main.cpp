#include "rusty_lib/lib.h"

#include <iostream>
#include <string>

int main() {
  auto fizzbuzzer = new_fizz_buzz();
  const auto result = fizzbuzzer->to_fizz_buzz(45);

  std::cout << std::string{result} << std::endl;

  return 0;
}
