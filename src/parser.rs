use clap::ArgMatches;
mod host;

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
    port: Option<String>,
    ssh_port: Option<String>,
}

impl Default for MushtAddress {
    fn default() -> Self {
        MushtAddress{
            user: None,
            host: String::new(),
            port: None,
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

pub fn parse(app: &ArgMatches) {

    let mut musht_args = MushtArgs::default();

    //host::resolve(app.value_of("HOST").unwrap().to_string());

    let addr = MushtAddress::from_str(app.value_of("HOST").unwrap().to_string());
    dbg!(addr);

}
