use clap::{Arg, App, ValueHint};

pub fn get_app() -> App<'static> {

    let app: App<'static> = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(concat!(
            env!("CARGO_PKG_DESCRIPTION"),
            "\nhomepage: ", env!("CARGO_PKG_HOMEPAGE")))
        .override_usage("Usage: mosh [options] [--] [user@]host[:ssh-port]")

        // arg for user/host/port
        .arg(Arg::new("HOST")
            .about("the host to connect to")
            .required(true)
            .hidden(true)
            .value_hint(ValueHint::Hostname)
            .index(1))
        
        //args for execs
        .arg(Arg::new("MOSH")
            .long("mosh")
            .value_name("path")
            .about("mosh executable on local machine")
            .default_value("mosh")
            .help_heading(Some("MOSH EXECUTABLES"))
            .display_order(0))
        .arg(Arg::new("CLIENT")
            .long("client")
            .value_name("path")
            .about("mosh client on local machine")
            .default_value("mosh-client")
            .help_heading(Some("MOSH EXECUTABLES"))
            .display_order(1))
        .arg(Arg::new("SERVER")
            .long("server")
            .value_name("remote-path")
            .about("mosh server on remote machine")
            .default_value("mosh-server")
            .help_heading(Some("MOSH EXECUTABLES"))
            .display_order(2))

        //predict arg
        .arg(Arg::new("PREDICT")
            .long("predict")
            .value_name("method")
            .about("change prediction type \
            \n      adaptive - local echo for slower links [default] \
            \n      always - use local echo even on fast links \
            \n      never - never use local echo \
            \n      experimental - aggressively echo even when incorrect")
            .default_value("adaptive")
            .possible_values(&[ "adaptive", "always", "never", "experimental" ])
            .hide_possible_values(true)
            .hide_default_value(true)
            .help_heading(Some("PREDICTIONS"))
            .display_order(3))
        .arg(Arg::new("OVERWRITE")
            .short('o')
            .long("predict-overwrite")
            .about("prediction overwrites instead of inserting")
            .help_heading(Some("PREDICTIONS"))
            .display_order(4))

        //networking args
        .arg(Arg::new("FAMILY")
            .long("family")
            .value_name("method")
            .about("change ip options \
            \n      inet - use IPv4 only \
            \n      inet6 - use IPv6 only \
            \n      auto - autodetect network type for single-family hosts only \
            \n      all - try all network types \
            \n      prefer-inet - use all network types, but try IPv4 first [default] \
            \n      prefer-inet6 - use all network types, but try IPv6 first ")
            .default_value("prefer-inet")
            .possible_values(&[ "inet", "inet6", "auto", "all", "prefer-inet", "prefer-inet6" ])
            .hide_possible_values(true)
            .hide_default_value(true)
            .help_heading(Some("NETWORK OPTIONS"))
            .display_order(5))
        .arg(Arg::new("PORT")
            .short('p')
            .long("port")
            .value_name("port[:port2]")
            .about("server-side UDP port or range (No effect on server-side SSH port)")
            .help_heading(Some("NETWORK OPTIONS"))
            .display_order(6))
        .arg(Arg::new("SSH PORT")
            .short('s')
            .long("ssh-port")
            .value_name("port")
            .about("server-side TCP ssh port (overrides port specified with --ssh)")
            .help_heading(Some("NETWORK OPTIONS"))
            .display_order(7))
        .arg(Arg::new("SSH")
            .long("ssh")
            .value_name("command")
            .about("ssh command to run when setting up session \
            \n      [example: \"ssh -p 2222\"]")
            .default_value("ssh")
            .help_heading(Some("NETWORK OPTIONS"))
            .display_order(8))
        .arg(Arg::new("BIND SERVER")
            .long("bind-server")
            .value_name("option")
            .about("ask the server to reply from an IP address")
            .help_heading(Some("NETWORK OPTIONS"))
            .default_value("ssh")
            .possible_values(&[ "ssh", "any", "IP" ])
            .display_order(9))
        .arg(Arg::new("LOCAL")
            .long("local")
            .about("run mosh-server locally without using ssh")
            .help_heading(Some("NETWORK OPTIONS"))
            .display_order(10))
        .arg(Arg::new("EXPERIMENTAL REMOTE IP")
            .long("experimental-remote-ip")
            .value_name("option")
            .about("select the method for discovering the remote IP address to use for mosh \n")
            .help_heading(Some("NETWORK OPTIONS"))
            .default_value("proxy")
            .possible_values(&[ "local", "remote", "proxy" ])
            .display_order(11))

        //client options args
        .arg(Arg::new("NO INIT")
            .long("no-init")
            .about("do not send terminal initialization string")
            .help_heading(Some("CLIENT OPTIONS"))
            .display_order(12))
        .arg(Arg::new("NO SSH PTY")
            .long("no-ssh-pty")
            .about("do not allocate a pseudo tty on ssh connection")
            .help_heading(Some("CLIENT OPTIONS"))
            .display_order(13));
    
    app

}