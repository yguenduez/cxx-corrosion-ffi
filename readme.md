# CPP Rust Bridge

A rust bridge that generates header and cpp files from rust code with
the [cxx](https://cxx.rs/) (rust+cpp) and [corrision](https://github.com/corrosion-rs/corrosion) (rust+cmake).


## About

The rust code `rusty_code/src/lib.rs` contains a class (rust: struct) with a method.
The cxx crate generates the headers and cpp files for the rust code.
The corrosion crate marries those generated files with cmake.

Now you can define a cpp executable (main.cpp), that links against the rust cmake
lib target generated from the corrosion crate.

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

## Tested on

- Windows 10
  - with MinGW