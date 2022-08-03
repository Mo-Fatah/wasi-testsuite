use std::{
    env,
    process::{self, ExitCode},
};

use wasi;
use wasi_tests::{open_scratch_directory, print_err, print_succ, print_warning};

/// simple file creation, should return Ok(Fd)
unsafe fn test_path_open_create(fd: wasi::Fd) -> u8 {
    match wasi::path_open(fd, 0, "file", wasi::OFLAGS_CREAT, 0, 0, 0) {
        Ok(_) => (),
        Err(e) => {
            print_err(format!(
                "unexpected error while trying to open a simple file\n{:?}",
                e
            ));
            return 1;
        }
    }

    0
}

/// this test expects that the previous function (test_path_open_create) has already created the
/// file. path_open should return ERRNO_EXIST for an already existing file with (OFLAGS_CREAT |
/// OFLAGS_EXCL) flags 
unsafe fn test_path_open_exists(fd: wasi::Fd) -> u8 {
    match wasi::path_open(fd, 0, "file", wasi::OFLAGS_CREAT | wasi::OFLAGS_EXCL, 0, 0, 0) {
        Ok(_) => {
            print_err("trying to create a file that already exists".to_string());
            return 1;
        },
        Err(e) => {
            if e != wasi::ERRNO_EXIST {
                print_err(format!("Expected ERRNO_EXIST, found ERRNO_{} instead", e.name()));
                return 1;
            }
        }
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
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1)
        }
    };

    print_warning("Testing path_open ...".to_string());

    unsafe {
        if test_path_open_create(dir_fd) == 0 && test_path_open_exists(dir_fd) == 0{
            print_succ("path_open tests passed \u{2713}".to_string());
            return ExitCode::SUCCESS;
        } else {
            print_err("path_open tests failed \u{2717}".to_string());
            return ExitCode::FAILURE;
        }
    }
}
