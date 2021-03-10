use clap::ArgMatches;

#[derive(Debug)]
pub struct MushtArgs{

    user: Option<String>,
    host: String,
    port: String,
    ssh_port: String,

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

            user: None,
            host: String::new(),
            port: "60000:61000".to_string(),
            ssh_port: "22".to_string(),
        
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

pub fn parse(app: &ArgMatches) {

    let mut musht_args = MushtArgs::default();

    let raw_host = app.value_of("HOST").unwrap();

}
