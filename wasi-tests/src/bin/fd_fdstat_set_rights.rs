use std::{
    env,
    process::{self, ExitCode},
};

use wasi_tests::{open_scratch_directory, print_err, print_succ, print_warning};

unsafe fn remove_file_rights(dir_fd: wasi::Fd) -> u8 {
    // creating a file with some base rights
    let file_fd = wasi::path_open(
        dir_fd,
        0,
        "file",
        wasi::OFLAGS_CREAT,
        wasi::RIGHTS_FD_READ | wasi::RIGHTS_FD_SYNC,
        0,
        0,
    )
    .expect("creating a file");
    let fd_stat = wasi::fd_fdstat_get(file_fd).expect("getting fd stat");
    // make sure that the fd has base fd_read rights
    assert_ne!(
        fd_stat.fs_rights_base & wasi::RIGHTS_FD_READ,
        0,
        "file should have fd_read right"
    );
    assert_ne!(
        fd_stat.fs_rights_base & wasi::RIGHTS_FD_SYNC,
        0,
        "file should have fd_sync right"
    );

    // dropping fd_read right and keep fd_sync
    if let Err(e) = wasi::fd_fdstat_set_rights(file_fd, wasi::RIGHTS_FD_SYNC, 0) {
        print_err(format!(
            ">> failed to drop fd_read right.\nFound Error: {:?}",
            e
        ));
        return 1;
    }
    // file should have fd_sync right without fd_read
    let fd_stat = wasi::fd_fdstat_get(file_fd).expect("getting fd stat");
    assert_eq!(
        fd_stat.fs_rights_base & wasi::RIGHTS_FD_READ,
        0,
        "file should NOT have fd_read right"
    );
    assert_ne!(
        fd_stat.fs_rights_base & wasi::RIGHTS_FD_SYNC,
        0,
        "file should have fd_sync right"
    );

    // droping all rights
    if let Err(e) = wasi::fd_fdstat_set_rights(file_fd, 0, 0) {
        print_err(format!(
            ">> failed to drop fd_sync right.\nFound Error: {:?}",
            e
        ));
        return 1;
    }
    let fd_stat = wasi::fd_fdstat_get(file_fd).expect("getting fd stat");
    assert_eq!(
        fd_stat.fs_rights_base & wasi::RIGHTS_FD_READ,
        0,
        "file should NOT have fd_read right"
    );
    assert_eq!(
        fd_stat.fs_rights_base & wasi::RIGHTS_FD_SYNC,
        0,
        "file should NOT have fd_sync right"
    );
    wasi::path_unlink_file(dir_fd, "file").expect("removing file");

    // trying the same with directory
    wasi::path_create_directory(dir_fd, "new_dir").expect("creating new_dir");
    let new_dir_fd = wasi::path_open(
        dir_fd,
        0,
        "new_dir",
        wasi::OFLAGS_DIRECTORY,
        wasi::RIGHTS_FD_READDIR,
        wasi::RIGHTS_FD_READ,
        0,
    )
    .expect("opening new_dir");

    // checking that base and inheriting rights exist
    let dir_fd_stat = wasi::fd_fdstat_get(new_dir_fd).expect("getting dir stat");
    assert_ne!(
        dir_fd_stat.fs_rights_base & wasi::RIGHTS_FD_READDIR,
        0,
        "directory should have base fd_readdir rights"
    );
    assert_ne!(
        dir_fd_stat.fs_rights_inheriting & wasi::RIGHTS_FD_READ,
        0,
        "directory should have inheriting fd_read rights"
    );
    // droping base and inheriting rights
    if let Err(e) = wasi::fd_fdstat_set_rights(new_dir_fd, 0, 0) {
        print_err(format!(
            ">> failed to drop base and inheriting rights.\nFound Error: {:?}",
            e
        ));
        return 1;
    }
    // checking that all rights was removed
    let dir_fd_stat = wasi::fd_fdstat_get(new_dir_fd).expect("getting dir stat");
    assert_eq!(
        dir_fd_stat.fs_rights_base & wasi::RIGHTS_FD_READDIR,
        0,
        "directory should NOT have base fd_readdir rights"
    );
    assert_eq!(
        dir_fd_stat.fs_rights_inheriting & wasi::RIGHTS_FD_READ,
        0,
        "directory should NOT have inheriting fd_read rights"
    );
    wasi::path_remove_directory(dir_fd, "new_dir").expect("removing new_dir");

    0
}

/// using fd_fdstat_set_rights to add rights should fail
unsafe fn add_file_rights(dir_fd: wasi::Fd) -> u8 {
    // creating a file with arbitrary rights
    let file_fd = wasi::path_open(
        dir_fd,
        0,
        "file",
        wasi::OFLAGS_CREAT,
        wasi::RIGHTS_FD_SEEK,
        0,
        0,
    )
    .expect("creating file");
    // trying to add fd_read right should fail
    match wasi::fd_fdstat_set_rights(file_fd, wasi::RIGHTS_FD_READ, 0) {
        Ok(_) => {
            print_err(">> adding new rights to a file should return Err(). Found Ok()".to_string());
            return 1;
        }
        Err(e) => {
            if e != wasi::ERRNO_NOTCAPABLE {
                print_err(format!(
                    ">> adding new rights to a file should return ERRNO_NOTCAPABLE\n\
                                  Found Error: ERRNO_{}",
                    e.name()
                ));
                return 1;
            }
        }
    }
    wasi::path_unlink_file(dir_fd, "file").expect("removing file");

    // trying the with a directory
    wasi::path_create_directory(dir_fd, "new_dir").expect("creating new_dir");
    let new_dir_fd = wasi::path_open(dir_fd, 0, "new_dir", wasi::OFLAGS_DIRECTORY,
                                     wasi::RIGHTS_FD_READDIR, wasi::RIGHTS_FD_READ, 0).expect("opening new_dir");
    match wasi::fd_fdstat_set_rights(new_dir_fd, wasi::RIGHTS_FD_FILESTAT_GET, wasi::RIGHTS_FD_READDIR) {
        Ok(_) => {
            print_err(">> adding new rights to a directory should return Err(). Found Ok()".to_string());
            return 1;
        }
        Err(e) => {
            if e != wasi::ERRNO_NOTCAPABLE {
                print_err(format!(
                    ">> adding new rights to a directory should return ERRNO_NOTCAPABLE\n\
                                  Found Error: ERRNO_{}",
                    e.name()
                ));
                return 1;
            }
        }
    }

    wasi::path_remove_directory(dir_fd, "new_dir").expect("removing new_dir");

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

    print_warning("Testing fd_fdstat_set_rights ...".to_string());

    unsafe {
        if remove_file_rights(dir_fd) + add_file_rights(dir_fd) == 0 {
            print_succ("fd_fdstat_set_rights tests passed \u{2713}".to_string());
            return ExitCode::SUCCESS;
        } else {
            print_err("fd_fdstat_set_rights tests failed \u{2717}".to_string());
            return ExitCode::FAILURE;
        }
    }
}
