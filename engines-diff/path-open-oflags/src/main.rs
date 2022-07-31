use wasi;

fn main() {

    // For simplicity, the file descriptor will be passed manually here.
    // This fd refers to the file descriptor of the directory passed to the wasm engine. 
    // Since this program is tested against wasmtime and wasmer, you should change fd to 3
    // with wasmtime, and 4 with wasmer
    let dir_fd = 3;
    
    unsafe {
        let result = wasi::path_open(
            dir_fd,
            0,
            "file",
            wasi::OFLAGS_EXCL,
            0,
            0,
            0
        );

        println!("\nresult: {:?}\n", result);

        assert_eq!( 
            result,
            Err(wasi::ERRNO_EXIST)
        );
    }


}
