# xcheck

`cargo xtask` shows "stable" instead of the expected "nightly" from the rust-toolchain.toml file
in the xcheck-bpf subdirectory.

A change like in https://github.com/aya-rs/aya-template/pull/79 (use `rustup run nightly cargo -V`)
works around the issue.

```
Run 'cargo -V' in the xcheck-ebpf subdirectory:
Current dir: "\\\\?\\C:\\Users\\dmitris\\dev\\tmp\\xcheck\\xcheck-ebpf"
status: exit code: 0
cargo 1.67.0 (8ecd4f20a 2023-01-10)
```
