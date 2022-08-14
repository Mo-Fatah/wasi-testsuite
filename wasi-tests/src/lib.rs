use std::{
    env,
    process::{self, Command},
};
use wasi;

pub fn print_err(message: String) {
    eprintln!("\n{}", format!("\x1b[91m{}\x1b[0m", message));
}
pub fn print_succ(message: String) {
    println!("{}", format!("\x1b[92m{}\x1b[0m", message));
}
pub fn print_warning(message: String) {
    println!("\n{}", format!("\x1b[93m{}\x1b[0m", message));
}

/// Opens a fresh file descriptor for `path` where `path` should be a preopened
/// directory.
/// this function is from wasmtime's wasi-tests
pub fn open_scratch_directory(path: &str) -> Result<wasi::Fd, String> {
    unsafe {
        for i in 3.. {
            let stat = match wasi::fd_prestat_get(i) {
                Ok(s) => s,
                Err(_) => break,
            };
            // check that it is a directory
            if stat.tag != 0 {
                continue;
            }
            let mut dst = Vec::with_capacity(stat.u.dir.pr_name_len);
            if wasi::fd_prestat_dir_name(i, dst.as_mut_ptr(), dst.capacity()).is_err() {
                continue;
            }
            dst.set_len(stat.u.dir.pr_name_len);
            if dst == path.as_bytes() {
                let (base, inherit) = fd_get_rights(i);
                return Ok(
                    wasi::path_open(i, 0, ".", wasi::OFLAGS_DIRECTORY, base, inherit, 0)
                        .expect("failed to open dir"),
                );
            }
        }

        Err(format!("failed to find scratch dir"))
    }
}

/// this function is from wasmtime's wasi-tests
pub unsafe fn fd_get_rights(fd: wasi::Fd) -> (wasi::Rights, wasi::Rights) {
    let fdstat = wasi::fd_fdstat_get(fd).expect("fd_fdstat_get failed");
    (fdstat.fs_rights_base, fdstat.fs_rights_inheriting)
}

pub fn wasm_modules_path(module_name: &str) -> String {
    let crate_path = env::current_dir().unwrap().to_str().unwrap().to_string();

    format!("{}/target/wasm32-wasi/debug/{}", crate_path, module_name)
}

pub fn engine_name() -> String {
    let mut args = env::args();
    let _prog = args.next();
    let engine = if let Some(engine) = args.next() {
        engine
    } else {
        println!("usage: cargo run --bin init <wasm engine name>");
        process::exit(1);
    };

    engine
}

pub fn engine_command(engine_params: Vec<&str>, module: &str, module_params: Vec<&str>) -> Command {
    let engine_name = engine_name();
    let mut command = Command::new(&engine_name);

    if engine_name == "wasmer" {
        command.arg("run");
    }

    for param in engine_params {
        command.arg(param);
    }

    command.arg(wasm_modules_path(module));

    for param in module_params {
        command.arg(param);
    }

    command
}
