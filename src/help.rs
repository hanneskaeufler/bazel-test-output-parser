pub fn print_help() {
    println!(
        "\
This is a stupid tiny parser for the output
produced by bazel's (https://bazel.build) test runner.
When running e.g. `bazel test //...`, a log file will
pre written and printed to stdout, which can be parsed
by this program to get a list of junit test results that
bazel produced.

Usage example:
    cat my.log | {}",
        env!("CARGO_PKG_NAME")
    );
}
