fn main() {
    // Generate Rust bindings for C++.
    let mut config = cbindgen::Config::default();
    config.language = cbindgen::Language::Cxx;
    config.pragma_once = true;
    cbindgen::Builder::new()
        .with_config(config)
        .with_crate("./")
        .generate()
        .expect("failed to generate rust bindings for c++")
        .write_to_file("./regex/extension/rust.h");

    // Compile C++ code.
    cc::Build::new()
        .include("./regex/contrib")
        .include("./regex/extension")
        .file("./regex/entry.cpp")
        .file("./regex/extension/utils.cpp")
        .file("./regex/extension/meta.cpp")
        .file("./regex/extension/regex.cpp")
        .cpp(true)
        .std("c++14")
        .flag_if_supported("/utf-8")
        .compile("rusqlite_regex");
    println!("cargo:rerun-if-changed=./regex/entry.cpp");
    println!("cargo:rerun-if-changed=./regex/extension/utils.cpp");
    println!("cargo:rerun-if-changed=./regex/extension/meta.cpp");
    println!("cargo:rerun-if-changed=./regex/extension/regex.cpp");
}
