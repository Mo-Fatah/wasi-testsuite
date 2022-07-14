## End of Path
This simple program illustrate the difference between `wasmer` and `wasmtime` engines in directory pathes when using [fd_prestat_dir_name](https://docs.rs/wasi/0.11.0+wasi-snapshot-preview1/wasi/fn.fd_prestat_dir_name.html) and [fd_prestat_get](https://docs.rs/wasi/0.11.0+wasi-snapshot-preview1/wasi/fn.fd_prestat_get.html) from the [wasi](https://crates.io/crates/wasi) crate.

## Run the program
After compiling to `wasm32-wasi`, make a directory with a random name, let's call it `scratch_dir`.

Run the program with `wasmtime` by:
```
wasmtime --dir scratch_dir ./target/wasm32-wasi/debug/end-of-path.wasm scratch_dir
```

Run the program with `wasmer` by:
```
wasmer run --dir scratch_dir ./target/wasm32-wasi/debug/end-of-path.wasm scratch_dir
```
