
pub struct MushtArgs {
    pub mosh_port: String,
    pub ssh_port: String,
    pub user: String,
    pub host: String,
    pub args: Vec<String>
}

impl Default for MushtArgs {
    fn default() -> MushtArgs {
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
    let mut parsed_args: Vec<String> = Vec::with_capacity(args.len()+2);

    // get vec of args parsed with spaces, not equal signs
    for i in 1..args.len() {
        let option = (*args[i]).split("=");
        for arg in option {
            parsed_args.push((*arg).to_string());
        }
    }

    // 

    parsed_args

}
