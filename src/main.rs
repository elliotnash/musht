mod help;
mod args;
mod parser;

//TODO User clap_generate for proper tab complete
use std::env;
use std::process::Command;

fn main() {

    let matches = args::get_app().get_matches();

    let musht_args = parser::parse(&matches);

    println!("{}", musht_args.address.to_string());

    // build mosh command with args
    // let mut command = Command::new("mosh")
    //     .arg(musht_args.address.to_string());

    // command.arg("--ssh").arg(args.get_ssh_args());
    // if args.mosh_port != "" {command.arg("-p").arg(&args.mosh_port);}
    // command.arg(args.get_user_host());

    // dbg!(args);
    // dbg!(&command);

    // //spawn mosh blocking
    // println!("[starting mosh.]");
    // if command.status().is_err(){
    //     println!("failed to start mosh, is it installed?");
    // }

}
