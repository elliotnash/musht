mod args;
mod parser;

//TODO User clap_generate for proper tab complete
use std::process::Command;

fn main() {

    let matches = args::get_app().get_matches();

    let musht_args = parser::parse(&matches);

    // build mosh command with args
    let mut command = Command::new(&musht_args.mosh);
    command.arg(&musht_args.address.to_string())
        .arg("--client").arg(&musht_args.client)
        .arg("--predict").arg(&musht_args.predict)
        .arg("--family").arg(&musht_args.family)
        .arg("--ssh").arg(&musht_args.get_ssh_args())
        .arg("--bind-server").arg(&musht_args.bind_server)
        .arg("--experimental-remote-ip").arg(&musht_args.experimental_remote_ip);

    // add optional args
    if let Some(port) = &musht_args.address.mosh_port {command.arg("--port").arg(port);};
    if let Some(server) = &musht_args.address.server {command.arg("--server").arg(server);};

    // add flags
    if (&musht_args).predict_overwrite {command.arg("--predict-overwrite");};
    if (&musht_args).local {command.arg("--local");};
    if (&musht_args).no_init {command.arg("--no-init");};
    if (&musht_args).no_ssh_pty {command.arg("--no-ssh-pty");};

    
    //spawn mosh blocking
    println!("[starting mosh.]");
    if command.status().is_err(){
        println!("failed to start mosh, is it installed?");
    }

}
