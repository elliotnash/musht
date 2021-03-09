use clap::{Arg, App, ArgGroup, ValueHint, ArgMatches};

pub fn get_args() -> ArgMatches {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(&*format!( "{description}\nhomepage: {homepage}",
            description = env!("CARGO_PKG_DESCRIPTION"),
            homepage = env!("CARGO_PKG_HOMEPAGE")))
        .override_usage("Usage: mosh [options] [--] [user@]host[:ssh-port]")

        .arg(Arg::new("HOST")
            .about("The host to connect to")
            .required(true)
            .value_hint(ValueHint::Hostname)
            .index(1))

        .arg(Arg::new("PORT")
            .short('p')
            .long("port")
            .about("Sets the port(s) mosh communicates on"))

        .group(ArgGroup::new("PORTS")
            .args(&["PORT"])).help_heading("TEST")
        
        .get_matches()
}