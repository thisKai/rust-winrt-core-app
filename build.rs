fn main() {
    cpp_build::Config::new()
        .include("src/pch.h")
        .file("src/pch.cpp")
        .include("src/App.h")
        .file("src/App.cpp")
        .flag("-std:c++17")
        .flag("/EHsc")
        .build("src/main.rs");
}
