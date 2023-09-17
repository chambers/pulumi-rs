//use std::path::PathBuf;

#[path = "proto/"]
pub mod proto {
    #[path = "proto/pulumi"]
    pub mod pulumi {
        include!("proto/pulumi/pulumirpc.rs");
    }
}
fn main() {
    let error_cause = proto::pulumi::ErrorCause {
        message: String::from("This is an error message"),
        stack_trace: String::from("This is a stack trace"),
    };

    let message = error_cause.message;
    println!("Error message: {}", message);
}
