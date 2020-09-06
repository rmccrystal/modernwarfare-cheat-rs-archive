fn main() {
    cxx_build::bridge("src/sdk/encryption.rs")
        .file("src/sdk/encryption.cpp")
        .flag_if_supported("-std=c++20")
        .compile("encryption-cpp");

    println!("cargo:rerun-if-changed=src/sdk/encryption.rs");
    println!("cargo:rerun-if-changed=src/sdk/encryption.cpp");
    println!("cargo:rerun-if-changed=src/sdk/encryption.h");
}