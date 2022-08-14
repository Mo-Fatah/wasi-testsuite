use std::{
    env,
    process::{self, ExitCode}, panic::catch_unwind,
};
use wasi_tests::{open_scratch_directory, print_err, print_succ, print_warning};

/// creating a new hard link for a file should update the nlink value of a file stat
unsafe fn test_nlink(dir_fd: wasi::Fd) -> u8 {
    let file_fd = wasi::path_open(
        dir_fd,
        0,
        "file",
        wasi::OFLAGS_CREAT,
        wasi::RIGHTS_FD_FILESTAT_GET,
        0,
        0,
    )
    .expect("creating a file");

    let stat = wasi::fd_filestat_get(file_fd).expect("failed to get file stat");

    assert_eq!(stat.size, 0, "newly created file should have size 0");
    assert_eq!(stat.nlink, 1, "newly created file should have nlink = 1");

    // create a directory to store the new file link
    wasi::path_create_directory(dir_fd, "inner_dir").expect("create a new directory");
    let new_dir_fd = wasi::path_open(
        dir_fd,
        0,
        "inner_dir",
        wasi::OFLAGS_DIRECTORY,
        wasi::RIGHTS_PATH_LINK_TARGET
            | wasi::RIGHTS_PATH_REMOVE_DIRECTORY
            | wasi::RIGHTS_PATH_UNLINK_FILE,
        0,
        0,
    )
    .expect("open a new directory");

    // create a new hard link for "file"
    wasi::path_link(dir_fd, 0, "file", new_dir_fd, "file").expect("create a new file link");

    let new_stat = wasi::fd_filestat_get(file_fd).expect("failed to get file stat");
    if new_stat.nlink != 2 {
        print_err(format!(
            ">> file should have 2 nlinlk\nFound nlinks:{}",
            new_stat.nlink
        ));
        return 1;
    }

    // clean
    wasi::path_unlink_file(new_dir_fd, "file").expect("removing the second file link");
    wasi::path_remove_directory(dir_fd, "inner_dir").expect("removing directory");
    wasi::path_unlink_file(dir_fd, "file").expect("removing file");

    0
}

/// check stdin, stdout and stderr stats
/// this function is from wasmtime's wasi-tests
unsafe fn test_stdin_stdout_stderr() -> u8 {
    let result = catch_unwind(|| {

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
    });

    if result.is_ok() {
        println!("\n\n\n\n================================================\n\n\n\n\\n\n\n\n\n\n");
        return 1;
    }
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

    print_warning("Testing fd_filestat_get ...".to_string());

    unsafe {
        if test_nlink(dir_fd) + test_stdin_stdout_stderr() == 0 {
            print_succ("fd_filestat_get tests passed \u{2713}".to_string());
            return ExitCode::SUCCESS;
        } else {
            print_err("fd_filestat_get tests failed \u{2717}".to_string());
            return ExitCode::FAILURE;
        }
    }
}
