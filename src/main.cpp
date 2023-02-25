#include "rusty_lib/lib.h"

#include <iostream>
#include <string>

int main() {
  // Opaque type
  auto fizzbuzzer = new_fizz_buzz();
  const auto result = fizzbuzzer->to_fizz_buzz(45);
  std::cout << std::string{result} << std::endl;

  // Shared struct defined in ffi module
  DataBlob blob{};
  blob.mini_blobs.push_back(0x01);
  blob.mini_blobs.push_back(0x02);
  for (auto const& miniBlob: blob.mini_blobs){
    std::cout << std::to_string(miniBlob) << std::endl;
  }

  return 0;
}
