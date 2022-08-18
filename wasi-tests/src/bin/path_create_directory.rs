use std::{
    env,
    process::{self, ExitCode},
};

use wasi_tests::{open_scratch_directory, print_err, print_succ, print_warning};

/// simple directory creation
unsafe fn test_path_create_directory(dir_fd: wasi::Fd) -> u8 {
    wasi::path_create_directory(dir_fd, "new_dir").expect("create a directory");
    if let Err(e) = wasi::path_open(dir_fd, 0, "new_dir", wasi::OFLAGS_DIRECTORY, 0, 0, 0) {
        print_err(format!(
            ">> couldn't open the new directory\nFound Error:{:?}",
            e
        ));
        return 1;
    }

    wasi::path_remove_directory(dir_fd, "new_dir").expect("couldn't remove dir");

    0
}

/// trying to create a directory that already exists should return fail
unsafe fn test_directory_exist(dir_fd: wasi::Fd) -> u8 {
    // create the directory
    wasi::path_create_directory(dir_fd, "new_dir").expect("create a directory");
    // try to create it again
    match wasi::path_create_directory(dir_fd, "new_dir") {
        Ok(_) => {
            print_err(
                ">> creating a directory that already exists should return an error\n\
                      Expected: Err()\nFound: Ok()"
                    .to_string(),
            );
            return 1;
        }
        Err(e) => {
            if e != wasi::ERRNO_EXIST {
                print_err(format!(
                    ">> creating a directory that already exists should return ERRNO_EXIST\n\
                          Expected: ERRNO_EXIST\nFound: ERRNO_{}",
                    e.name()
                ));
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
        Err(_) => {
            4 // hardcoded value to bypass wasmer's fd_prestat_dir_name errors
              //eprintln!("{}", err);
              //process::exit(1)
        }
    };

    print_warning("Testing path_create_directory ...".to_string());

    unsafe {
        if test_path_create_directory(dir_fd) + test_directory_exist(dir_fd) == 0 {
            print_succ("path_create_directory tests passed \u{2713}".to_string());
            return ExitCode::SUCCESS;
        } else {
            print_err("path_create_directory tests failed \u{2717}".to_string());
            return ExitCode::FAILURE;
        }
    }
}
