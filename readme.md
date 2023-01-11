# CPP Rust Bridge

A rust bridge that generates header and cpp files with
the [cxx](https://cxx.rs/)(rust+cpp) and [corrision](https://github.com/corrosion-rs/corrosion) (rust+cmake)


## What is this

In the rust code `rusty_code/src/lib.rs` a class (rust: struct) is defined with a method.
The cxx crate generates the headers and cpp files for the rust code.
The corrosion crate marries those generated files with cmake.

Now you can define a cpp cmake target (main.cpp), that links against the rust cmake
target given from the corrosion crate.

## Prerequisites

Having [Rust](https://www.rust-lang.org/tools/install) installed.

If you are building on windows like me you need the correct toolchain
to compile the rust code.
There just use rustup (shipped with rust)

```
rustup target add --toolchain stable-x86_64-pc-windows-msvc x86_64-pc-windows-gnu
```

## Howto build

```
mkdir build
cd build
cmake --preset default ..
ninja -j<number of threads>
```

or: if you are using CLion like me, just load the top level *CMakeLists.txt*.
with the cmake arguments being *--preset default*.

After that, just built it.