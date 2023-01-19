mod fizz_buzz;

use fizz_buzz::FizzBuzz;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type MyRustType;
        fn give_me_42(self: &MyRustType) -> usize;
        fn new_type() -> Box<MyRustType>;

        type FizzBuzz;
        fn to_fizz_buzz(self: &FizzBuzz, input: u32) -> String;
        fn new_fizz_buzz() -> Box<FizzBuzz>;
    }
}

fn new_type() -> Box<MyRustType> {
    Box::new(MyRustType)
}

fn new_fizz_buzz() -> Box<FizzBuzz>{
    Box::new(FizzBuzz)
}

struct MyRustType;

impl MyRustType {
    fn give_me_42(&self) -> usize {
        42
    }
}
