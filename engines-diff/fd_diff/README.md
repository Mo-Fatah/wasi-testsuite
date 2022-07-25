## File Descriptor Diffs

This program illustrate how `wasmtime` and `wasmer` behave with file descriptors `0`, `1` and `2`passed to (fd_prestat_get)[https://github.com/WebAssembly/WASI/blob/main/phases/snapshot/docs.md#-fd_prestat_getfd-fd---resultprestat-errno]. 

As short summary, `wasmtime` retrun a `Bad file descriptor` error, while `wasmer` accepts them and returns a `Prestat` struct that represents the the corresponding file (stdin for fd=0, stdout for fd=1, stderr for std=2)
