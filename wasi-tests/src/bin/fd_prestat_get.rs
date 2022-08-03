use std::{
    env,
    process::{self, ExitCode},
};
use wasi;
use wasi_tests::{print_err, print_succ, print_warning};

/// check that file descriptors 0, 1 and 2 shouldn't be accepted as a paramter and
/// fd_prestat_get should return 'bad file descriptor' error if they are passed
unsafe fn test_zero_one_two_fd() -> u8 {
    match wasi::fd_prestat_get(0) {
        Ok(_) => {
            print_err("test failed: file decriptor 0 should not be accepted".to_string());
            return 1;
        }
        Err(e) => {
            if e != wasi::ERRNO_BADF {
                print_err(format!(
                    "test failed: should get ERRNO_BADF. Got ERRNO_{} instead",
                    e.name()
                ));
                return 1;
            }
        }
    };

    match wasi::fd_prestat_get(1) {
        Ok(_) => {
            print_err("test failed: file decriptor 1 should not be accepted".to_string());
            return 1;
        }
        Err(e) => {
            if e != wasi::ERRNO_BADF {
                print_err(format!(
                    "Should get ERRNO_BADF. Got ERRNO_{} instead",
                    e.name()
                ));
                return 1;
            }
        }
    };

    match wasi::fd_prestat_get(2) {
        Ok(_) => {
            print_err("test failed: file decriptor 2 should not be accepted".to_string());
            return 1;
        }
        Err(e) => {
            if e != wasi::ERRNO_BADF {
                print_err(format!(
                    "Should get ERRNO_BADF. Got ERRNO_{} instead",
                    e.name()
                ));
                return 1;
            }
        }
    };

    0
}

/// the dir name length returned from fd_prestat_get should be exactly the same as the dir name
/// length passed to the wasm module
unsafe fn test_correct_dir_name_len(arg: String) -> u8 {
    for i in 3.. {
        let stat = match wasi::fd_prestat_get(i) {
            Ok(s) => s,
            Err(_) => break,
        };

        if arg.len() == stat.u.dir.pr_name_len {
            return 0;
        }
    }

    print_err("couldn't find the directory/file".to_string());

    1
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

    print_warning("Testing fd_prestat_get ...".to_string());
    unsafe {
        if test_zero_one_two_fd() == 0 && test_correct_dir_name_len(arg) == 0 {
            print_succ("fd_prestat_get tests passed \u{2713}".to_string());
            return ExitCode::SUCCESS;
        } else {
            print_err("fd_prestat_get tests failed \u{2717}".to_string());
            return ExitCode::FAILURE;
        }
    }
}
