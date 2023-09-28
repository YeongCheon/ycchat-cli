fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(
            &[
                "protobuf/model/server.proto",
                "protobuf/auth/auth.proto",
                "protobuf/server/server.proto",
            ],
            &["protobuf/"],
        )?;
    Ok(())
}
