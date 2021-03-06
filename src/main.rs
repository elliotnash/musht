mod help;
mod args;

use std::env;
use std::process::Command;

fn main() {

    println!("[starting musht.]");

    let args: Vec<String> = env::args().collect();

    let args = args::parse(args);

    // build mosh command with args
    let mut command = Command::new("mosh");
    for i in 0..args.args.len() {
        command.arg(&args.args[i]);
    }
    command.arg("--ssh").arg(args.get_ssh_args())
        .arg("-p").arg(&args.mosh_port)
        .arg(args.get_user_host());

    dbg!(&command);

    // spawn mosh blocking
    println!("[starting mosh.]");
    if command.status().is_err(){
        println!("failed to start mosh, is it installed?")
    }

}
