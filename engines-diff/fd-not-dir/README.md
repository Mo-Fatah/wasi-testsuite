### Fd not Dir
Passing a non-directory fd to `path_open` returns `ERRNO_NOTDIR` with `wasmtime`, and `ERRNO_INVAL` with `wasmer`
