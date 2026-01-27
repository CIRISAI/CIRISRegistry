use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "../protocol/ciris_registry.proto";

    // Tell cargo to re-run if proto file changes
    println!("cargo:rerun-if-changed={}", proto_file);

    // Configure tonic-build
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let descriptor_path = out_dir.join("ciris_registry_descriptor.bin");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        // Generate file descriptor set for reflection
        .file_descriptor_set_path(&descriptor_path)
        // Output directory
        .out_dir(&out_dir)
        // Compile the proto file
        .compile_protos(&[proto_file], &["../protocol"])?;

    Ok(())
}
