use eyre::Result;
fn main() -> Result<(), eyre::Report> {
    //convert protos to rust
    // tonic_build::configure()
    //     .build_client(true)
    //     .build_server(false)
    //     //if you don't compile well known types prost-types crate must be added
    //     .compile_well_known_types(true)
    //     .out_dir("src/proto/")
    //     .include_file("mod.rs")
    //     .compile(
    //         &[
    //             "proto/pulumi/resource.proto",
    //             "proto/pulumi/alias.proto",
    //             "proto/pulumi/source.proto",
    //             "proto/pulumi/plugin.proto",
    //             "proto/pulumi/errors.proto",
    //             "proto/pulumi/analyzer.proto",
    //             "proto/pulumi/language.proto",
    //             "proto/pulumi/provider.proto",
    //             "proto/pulumi/converter.proto",
    //             "proto/pulumi/codegen/hcl.proto",
    //             "proto/pulumi/codegen/loader.proto",
    //             "proto/pulumi/codegen/mapper.proto",
    //         ],
    //         //root folder for protos
    //         &["proto"],
    //     )?;

    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .out_dir("src//language")
        .compile_well_known_types(false)
        .include_file("language.rs")
        .compile(
            &["proto/pulumi/language.proto"],
            //root folder for protos
            &["proto"],
        )?;

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir("src/pulumi")
        .include_file("resource.rs")
        .compile_well_known_types(false)
        .compile(
            &["proto/pulumi/resource.proto"],
            //root folder for protos
            &["proto"],
        )?;

    Ok(())
}
