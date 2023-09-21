mod proto;
use proto::pulumirpc::ErrorCause;

fn main() {
    let error_cause = ErrorCause {
        message: String::from("This is an error message"),
        stack_trace: String::from("This is a stack trace"),
    };

    let message = error_cause.message;
    println!("Yay! {:#?}", &message);
}
