fn main() {
    tonic_build::compile_protos("../proto/model.proto").unwrap();
}