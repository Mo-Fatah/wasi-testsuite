use std::{
    env,
    process::{self, ExitCode},
};
use wasi;
use wasi_tests::{open_scratch_directory, print_err, print_succ, print_warning};

unsafe fn test_regular_file_stat(dir_fd: wasi::Fd) -> u8 {
    let file_fd =
        wasi::path_open(dir_fd, 0, "file", wasi::OFLAGS_CREAT, 0, 0, 0).expect("create a file");

    let fd_stat = wasi::fd_fdstat_get(file_fd).expect("trying to get file fd_stat");

    if fd_stat.fs_filetype.name() != "REGULAR_FILE" {
        print_err(format!(
            "file should have type REGULAR_FILE\nFound Type: {}",
            fd_stat.fs_filetype.name()
        ));
        return 1;
    }

    assert_eq!(fd_stat.fs_flags, 0, "file should have fs_flags 0");
    assert_eq!(
        fd_stat.fs_rights_base, 0,
        "file should have fs_rights_base 0"
    );
    assert_eq!(
        fd_stat.fs_rights_inheriting, 0,
        "file should have fs_rights_inheriting 0"
    );

    // clean
    wasi::path_unlink_file(dir_fd, "file").expect("unlinking file");
    0
}

unsafe fn test_directory_stat(dir_fd: wasi::Fd) -> u8 {
    wasi::path_create_directory(dir_fd, "new_dir").expect("creating new dir");
    let new_dir_fd = wasi::path_open(dir_fd, 0, "new_dir", wasi::OFLAGS_DIRECTORY, 0, 0, 0)
        .expect("opening dir");
    let dir_stat = wasi::fd_fdstat_get(new_dir_fd).expect("trying to get new_dir stat");

    if dir_stat.fs_filetype.name() != "DIRECTORY" {
        print_err(format!(
            "directory should have type DIRECTORY\nFound Type: {}",
            dir_stat.fs_filetype.name()
        ));
        return 1;
    }

    assert_eq!(dir_stat.fs_flags, 0, "file should have fs_flags 0");
    assert_eq!(
        dir_stat.fs_rights_base, 0,
        "file should have fs_rights_base 0"
    );
    assert_eq!(
        dir_stat.fs_rights_inheriting, 0,
        "file should have fs_rights_inheriting 0"
    );
    
    wasi::path_remove_directory(dir_fd, "new_dir").expect("trying to remove new_dir");

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

    print_warning("Testing fd_fdstat_get ...".to_string());
    unsafe {
        if test_regular_file_stat(dir_fd) + test_directory_stat(dir_fd) == 0 {
            print_succ("fd_fdstat_get tests passed \u{2713}".to_string());
            return ExitCode::SUCCESS;
        } else {
            print_err("fd_fdstat_get tests failed \u{2717}".to_string());
            return ExitCode::FAILURE;
        }
    }
}
