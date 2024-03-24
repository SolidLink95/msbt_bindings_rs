fn main() {
    // Add the `cc` crate as a dependency
    // by adding `cc = "1.0"` to your `Cargo.toml` file.
    // cc::Build::new()
    //     .cpp(true) // Enable C++ compilation
    //     .file("msbt_bindings.cpp")
    //     // .file("./repos/msbt/src/msbt.cpp")
    //     // .file("./repos/msbt/src/tags.cpp")
    //     .include("./repos/msbt/src/include")
    //     .include("./repos/msbt/src/include")
    //     .include("include")
    //     .compile("msbt"); 
    println!("cargo:rustc-link-search=native=lib");
println!("cargo:rustc-link-lib=static=msbt");

}
