fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/proto")
        .compile(&["../proto/ml_service.proto"], &["../proto"])?;
    
    println!("cargo:rerun-if-changed=../proto/ml_service.proto");
    println!("cargo:rerun-if-changed=build.rs");
    
    Ok(())
}
