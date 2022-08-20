use std::{
    env,
    process::{self, ExitCode},
};
use wasi_tests::{open_scratch_directory, print_err, print_succ, print_warning};

unsafe fn simple_unlink(dir_fd: wasi::Fd) -> u8 {
    wasi::path_open(dir_fd, 0, "file", wasi::OFLAGS_CREAT, 0, 0, 0).expect("creating a file");
    // calling path_unlink_file should succeed
    if let Err(_) = wasi::path_unlink_file(dir_fd, "file") {
        print_err(">> couldn't unlink file".to_string());
        return 1;
    }

    match wasi::path_open(dir_fd, 0, "file", 0, 0, 0, 0) {
        Ok(_) => {
            print_err(">> opening an unlinked file should return Err(). Found Ok()".to_string());
            return 1;
        }
        Err(e) => {
            if e != wasi::ERRNO_NOENT {
                print_err(format!(
                    ">> opening an unlinked file should get ERRNO_NOENT\nFound Error: ERRNO_{}",
                    e.name()
                ));
                return 1;
            }
        }
    }
    0
}

/// checking that removing one link doesn't affect other links
unsafe fn unlink_multi_links(dir_fd: wasi::Fd) -> u8 {
    wasi::path_open(dir_fd, 0, "file", wasi::OFLAGS_CREAT, 0, 0, 0).expect("creating a file");
    // creating new link for "file"
    wasi::path_link(dir_fd, 0, "file", dir_fd, "file_link2").expect("creating new link");
    // unlink first link for "file"
    wasi::path_unlink_file(dir_fd, "file").expect("trying to remove first link of file");
    // checking that second link still exists
    if let Err(e) = wasi::path_open(dir_fd, 0, "file_link2", 0, 0, 0, 0) {
        print_err(format!(
            ">> failed to open a valid link for a file\nFound Error: {:?}",
            e
        ));
        return 1;
    }
    // removing second link
    wasi::path_unlink_file(dir_fd, "file_link2").expect("trying to remove second link of file");
    match wasi::path_open(dir_fd, 0, "file_link2", 0, 0, 0, 0) {
        Ok(_) => {
            print_err(
                ">> opening a removed file should return an error. Found Ok()".to_string(),
            );
            return 1;
        }
        Err(e) => {
            if e != wasi::ERRNO_NOENT {
                print_err(format!(
                    ">> opening a removed file should return ERRNO_NOENT\nFound Error: ERRNO_{}",
                    e.name()
                ));
                return 1;
            }
        }
    }

    0
}

unsafe fn unlink_dir(dir_fd: wasi::Fd) -> u8 {
    // create directory
    wasi::path_create_directory(dir_fd, "new_dir").expect("creating a directory");
    wasi::path_open(dir_fd, 0, "new_dir", wasi::OFLAGS_DIRECTORY, 0, 0, 0)
        .expect("opening directory");
    match wasi::path_unlink_file(dir_fd, "new_dir") {
        Ok(_) => {
            print_err(
                ">> using path_unlink_file on a directory should return Err(). Found Ok()"
                    .to_string(),
            );
            return 1;
        }
        Err(e) => {
            if e != wasi::ERRNO_ISDIR {
                print_err(format!(
                    ">> using path_unlink_file on a directory should return ERRNO_ISDIR\n\
                                  Found Error: ERRNO_{}",
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

    print_warning("Testing path_unlink_file ...".to_string());

    unsafe {
        if simple_unlink(dir_fd) + unlink_multi_links(dir_fd) + unlink_dir(dir_fd) == 0 {
            print_succ("path_unlink_file tests passed \u{2713}".to_string());
            return ExitCode::SUCCESS;
        } else {
            print_err("path_unlink_file tests failed \u{2717}".to_string());
            return ExitCode::FAILURE;
        }
    }
}
