fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let protoc_path = std::path::Path::new(&manifest_dir)
        .parent()
        .unwrap()
        .join("protoc")
        .join("bin")
        .join("protoc.exe");

    unsafe {
        std::env::set_var("PROTOC", protoc_path);
    }

    tonic_build::configure()
        .compile_protos(&["../proto/axon.proto"], &["../proto"])?;
    Ok(())
}
