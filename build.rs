fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/carbon.proto")?;
    tonic_build::configure()
        .build_server(false)
        .compile(&["proto/prometheus/remote.proto"], &["proto/prometheus"])?;
    Ok(())
}
