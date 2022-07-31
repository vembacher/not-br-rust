# This is probably not the best way to do this.
cmake_minimum_required(VERSION 3.10)

project(notbr-example)



add_custom_target(
        libnotbr
        COMMAND cargo build --release
)
include_directories(${CMAKE_SOURCE_DIR}/../../not-br-ffi/include)
link_directories("${CMAKE_SOURCE_DIR}/../../target/release")
add_executable(simple-example simple-example.c ..)
target_link_libraries(simple-example notbr)
add_dependencies(simple-example libnotbr)
