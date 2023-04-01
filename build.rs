fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
                &["proto/torresix/torresix.proto"],
                &["proto"]
        )
        .unwrap();
    Ok(())
}