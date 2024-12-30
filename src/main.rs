use std::{env, process::Command};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let program = args.get_mut(1).cloned().unwrap();
    let args: Vec<String> = args.split_off(2);

    println!("{:?}", args);
    let mut child = Command::new(program)
        .args(args)
        .spawn()
        .expect("failed to execute child");

    let status = child.wait().expect("failed to wait on child");

    match status.code() {
        Some(code) => println!("Exited with status code: {code}"),
        None => println!("Process terminated by signal"),
    }
}
