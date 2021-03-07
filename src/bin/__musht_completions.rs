use std::process::Command;
use std::fs;

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

    //process hosts
    let mut hosts = get_hosts();
    hosts.sort_unstable();
    hosts.dedup();

    for host in hosts {
       if host.starts_with(&comp_word) {
           println!("{}{}", &prefix, &host);
       }
    }
}

fn get_hosts() -> Vec<String> {

    // read ssh known_hosts file
    let contents = fs::read_to_string("/home/elliot/.ssh/known_hosts")
        .expect("Something went wrong reading the file");
    let hosts: Vec<String> = contents.lines().map(|x| {
        x.split_whitespace().next().unwrap().split(":").next().unwrap().replace(&['[', ']'][..], "") 
    }).collect();

    hosts

}
