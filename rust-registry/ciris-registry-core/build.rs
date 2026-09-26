use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Phase 4 of #33 — proto path updated to reflect the workspace split.
    // ciris-registry-core lives at rust-registry/ciris-registry-core/,
    // so the protocol/ directory at the repo root is reached via "../../".
    let proto_file = "../../protocol/ciris_registry.proto";

    // Tell cargo to re-run if proto file changes
    println!("cargo:rerun-if-changed={}", proto_file);

    // Configure tonic-build
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let descriptor_path = out_dir.join("ciris_registry_descriptor.bin");

    // The gRPC service stubs exist only in the `standalone` build; the fold
    // build compiles the proto messages as plain prost types (no tonic).
    let standalone = std::env::var_os("CARGO_FEATURE_STANDALONE").is_some();

    tonic_build::configure()
        .build_server(standalone)
        .build_client(standalone)
        // Generate file descriptor set for reflection
        .file_descriptor_set_path(&descriptor_path)
        // Output directory
        .out_dir(&out_dir)
        // Compile the proto file
        .compile_protos(&[proto_file], &["../../protocol"])?;

    Ok(())
}
