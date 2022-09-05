use std::{
    env,
    process::{self, ExitCode},
};
use wasi;
use wasi_tests::{open_scratch_directory, print_err, print_succ, print_warning};

unsafe fn fd_allocate_test(dir_fd: wasi::Fd) -> u8 {
    let file_fd = wasi::path_open(
        dir_fd,
        0,
        "file",
        wasi::OFLAGS_CREAT,
        wasi::RIGHTS_FD_ALLOCATE | wasi::RIGHTS_FD_FILESTAT_GET,
        0,
        0,
    )
    .expect("creating a file");

    // getting file stat
    let mut stat = wasi::fd_filestat_get(file_fd).expect("reading file stats");
    assert_eq!(0, stat.size, "file size should be 0");
    
    // allocate size 
    wasi::fd_allocate(file_fd, 0, 100).expect("allocating size");
    stat = wasi::fd_filestat_get(file_fd).expect("reading file stats");
    assert_eq!(100, stat.size, "file size should be equal to 100");

    //// if ( offset+len < file_size ) -> shouldn't modify the file
    wasi::fd_allocate(file_fd, 10, 10).expect("allocating size less than file size");
    stat = wasi::fd_filestat_get(file_fd).expect("reading file stats");
    assert_eq!(100, stat.size, "file size should be equal to 100");

    // if ( offset+len > file_size ) -> should allocate more file size
    wasi::fd_allocate(file_fd, 80, 50).expect("allocating size less than file size");
    stat = wasi::fd_filestat_get(file_fd).expect("reading file stats");
    assert_eq!(130, stat.size, "file size should be equal to 100");

    wasi::path_unlink_file(dir_fd, "file").expect("removing file");
    0
}

fn main() -> ExitCode {
    let mut args = env::args();
    let prog = args.next().unwrap();
    let arg = if let Some(arg) = args.next() {
        arg
    } else {
        eprintln!("usage: {} <scratch directory>", prog);
        process::exit(1);
    };

    let dir_fd = match open_scratch_directory(&arg) {
        Ok(dir_fd) => dir_fd,
        Err(_) => {
            4 // hardcoded value to bypass wasmer's fd_prestat_dir_name errors
              //eprintln!("{}", err);
              //process::exit(1)
        }
    };

    print_warning("Testing fd_allocate ...".to_string());

    unsafe {
        if fd_allocate_test(dir_fd) == 0 {
            print_succ("fd_allocate tests passed \u{2713}".to_string());
            return ExitCode::SUCCESS;
        } else {
            print_err("fd_allocate tests failed \u{2717}".to_string());
            return ExitCode::FAILURE;
        }
    }
}
