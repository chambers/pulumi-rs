use eyre::Result;
fn main() -> Result<(), eyre::Report> {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile_well_known_types(false)
        .out_dir("src/proto/pulumi/codegen")
        .compile(
            &[
                "proto/pulumi/codegen/hcl.proto",
                "proto/pulumi/codegen/loader.proto",
                "proto/pulumi/codegen/mapper.proto",
            ],
            &["proto"],
        )?;

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile_well_known_types(false)
        .out_dir("src/proto/pulumi")
        .compile(
            &[
                "proto/pulumi/resource.proto",
                "proto/pulumi/alias.proto",
                "proto/pulumi/source.proto",
                "proto/pulumi/alias.proto",
                "proto/pulumi/plugin.proto",
                "proto/pulumi/errors.proto",
                "proto/pulumi/analyzer.proto",
                "proto/pulumi/language.proto",
                "proto/pulumi/provider.proto",
                "proto/pulumi/converter.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}
