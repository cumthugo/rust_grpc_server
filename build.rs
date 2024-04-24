fn main() {
    tonic_build::compile_protos("proto/hello.proto")
        .expect("Failed to compile proto files");
}