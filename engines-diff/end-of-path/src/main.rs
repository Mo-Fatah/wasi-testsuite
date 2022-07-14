use std::{env, process};
use anyhow::Result;
use wasi;

fn main() -> Result<()> {
    let mut args = env::args(); 
    let prog = args.next().unwrap();
    let arg = if let Some(arg) = args.next() {
        arg
    } else {
        eprintln!("usage: {} <scratch directory>", prog);
        process::exit(1);
    };

    // looping from file descriptor number 3 forward ...
    for i in 3.. {

        unsafe {
            // return a Prestat struct for the file/dir with fd = i
            let stat = match wasi::fd_prestat_get(i) {
                Ok(s) => s,
                Err(_) => break,
            };

            // a vector to represent the directory name in bytes
            let mut dst = Vec::with_capacity(stat.u.dir.pr_name_len);

            if wasi::fd_prestat_dir_name(i, dst.as_mut_ptr(), dst.capacity()).is_err() {
                continue;
            }

            dst.set_len(stat.u.dir.pr_name_len);

            if dst == arg.as_bytes() { 
                println!("arg: {:?}\ndst: {:?}", arg.as_bytes(), dst);
                println!("Wasmtime used");
            } else {  
                if i == 3 {
                    continue; // fd=3 is reserved for the root, we skip that and start from fd=4
                }
                println!("arg: {:?}\ndst: {:?}", arg.as_bytes(), dst);
                println!("Wasmer used");
            }
        }
    }

    Ok(())
}
