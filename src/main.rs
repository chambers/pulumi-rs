//use std::path::PathBuf;

mod proto {

    use tonic::*;
    //"pulumirpc" refers to the 'package' name inside source .proto file"
    include_proto!("pulumirpc");
}

fn main() {
    let error_cause = proto::ErrorCause {
        message: String::from("This is an error message"),
        stack_trace: String::from("This is a stack trace"),
    };

    let message = error_cause.message;
    println!("Error message: {}", message);
}
