fn main() {
    cpp_build::Config::new()
        .include("src/pch.h")
        .file("src/pch.cpp")
        .include("src/ffi.h")
        .file("src/ffi.cpp")
        .include("src/App.h")
        .file("src/App.cpp")
        .flag("-std:c++17")
        .flag("/EHsc")
        .build("src/lib.rs");
}
