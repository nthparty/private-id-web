fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/common.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
    tonic_build::compile_protos("proto/privateid.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
    Ok(())
}
