use lazy_static::lazy_static;
use regex::Regex;
use clap::ArgMatches;
mod host;

lazy_static! {
    static ref ARG_REGEX: Regex = Regex::new(r#"[^\s"']+|"([^"]*)"|'([^']*)'"#).unwrap();
}

#[derive(Debug)]
pub struct MushtArgs{

    address: MushtAddress,

    ssh: String,

    mosh: String,
    client: String,
    server: String,
    predict: String,
    predict_overwrite: bool,
    family: String,
    bind_server: String,
    no_ssh_pty: bool,
    no_init: bool,
    local: bool,
    experimental_remote_ip: String

}

impl Default for MushtArgs {
    fn default() -> Self {
        MushtArgs{
            
            address: MushtAddress::default(),

            ssh: String::new(),
        
            mosh: "mosh".to_string(),
            client: "mosh-client".to_string(),
            server: "mosh-server".to_string(),
            predict: "adaptive".to_string(),
            predict_overwrite: false,
            family: "prefer-inet".to_string(),
            bind_server: "ssh".to_string(),
            no_ssh_pty: false,
            no_init: false,
            local: false,
            experimental_remote_ip: "proxy".to_string()

        }
    }
}

#[derive(Debug)]
pub struct MushtAddress{
    user: Option<String>,
    host: String,
    mosh_port: Option<String>,
    ssh_port: Option<String>,
}

impl Default for MushtAddress {
    fn default() -> Self {
        MushtAddress{
            user: None,
            host: String::new(),
            mosh_port: None,
            ssh_port: None
        }
    }
}

impl MushtAddress {
    pub fn from_str(raw_host: String) -> Self {
        let mut musht_address = MushtAddress::default();
        
        // parse user
        let host_port = if raw_host.contains("@") {
            let mut split = raw_host.splitn(2, "@");
            musht_address.user = Some(split.next().unwrap().to_string());
            split.next().unwrap().to_string()
        } else {raw_host};

        // parse port and host
        if host_port.contains(":") {
            let mut split = host_port.splitn(2, ":");
            musht_address.host = split.next().unwrap().to_string();
            musht_address.ssh_port = Some(split.next().unwrap().to_string());
        } else {musht_address.host = host_port;}

        musht_address
    }
}

pub fn parse(matches: &ArgMatches) -> MushtArgs {

    let mut musht_args = MushtArgs::default();

    // resolve address and add to musht_args. Scoped because I kept typing musht_address ::)))))))))
    {
        let mut musht_address = MushtAddress::from_str(matches.value_of("HOST").unwrap().to_string());
        host::resolve(&mut musht_address);
        musht_args.address = musht_address;
    }

    // parse ssh args
    if let Some(ssh) = matches.value_of("SSH") {
        let ssh_args = parse_ssh_args(ssh);
        musht_args.ssh = ssh_args.0;
        // only overwrite if value is some
        if ssh_args.1.is_some() {
            musht_args.address.ssh_port = ssh_args.1;
        }
    }

    // parse other args
    if let Some(mosh) = matches.value_of("MOSH") {musht_args.mosh = mosh.to_string();};
    if let Some(client) = matches.value_of("CLIENT") {musht_args.client = client.to_string();};
    if let Some(server) = matches.value_of("SERVER") {musht_args.server = server.to_string();};
    if let Some(predict) = matches.value_of("PREDICT") {musht_args.predict = predict.to_string();};
    musht_args.predict_overwrite = matches.is_present("OVERWRITE");
    if let Some(family) = matches.value_of("FAMILY") {musht_args.family = family.to_string();};
    if let Some(port) = matches.value_of("PORT") {musht_args.address.mosh_port = Some(port.to_string());};
    if let Some(ssh_port) = matches.value_of("SSH PORT") {musht_args.address.ssh_port = Some(ssh_port.to_string());};
    if let Some(bind_server) = matches.value_of("PREDICT") {musht_args.bind_server = bind_server.to_string();};
    musht_args.local = matches.is_present("LOCAL");
    if let Some(experimental_remote_ip) = matches.value_of("EXPERIMENTAL REMOTE IP") 
        {musht_args.experimental_remote_ip = experimental_remote_ip.to_string();};
    musht_args.no_init = matches.is_present("NO INIT");
    musht_args.no_ssh_pty = matches.is_present("NO SSH PTY");

    musht_args

}

fn parse_ssh_args(ssh_args: &str) -> (String, Option<String>){
    let mut args: Vec<&str> = ARG_REGEX.find_iter(ssh_args).map(|x| x.as_str()).collect();

    let mut port: Option<&str> = None;
    for i in 0..args.len() {
        if args[i].contains("-p") {
            port = Some(&args[i][2..]);
            if port.is_some() && i+1 < args.len() {
                port = Some(args[i+1]);
                args[i+1] = "";
            }
            //now efectively remove from args
            args[i] = "";
        }
    }

    (args.into_iter().filter(|x| *x != "").collect::<Vec<&str>>().join(" "), port.map(String::from))
}
