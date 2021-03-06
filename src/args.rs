use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ARG_REGEX: Regex = Regex::new(r#"[^\s"']+|"([^"]*)"|'([^']*)'"#).unwrap();
}

#[derive(Debug)]
pub struct MushtArgs {
    pub mosh_port: String,
    pub ssh_port: String,
    pub ssh_args: String,
    pub user: String,
    pub host: String,
    pub args: Vec<String>
}

impl Default for MushtArgs {
    fn default() -> Self {
        MushtArgs {
            mosh_port: String::new(),
            ssh_port: "22".to_string(),
            ssh_args: String::new(),
            user: String::new(),
            host: String::new(),
            args: Vec::new()
        }
    }
}

pub fn parse(args: &Vec<String>) -> MushtArgs {
    // give a little wiggle room for expanding vec without relocation
    let mut musht_args = MushtArgs{args: Vec::with_capacity(args.len()+2), ..Default::default()};

    // get vec of args parsed with spaces, not equal signs
    for i in 1..args.len() {
        let full_arg = (*args[i]).to_string().to_lowercase();
        let option: Vec<&str> = full_arg.splitn(2, "=").collect();
        for arg in &option {

            match *arg {
                "-p" | "--p" | "-port" | "--port" => {
                    if i+2 < args.len() {
                        musht_args.mosh_port = option[1].to_string();
                    } else if i+2 < args.len() {
                        musht_args.mosh_port = (*args[i+1]).to_string();
                    }
                },
                "-ssh" | "--ssh" => {
                    let ssh_args = parse_ssh_args(option[1]);
                    musht_args.ssh_args = ssh_args.0;
                    if ssh_args.1 != "" {musht_args.ssh_port = ssh_args.1;}
                }
                _ => {}
            }

            musht_args.args.push((*arg).to_string());
        }
    }

    musht_args

}

fn parse_ssh_args(ssh_args: &str) -> (String, String){
    let mut args: Vec<&str> = ARG_REGEX.find_iter(ssh_args).map(|x| x.as_str()).collect();

    let mut port = "";
    for i in 0..args.len() {
        if args[i].contains("-p") {
            port = &args[i][2..];
            if port == "" && i+1 < args.len() {
                port = args[i+1];
                args[i+1] = "";
            }
            //now efectively remove from args
            args[i] = "";
        }
    }
    (args.join(" "), port.to_string())
}
