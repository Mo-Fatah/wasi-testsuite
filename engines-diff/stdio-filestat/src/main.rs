use wasi;

fn main() {
    unsafe {
        let stat = wasi::fd_filestat_get(wasi::FD_STDIN).expect("failed filestat 0");
        assert_eq!(stat.size, 0, "stdio size should be 0");
        assert_eq!(stat.atim, 0, "stdio atim should be 0");
        assert_eq!(stat.mtim, 0, "stdio mtim should be 0");
        assert_eq!(stat.ctim, 0, "stdio ctim should be 0");

        let stat = wasi::fd_filestat_get(wasi::FD_STDOUT).expect("failed filestat 1");
        assert_eq!(stat.size, 0, "stdio size should be 0");
        assert_eq!(stat.atim, 0, "stdio atim should be 0");
        assert_eq!(stat.mtim, 0, "stdio mtim should be 0");
        assert_eq!(stat.ctim, 0, "stdio ctim should be 0");

        let stat = wasi::fd_filestat_get(wasi::FD_STDERR).expect("failed filestat 2");
        assert_eq!(stat.size, 0, "stdio size should be 0");
        assert_eq!(stat.atim, 0, "stdio atim should be 0");
        assert_eq!(stat.mtim, 0, "stdio mtim should be 0");
        assert_eq!(stat.ctim, 0, "stdio ctim should be 0");
    }
}
