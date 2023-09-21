pub mod codegen {
    include!("codegen.rs");
}
pub mod google {
    pub mod protobuf {
        include!("google.protobuf.rs");
    }
}
pub mod pulumirpc {
    pub mod codegen {
        include!("pulumirpc.codegen.rs");
    }
    include!("pulumirpc.rs");
}
