# bazel-test-output-parser

This solely exists to work around https://github.com/bazelbuild/bazel/issues/14648.
In an ideal world I would've implemented a change in bazel itself to fix the issue,
but then again the bazel source is tough to dive into and I wanted to learn Rust
anyway and be able to cross compile from macOS to windows and ubuntu linux.

## Building

For host:

`cargo build --release`

For x64 linux on an aarch64 macOS host:

`./compile-to-linux`

Note that for this to work, you need to modify `~/.cargo/config.toml`

```
[target.x86_64-unknown-linux-gnu]
linker = "/path/to/zig-wrapper"
```
