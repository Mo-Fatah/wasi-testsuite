use std::{
    env,
    process::{self, ExitCode},
};

use wasi;
use wasi_tests::{fd_get_rights, open_scratch_directory, print_err, print_succ, print_warning};

/// opening a file that doesn't exist should fail
unsafe fn test_file_doesnt_exist(dir_fd: wasi::Fd) -> u8 {
    match wasi::path_open(dir_fd, 0, "file", 0, 0, 0, 0) {
        Ok(_) => {
            print_err(">> trying to open a file that doesn't exist".to_string());
            return 1;
        },
        Err(e) => {
            if e != wasi::ERRNO_NOENT {
                print_err(format!(">> opening a file that doesn't exist should return ERRNO_NOENT\nFound: ERRNO_{:?}", e.name()));
                return 1;
            }
        }
    }
    0
}

/// simple file creation, should return Ok(Fd)
unsafe fn test_path_create(dir_fd: wasi::Fd) -> u8 {
    if let Err(e) = wasi::path_open(dir_fd, 0, "file", wasi::OFLAGS_CREAT, 0, 0, 0) {
        print_err(format!(
            ">> unexpected error while trying to open a simple file\n{:?}",
            e
        ));
        return 1;
    }

    wasi::path_unlink_file(dir_fd, "file").expect("removing a file");

    0
}

/// trying to create a file that already exists with the OFLAGS_EXCL should Fail
unsafe fn test_path_exists(dir_fd: wasi::Fd) -> u8 {
    wasi::path_open(dir_fd, 0, "file", wasi::OFLAGS_CREAT, 0, 0, 0).expect("creating a file");

    match wasi::path_open(
        dir_fd,
        0,
        "file",
        wasi::OFLAGS_CREAT | wasi::OFLAGS_EXCL,
        0,
        0,
        0,
    ) {
        Ok(_) => {
            print_err(">> creating a file that already exists with OFLAGS_EXCL should return Err(ERRNO_EXIST):\n\
            Expected: Err(ERRNO_EXIST)\nFound: Ok(Fd)".to_string());
            return 1;
        }
        Err(e) => {
            if e != wasi::ERRNO_EXIST {
                print_err(format!(
                    ">> creating a file that already exists with OFLAGS_EXCL should return ERRNO_EXIST:\n\
                    Expected: ERRNO_EXIST\nFound: ERRNO_{} ",
                    e.name()
                ));
                return 1;
            }
        }
    }

    wasi::path_unlink_file(dir_fd, "file").expect("removing a file");

    0
}

/// passing a non-directory fd should fail.
unsafe fn test_path_fd_notdir(dir_fd: wasi::Fd) -> u8 {
    let file_fd =
        wasi::path_open(dir_fd, 0, "file", wasi::OFLAGS_CREAT, 0, 0, 0).expect("creating a file");

    match wasi::path_open(file_fd, 0, "another_file", wasi::OFLAGS_CREAT, 0, 0, 0) {
        Ok(_) => {
            print_err(
                ">> non-directory fd should return Err(ERRNO_NOTDIR):\n\
                Expected: Err(ERRNO_NOTDIR)\nFound: Ok(Fd)"
                    .to_string(),
            );
            return 1;
        }
        Err(e) => {
            if e != wasi::ERRNO_NOTDIR {
                print_err(format!(
                    ">> non-directory fd should return ERRNO_NOTDIR:\n\
                                Expected: ERRNO_NOTDIR\nFound: ERRNO_{}",
                    e.name()
                ));
                return 1;
            }
        }
    }

    wasi::path_unlink_file(dir_fd, "file").expect("removing a file");

    0
}

