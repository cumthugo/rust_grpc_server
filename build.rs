fn main() {
    tonic_build::compile_protos("proto/h625.proto")
        .expect("Failed to compile proto files");
}