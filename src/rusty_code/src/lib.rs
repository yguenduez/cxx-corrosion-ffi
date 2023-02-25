mod fizz_buzz;

use fizz_buzz::FizzBuzz;

#[cxx::bridge]
mod ffi {
    // Shared structs: fields are visible to both languages
    // One source of truth
    pub struct DataBlob {
        pub total_size: usize,
        mini_blobs: Vec<u8>,
    }

    // Opaque types: fields are secret to other language.
    // Cannot be passed as value!
    extern "Rust" {
        type FizzBuzz;
        fn to_fizz_buzz(self: &FizzBuzz, input: u32) -> String;
    }

    // Functions: implemented in either language - callable from the other
    extern "Rust" {
        fn new_fizz_buzz() -> Box<FizzBuzz>;
    }
}

// use the shared DataBlob struct from the ffi module
use ffi::DataBlob;
fn function_taking_a_blob(datablob: &DataBlob) -> usize {
    datablob.total_size
}

fn new_fizz_buzz() -> Box<FizzBuzz> {
    Box::new(FizzBuzz)
}
