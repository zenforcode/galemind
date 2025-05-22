fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_files(
        &["proto/health_check"],
    )?;
    Ok(())
}