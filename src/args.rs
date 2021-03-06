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
            ssh_port: String::new(),
            ssh_args: String::new(),
            user: String::new(),
            host: String::new(),
            args: Vec::new()
        }
    }
}

pub fn parse(mut args: Vec<String>) -> MushtArgs {
    // give a little wiggle room for expanding vec without relocation
    let mut musht_args = MushtArgs{args: Vec::with_capacity(args.len()+2), ..Default::default()};

    // get vec of args parsed with spaces, not equal signs
    for i in 1..args.len() {

        //skip if null arg
        if args[i] == "" {continue;}

        let full_arg = (*args[i]).to_string().to_lowercase();
        let is_arg = full_arg.starts_with("-");
        //make an option be split at space or equals
        let mut option: Vec<&str> = if is_arg {full_arg.splitn(2, "=").collect()}
            else {vec!(&full_arg)};
        let mut next_arg = String::new();
        if option.len() == 1 && is_arg && i+1 < args.len() {
            next_arg = args[i+1].clone();
            option.push(&next_arg);
            //null arg
            args[i+1] = String::new();
        }
        dbg!(&option);
        let mut add_option = true;
        for arg in &option {

            match *arg {
                "-p" | "--p" | "-port" | "--port" => {
                    if option.len() == 2 {
                        musht_args.mosh_port = option[1].to_string();
                        add_option = false;
                    } else if i+1 < args.len() {
                        musht_args.mosh_port = (*args[i+1]).to_string();
                    }
                },
                "-ssh" | "--ssh" => {
                    let ssh_args = parse_ssh_args(option[1]);
                    musht_args.ssh_args = ssh_args.0;
                    if ssh_args.1 != "" {musht_args.ssh_port = ssh_args.1;}
                    add_option = false;
                }
                _ => {
                    if arg.contains("@"){
                        let userhost: Vec<&str> = arg.splitn(2, "@").collect();
                        let hostport: Vec<&str> = userhost[1].splitn(2, ":").collect();
                        musht_args.user = userhost[0].to_string();
                        musht_args.host = hostport[0].to_string();
                        if hostport.len() == 2 {musht_args.ssh_port = hostport[1].to_string()}
                        add_option = false;
                    }
                }
            }

            if add_option {musht_args.args.push((*arg).to_string())};
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
    (args.into_iter().filter(|x| *x != "").collect::<Vec<&str>>().join(" "), port.to_string())
}
