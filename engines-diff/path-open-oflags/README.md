## Path Open Oflags

This simple piece of code uses the `path_open` function with the parameter `oflags` equals to `wasi::OFLAGS_EXCL`, which supposedly should return an error if the `path` already exists. The program tries to open the already existing `test_dir/file` and tests that the result is `ERRNO_EXIST`.

> **_NOTE:_** Change the `fd` variable in line 9 to `3` when using `wasmtime`, and to `4` when using `wasmer`

- Compile with `cargo build --target wasm32-wasi`
- Run with `wasmer` by `wasmer run --dir test_dir target/wasm32-wasi/debug/path-open-oflags.wasm test_dir`
- Run with `wasmtime` by `wasmtime --dir test_dir target/wasm32-wasi/debug/path-open-oflags.wasm test_dir`

With `wasmtime`, the test will fail since `wasmtime` returns `Ok()`, which is (I suppose) not a correct behavior.

