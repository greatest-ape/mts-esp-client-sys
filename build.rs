fn main() {
    cc::Build::new()
        .cpp(true)
        .file("include/libMTSClient.hpp")
        .file("include/libMTSClient.cpp")
        .compile("libMTSClient");
}
