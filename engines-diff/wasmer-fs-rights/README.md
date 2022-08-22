### Wasmer File system Rights
This is an example of using some `fd_*` WASI APIs on a file descriptor that doesn't have the right of these APIs. According to the WASI standard, this program should result in an error. However running it with wasmer (after creating a `scratch_dir` in the program root):

```
wasmer run --dir scratch_dir target/wasm32-wasi/debug/wasmer-fs-rights.wasm
```
will result:
```
written : 10
read : 10
file_content after : [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
```
which means that you can call `fd_read`, `fd_write` and `fd_seek` on any file descriptors regardless of the rights that this file descriptor have.
