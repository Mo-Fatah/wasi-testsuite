use std::{env, process};
use wasi::{self, fd_prestat_dir_name};
use wasi_tests::{format_succ, format_err, format_warning};

/// the name returned from fd_prestat_dir_name should be exactly the same as the name of the
/// directory passed to the wasm module
unsafe fn test_valid_path(path: String) -> u8 {

    for i in 3.. {
        let stat = match wasi::fd_prestat_get(i) {
            Ok(s) => s,
            Err(_) => break
        };
        
        let mut dst = Vec::with_capacity(stat.u.dir.pr_name_len);
        if wasi::fd_prestat_dir_name(i, dst.as_mut_ptr(), dst.capacity()).is_err() {
            continue;
        }
        dst.set_len(stat.u.dir.pr_name_len);

        if path.as_bytes() == dst && path == String::from_utf8_lossy(&dst) {
            return 0
        }
    }

    eprintln!("{}", format_err("failed to find dir".to_string()));

    1
}

/// 0, 1 and 2 file descriptors should return NOTDIR error
unsafe fn test_zero_one_two() -> u8 {

    let mut dst = Vec::with_capacity(6);

    match fd_prestat_dir_name(0, dst.as_mut_ptr(), 5) {
        Err(wasi::ERRNO_NOTDIR) => (),
        _ => {
            eprintln!("{}", format_err("file descriptor 0 should return NOTDIR Error".to_string()));
            return 1
        }
    }

    match fd_prestat_dir_name(1, dst.as_mut_ptr(), 6) {
        Err(wasi::ERRNO_NOTDIR) => (),
        _ => {
            eprintln!("{}", format_err("file descriptor 1 should return NOTDIR Error".to_string()));
            return 1
        }
    }

    match fd_prestat_dir_name(2, dst.as_mut_ptr(), 6) {
        Err(wasi::ERRNO_NOTDIR) => (),
        _ => {
            eprintln!("{}", format_err("file descriptor 2 should return NOTDIR Error".to_string()));
            return 1
        }
    }

    0
}


fn main () {
    let mut args = env::args();
    let prog = args.next().unwrap();
    let arg = if let Some(arg) = args.next() {
        arg
    } else {
        eprintln!("usage: {} <scratch directory>", prog);
        process::exit(1);
    };

    println!("{}", format_warning("Testing fd_prestat_dir_name ...".to_string()));

    unsafe {
        if test_valid_path(arg) == 0 && test_zero_one_two() == 0 {
            println!("{}", format_succ("fd_prestat_dir_name tests passed \u{2713}".to_string()));
        } else {
            eprintln!("{}", format_err("fd_prestat_dir_name tests failed \u{2717}".to_string()));
        }
    }
}
