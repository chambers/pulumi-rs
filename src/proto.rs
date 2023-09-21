pub mod codegen {
    include! "proto/codegen.rs"}
}
pub mod google {
    include! {"proto/google.protobuf.rs"}
}
pub mod pulumi {
    pub mod codegen {
        include! {"proto/pulumirpc.codegen.rs"}
    }
    include! {"proto/pulumirpc.rs"}
}
