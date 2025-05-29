fn main() {
    tonic_build::compile_protos("proto/prediction/prediction.proto").unwrap();
}
