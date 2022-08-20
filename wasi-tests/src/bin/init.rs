use std::process::{self, Command, ExitStatus};
use wasi_tests::engine_command;

fn main() {
    let mut failed_test_cases = 0;
    let mut tests_count = 0;

    println!("\nCompiling to wasm32-wasi..\n");
    let _command = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-wasi")
        .status()
        .unwrap();

    if !fd_prestat_get_test().success() {
        failed_test_cases += 1;
    };
    tests_count += 1;

    if !fd_prestat_dir_name_test().success() {
        failed_test_cases += 1;
    }
    tests_count += 1;

    if !path_open_test().success() {
        failed_test_cases += 1;
    }
    tests_count += 1;

    if !fd_filestat_get_test().success() {
        failed_test_cases += 1;
    }
    tests_count += 1;

    if !path_create_directory_test().success() {
        failed_test_cases += 1;
    }
    tests_count += 1;

    if !fd_fdstat_get_test().success() {
        failed_test_cases += 1;
    }
    tests_count += 1;

    if !path_unlink_file_test().success() {
        failed_test_cases += 1;
    }
    tests_count += 1;

    if failed_test_cases == 0 {
        println!(
            "\ntest results: {}. {} failed, {} passed",
            format!("\x1b[92m{}\x1b[0m", "Ok"),
            failed_test_cases,
            tests_count - failed_test_cases
        );
    } else {
        println!(
            "\ntest results: {}. {} failed, {} passed",
            format!("\x1b[91m{}\x1b[0m", "FAILED"),
            failed_test_cases,
            tests_count - failed_test_cases
        );
    }
}

fn fd_prestat_get_test() -> ExitStatus {
    // make a directory to be used by the module
    Command::new("mkdir").arg("scratch_dir").status().unwrap();

    let mut fd_prestat_get_test = engine_command(
        vec!["--dir", "scratch_dir"],
        "fd_prestat_get.wasm",
        vec!["scratch_dir"],
    );

    let fd_prestat_get_result = match fd_prestat_get_test.status() {
        Ok(result) => result,
        Err(e) => {
            eprintln!("couldn't run the engine\n{:?}", e);
            process::exit(1);
        }
    };

    // remove the directory
    Command::new("rmdir").arg("scratch_dir").status().unwrap();

    fd_prestat_get_result
}

fn fd_prestat_dir_name_test() -> ExitStatus {
    // make a directory to be used by the module
    Command::new("mkdir").arg("scratch_dir").status().unwrap();

    let mut fd_prestat_dir_name_test = engine_command(
        vec!["--dir", "scratch_dir"],
        "fd_prestat_dir_name.wasm",
        vec!["scratch_dir"],
    );

    let fd_prestat_dir_name_result = match fd_prestat_dir_name_test.status() {
        Ok(result) => result,
        Err(e) => {
            eprintln!("couldn't run the engine\n{:?}", e);
            process::exit(1);
        }
    };

    // remove the directory
    Command::new("rmdir").arg("scratch_dir").status().unwrap();

    fd_prestat_dir_name_result
}

fn path_open_test() -> ExitStatus {
    // make a directory to be used by the module
    Command::new("mkdir").arg("scratch_dir").status().unwrap();

    let mut path_open_test = engine_command(
        vec!["--dir", "scratch_dir"],
        "path_open.wasm",
        vec!["scratch_dir"],
    );

    let path_open_result = match path_open_test.status() {
        Ok(result) => result,
        Err(e) => {
            eprintln!("couldn't run the engine\n{:?}", e);
            process::exit(1);
        }
    };

    // remove the directory
    Command::new("rm")
        .arg("-rf")
        .arg("scratch_dir")
        .status()
        .unwrap();

    path_open_result
}

fn fd_filestat_get_test() -> ExitStatus {
    // make a directory to be used by the module
    Command::new("mkdir").arg("scratch_dir").status().unwrap();

    let mut fd_filestat_get_test = engine_command(
        vec!["--dir", "scratch_dir"],
        "fd_filestat_get.wasm",
        vec!["scratch_dir"],
    );

    let fd_filestat_get_result = match fd_filestat_get_test.status() {
        Ok(result) => result,
        Err(e) => {
            eprintln!("couldn't run the engine\n{:?}", e);
            process::exit(1);
        }
    };

    // remove the directory
    Command::new("rm")
        .arg("-rf")
        .arg("scratch_dir")
        .status()
        .unwrap();

    fd_filestat_get_result
}

fn path_create_directory_test() -> ExitStatus {
    // make a directory to be used by the module
    Command::new("mkdir").arg("scratch_dir").status().unwrap();

    let mut path_create_directory_test = engine_command(
        vec!["--dir", "scratch_dir"],
        "path_create_directory.wasm",
        vec!["scratch_dir"],
    );

    let path_create_directory_result = match path_create_directory_test.status() {
        Ok(result) => result,
        Err(e) => {
            eprintln!("couldn't run the engine\n{:?}", e);
            process::exit(1);
        }
    };

    // remove the directory
    Command::new("rm")
        .arg("-rf")
        .arg("scratch_dir")
        .status()
        .unwrap();

    path_create_directory_result
}

fn fd_fdstat_get_test() -> ExitStatus {
    // make a directory to be used by the module
    Command::new("mkdir").arg("scratch_dir").status().unwrap();

    let mut fd_fdstat_get_test = engine_command(
        vec!["--dir", "scratch_dir"],
        "fd_fdstat_get.wasm",
        vec!["scratch_dir"],
    );

    let fd_fdstat_get_result = match fd_fdstat_get_test.status() {
        Ok(result) => result,
        Err(e) => {
            eprintln!("couldn't run the engine\n{:?}", e);
            process::exit(1);
        }
    };

    // remove the directory
    Command::new("rm")
        .arg("-rf")
        .arg("scratch_dir")
        .status()
        .unwrap();

    fd_fdstat_get_result
}

fn path_unlink_file_test() -> ExitStatus {
    // make a directory to be used by the module
    Command::new("mkdir").arg("scratch_dir").status().unwrap();

    let mut path_unlink_file_test = engine_command(
        vec!["--dir", "scratch_dir"],
        "path_unlink_file.wasm",
        vec!["scratch_dir"],
    );

    let path_unlink_file_result = match path_unlink_file_test.status() {
        Ok(result) => result,
        Err(e) => {
            eprintln!("couldn't run the engine\n{:?}", e);
            process::exit(1);
        }
    };
    // remove the directory
    Command::new("rm")
        .arg("-rf")
        .arg("scratch_dir")
        .status()
        .unwrap();

    path_unlink_file_result
}
