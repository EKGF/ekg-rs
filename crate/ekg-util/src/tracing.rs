/// Initialize the tracing subscriber in the default way for any AWS Lambda
/// function
pub fn aws_lfn_init() {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        // ANSI colors don't do well with CloudWatch
        .with_ansi(false)
        .init();
}
