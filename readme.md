# Cxx Corrosion FFI Example

A rust bridge that generates headers and linkable binaries from rust code with
the [cxx](https://cxx.rs/) (rust+cpp) crate.
[Corrision](https://github.com/corrosion-rs/corrosion) (rust+cmake) enables you to define [CMake](https://cmake.org/) 
targets from the rust part.


## About

The rust code `rusty_code/src/lib.rs` contains a module with a class (rust: struct) with methods.
The cxx crate generates the headers for the rust code, that are defined within the 
```
mod ffi {
...
}
```
module. Outside of that, you can write idiomatic Rust code without to use any `unsafe` keywords. 
The corrosion crate exports cmake targets for your rust library, that is generated with cxx a priori, like so:

```cmake
corrosion_add_cxxbridge(rusty_lib CRATE rusty_code MANIFEST_PATH rusty_code FILES lib.rs)
```

where `rusty_lib` is the name of your rust library target.

In the end you only have to link against that cmake target (static or shared lib) coming from rust, 
e.g.(`main_cpp` being your C++ executable):

```cmake
add_executable(main_cpp main.cpp)
target_link_libraries(main_cpp PRIVATE rusty_bridge)
```

There is a cpp executable (main.cpp), that links against the rust cmake
lib target generated from the above process.

## Prerequisites

- Having [Rust](https://www.rust-lang.org/tools/install) installed.
- Having Cxx-Bridge CLI installed (for cxx builds without cargo - we build it with cmake!)
  ```shell
  cargo install cxxbridge-cmd
  ```

### On Windows

If you are building on windows you need the correct toolchain
to compile the rust code with the minGW compiler in CLion.
There just use rustup (shipped with rust)

```
rustup target add --toolchain stable-x86_64-pc-windows-msvc x86_64-pc-windows-gnu
```

## Howto build

Manually:

```
cmake --preset default
cmake --build --preset default
```

IDE (CLion):  
  - if you are using CLion like me, just load the top level `CMakeLists.txt`.
  - After that, just built it.


## Tested on

- Linux:
  - Manjaro (arch linux, built manually): `Linux 5.13.19-2-MANJARO x86_64`
- Windows 10
  - with MinGW (CLion bundled)

## Originated From

[trondhe/rusty_cmake](https://github.com/trondhe/rusty_cmake)