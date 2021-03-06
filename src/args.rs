
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

pub fn parse(args: &Vec<String>) -> Vec<String> {
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
                    parse_ssh_args(&mut musht_args, option[1])
                }
                _ => {}
            }

            musht_args.args.push((*arg).to_string());
        }
    }

    musht_args.args

}

fn parse_ssh_args(musht_args: &mut MushtArgs, ssh_args: &str){
    
}
