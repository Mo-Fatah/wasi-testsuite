use wasi;

fn main() {
    unsafe {
        // the directory passed to the wasm module
        // wasmtime -> dir_fd = 3
        // wasmer -> dir_fd = 4
        let dir_fd = 4;
        let file_fd = wasi::path_open(dir_fd, 0, "file", wasi::OFLAGS_CREAT, 0, 0, 0).unwrap();

        // pass file_fd to path_open as a directory fd. Should return an error
        if let Err(e) = wasi::path_open(file_fd, 0, "another_file", wasi::OFLAGS_CREAT, 0, 0, 0) {
            if e == wasi::ERRNO_NOTDIR {
                println!("Wasmtime");
            }
            if e == wasi::ERRNO_INVAL {
                println!("Wasmer");
            }
        }
    }
}
