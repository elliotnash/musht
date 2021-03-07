use std::process::Command;

fn main() {

    // get env vars
    let comp_line = std::env::var("COMP_LINE").unwrap();
    let home = std::env::var("HOME").unwrap();
    let comp_point = std::env::var("COMP_POINT").unwrap().parse::<usize>().unwrap()-1;

    let mut start_point = comp_point.clone();
    let chars: Vec<char> = comp_line.chars().collect();
    // find closest white space (word boundary)
    while chars[start_point] != ' ' {
        start_point -= 1;
    }
    // we don't want the whitepsace included
    if start_point != comp_point {start_point += 1}

    let comp_word = comp_line[start_point..comp_point+1].to_string();

    if comp_word.contains("@") {
        let split: Vec<&str> = comp_word.split("@").collect();
        print_hosts(
            home, 
            &format!("{}@", split[0]),
            split[1]
        );
    }

}

fn print_hosts(home: String, prefix: &str, comp_word: &str){
    //get raw output of host command
    let mut get_hosts = Command::new("awk");
    get_hosts.arg("-F").arg("[ ,:]")
        .arg(r#"/^[0-9a-zA-Z]/{sub(/[/,"",$1); sub(/]/,"",$1); print $1}"#)
        .arg(format!("{}/.ssh/known_hosts", home));
    let output = get_hosts.output().expect("failed to execute process");

    //process data 
    let hosts = String::from_utf8_lossy(&output.stdout);
    let mut hosts: Vec<&str> = hosts.split("\n").collect();
    hosts.sort_unstable();
    hosts.dedup();


    for host in hosts {
       if host.starts_with(&comp_word) {
           println!("{}{}", &prefix, &host);
       }
    }
}
