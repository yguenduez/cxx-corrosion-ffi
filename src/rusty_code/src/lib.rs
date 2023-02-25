mod fizz_buzz;

use fizz_buzz::FizzBuzz;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type FizzBuzz;
        fn to_fizz_buzz(self: &FizzBuzz, input: u32) -> String;
        fn new_fizz_buzz() -> Box<FizzBuzz>;
    }
}

fn new_fizz_buzz() -> Box<FizzBuzz>{
    Box::new(FizzBuzz)
}