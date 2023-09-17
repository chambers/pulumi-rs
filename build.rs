fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/proto/pulumi")
        //arg1=protos to compile, arg2=root folder of protos
        .compile(&["proto/pulumi/errors.proto"], &["proto"])?;

    Ok(())
}
