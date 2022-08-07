### Base rights
A file without base/inheriting rights passed to [`path_open`](https://github.com/WebAssembly/WASI/blob/main/phases/snapshot/docs.md#path_open) should return `0` for the rights returned from [`fd_fdstat_get`](https://github.com/WebAssembly/WASI/blob/main/phases/snapshot/docs.md#-fd_fdstat_getfd-fd---resultfdstat-errno)

wasmtime returns 
```
fs_rights_base: 0 
fs_rights_inheriting: 0
```
which as expected
<br />
wasmer returns 
```
fs_rights_base: 520093695
fs_rights_inheriting: 0
```
which is a strange number in `fs_rights_base`
