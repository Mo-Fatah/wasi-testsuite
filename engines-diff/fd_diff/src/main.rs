use wasi;

fn main() {
    unsafe {
        match wasi::fd_prestat_get(0) {
            Ok(prestat) => println!("wasmer returns Ok(Prestat) for file descriptors 0, 1 and 2"),

            Err(e) => println!("wasmtime returns wasi::ERRNO_BADF for file descriptors 0, 1 and 2
                               \nError Message: {}", e.message())
        }
    }
}
