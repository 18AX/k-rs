cmake_minimum_required(VERSION 3.22)
enable_language(C ASM)

set(CMAKE_C_COMPILER "gcc")
set(CMAKE_C_FLAGS "-ffreestanding -fno-stack-protector -fno-pic -fpie -mno-80387 -mno-mmx -mno-3dnow -mno-sse -mno-sse2 -mno-red-zone")

set(LINKER_SCRIPT "linker.ld")

set(CMAKE_EXE_LINKER_FLAGS "-nostdlib -T ${CMAKE_SOURCE_DIR}/${LINKER_SCRIPT} -Wl,-static,-no-pie,--no-dynamic-linker,-ztext -static-pie -nostdlib -z max-page-size=0x1000")
set(Rust_CARGO_TARGET "x86_64-unknown-none")

find_package(Corrosion REQUIRED)

if (NOT Rust_IS_NIGHTLY)
    message(FATAL_ERROR "Nightly features are required" )
endif()

corrosion_import_crate(MANIFEST_PATH rust/Cargo.toml NO_STD)

add_executable(krs.elf crt0.S)

target_link_libraries(krs.elf PRIVATE k)