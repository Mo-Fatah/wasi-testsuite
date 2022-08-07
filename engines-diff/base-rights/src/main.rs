use wasi;

fn main() {
    unsafe {
        // wasmtime -> dir_fd = 3
        // wasmer -> dir_fd = 4
        let dir_fd = 4;
        let file_fd = wasi::path_open(dir_fd, 0, "file", wasi::OFLAGS_CREAT, 0, 0, 0).expect("creating a file");

        let fd_stat = wasi::fd_fdstat_get(file_fd).expect("failed to get fd stat");

        println!("Both rights-base and rights-inheriting should be equal to 0");

        println!("fs_rights_base: {:?}\nfs_rights_inheriting: {:?}", fd_stat.fs_rights_base, fd_stat.fs_rights_inheriting);
    }

}
