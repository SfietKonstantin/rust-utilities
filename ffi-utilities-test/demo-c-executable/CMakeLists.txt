cmake_minimum_required(VERSION 3.22)

project(ffi-utilities-demo-c-executable)

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.6
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH ../demo-rust-lib/Cargo.toml)

add_executable(${PROJECT_NAME} main.cpp)
target_include_directories(${PROJECT_NAME} PUBLIC ../demo-rust-lib/)
target_link_libraries(${PROJECT_NAME} PUBLIC ffi_utilities_demo_rust_lib)
