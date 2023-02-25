corrosion_import_crate(MANIFEST_PATH rusty_code/Cargo.toml)
corrosion_add_cxxbridge(rusty_lib CRATE rusty_code MANIFEST_PATH rusty_code FILES lib.rs)

add_executable(main_cpp main.cpp)
target_link_libraries(main_cpp PRIVATE rusty_lib)
