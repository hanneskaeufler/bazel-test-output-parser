load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "fa331eab172660423f97519434ea43f102adfb4e71e482638843d1d7b149930d",
    strip_prefix = "rules_rust-78d702b0d578d8a3818fbce234a9f9ef7d4b1311",
    urls = [
        "https://github.com/bazelbuild/rules_rust/archive/78d702b0d578d8a3818fbce234a9f9ef7d4b1311.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains()

load("@rules_rust//tools/rust_analyzer:deps.bzl", "rust_analyzer_deps")

rust_analyzer_deps()
