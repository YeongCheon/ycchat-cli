fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(
            &[
                "protobuf/model/server.proto",
                "protobuf/auth/auth.proto",
                "protobuf/account/account.proto",
                "protobuf/user/user.proto",
                "protobuf/server/server.proto",
                "protobuf/server/category/category.proto",
                "protobuf/server/member/server_member.proto",
            ],
            &["protobuf/"],
        )?;
    Ok(())
}
