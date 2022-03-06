load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test", "rustfmt_test")

rust_binary(
    name = "main",
    srcs = ["main.rs"],
)

rustfmt_test(
    name = "format_tests",
    targets = [":main"],
)
