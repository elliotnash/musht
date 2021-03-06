mod help;
mod args;

use std::env;
use std::process::Command;

fn main() {

    println!("[starting musht.]");

    let args: Vec<String> = env::args().collect();

    // parse args
    let mut args = args::parse(args);
    // resolve srv records and
    args.resolve_ports();
    
    // build mosh command with args
    let mut command = Command::new("mosh");
    command.args(&args.args);

    command.arg("--ssh").arg(args.get_ssh_args());
    if args.mosh_port != "" {command.arg("-p").arg(&args.mosh_port);}
    command.arg(args.get_user_host());

    dbg!(&args);
    dbg!(&command);

    // spawn mosh blocking
    println!("[starting mosh.]");
    if command.status().is_err(){
        println!("failed to start mosh, is it installed?")
    }

}
