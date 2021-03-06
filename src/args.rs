
pub struct MushtArgs {
    pub mosh_port: String,
    pub ssh_port: String,
    pub user: String,
    pub host: String,
    pub args: Vec<String>
}

impl Default for MushtArgs {
    fn default() -> Self {
        MushtArgs {
            mosh_port: String::new(),
            ssh_port: "22".to_string(),
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
        let option = (*args[i]).splitn(2, "=");
        for arg in option {

            match arg {
                "-p" | "--port" => {}
                _ => {}
            }

            musht_args.args.push((*arg).to_string());
        }
    }

    musht_args.args

}
