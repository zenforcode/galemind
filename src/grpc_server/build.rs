fn main() {
    tonic_build::compile_protos("proto/health_check/health_check.proto").unwrap();
}