/// test OFLAGS_DIRECTORY: Fail if not a directory
unsafe fn test_path_directory(dir_fd: wasi::Fd) -> u8 {
    wasi::path_create_directory(dir_fd, "inner_dir").expect("creating a directory");
    wasi::path_open(dir_fd, 0, "file", wasi::OFLAGS_CREAT, 0, 0, 0).expect("creating a file");

    // try with a file
    match wasi::path_open(dir_fd, 0, "file", wasi::OFLAGS_DIRECTORY, 0, 0, 0) {
        Ok(_) => {
            print_err(
                "path_open with OFLAGS_DIRECTORY should fail if `path` is not a directory:\n\
                Expected: Err(ERRNO_NOTDIR)\nFound: Ok(Fd)"
                    .to_string(),
            );
            return 1;
        }
        Err(e) => {
            if e != wasi::ERRNO_NOTDIR {
                print_err(format!(
                    "path_open with OFLAGS_DIRECTORY should fail if `path` is not a directory:\n\
                    Expected: ERRNO_NOTDIR\nFound: ERRNO_{}",
                    e.name()
                ));
                return 1;
            }
        }
    }

    //try with a directory
    if let Err(e) = wasi::path_open(dir_fd, 0, "inner_dir", wasi::OFLAGS_DIRECTORY, 0, 0, 0) {
        print_err(format!(
            "path_open with OFLAGS_DIRECTORY and directory path should return Ok:\
                  \nExpected: Ok(fd)\nFound: Err( {:?} )",
            e
        ));
        return 1;
    }

    wasi::path_remove_directory(dir_fd, "inner_dir").expect("removing a directory");
    wasi::path_unlink_file(dir_fd, "file").expect("removing a file");

    0
}
/// test only an fd with fd_read rights can invoke fd_rights
unsafe fn test_fdread_rights(dir_fd: wasi::Fd) -> u8 {
    // case 1: an fd without fd_read rights should fail in invoking fd_read
    let no_fdread =
        wasi::path_open(dir_fd, 0, "file", wasi::OFLAGS_CREAT, 0, 0, 0).expect("creating a file");
    let (rbase, rinher) = fd_get_rights(no_fdread);

    // checking that the fd has no base fd_read rights
    if rbase & wasi::RIGHTS_FD_READ != 0 {
        print_err(">> fd should not have base RIGHTS_FD_READ".to_string());
        return 1;
    }
    // checking that the fd has no inheriting fd_read rights
    if rinher & wasi::RIGHTS_FD_READ != 0 {
        print_err(">> fd should not have inheriting RIGHTS_FD_READ".to_string());
        return 1;
    }

    let content = &mut [0u8; 1];
    let iovec = wasi::Iovec {
        buf: content.as_mut_ptr(),
        buf_len: content.len(),
    };

    // invoking fd_read should fail
    match wasi::fd_read(no_fdread, &[iovec]) {
        Ok(_) => {
            print_err(">> reading from a file without RIGHTS_FD_READ should fail".to_string());
            return 1;
        }
        Err(e) => {
            if e != wasi::ERRNO_BADF {
                print_err(format!(
                    ">> reading from a file without RIGHTS_FD_READ should return ERRNO_BADF\n\
                          Expected: ERRNO_BADF\nFound: ERRNO_{}",
                    e.name()
                ));
                return 1;
            }
        }
    }

    // case 2: an fd with fd_read rights should be able to invoke fd_read

    let ok_fdread = wasi::path_open(
        dir_fd,
        0,
        "file",
        wasi::OFLAGS_CREAT,
        wasi::RIGHTS_FD_READ,
        0,
        0,
    )
    .expect("creating a file");

    // invoking fd_read should succeed
    if let Err(e) = wasi::fd_read(ok_fdread, &[iovec]) {
        print_err(format!(
            ">> reading from a file with RIGHTS_FD_READ should succeed\nFound Error:{:?}",
            e
        ));
        return 1;
    }

    wasi::path_unlink_file(dir_fd, "file").expect("removing a file");

    0
}

/// Expects a Directory passed as an env variable
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

    print_warning("Testing path_open ...".to_string());

    unsafe {
        if test_file_doesnt_exist(dir_fd) 
            + test_path_create(dir_fd)
            + test_path_exists(dir_fd)
            + test_path_fd_notdir(dir_fd)
            + test_path_directory(dir_fd)
            + test_fdread_rights(dir_fd)
            == 0
        {
            print_succ("path_open tests passed \u{2713}".to_string());
            return ExitCode::SUCCESS;
        } else {
            print_err("path_open tests failed \u{2717}".to_string());
            return ExitCode::FAILURE;
        }
    }
}
