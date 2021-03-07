use super::MushtArgs;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TxtData {
    ssh_port: String,
    mosh_ports: String
}

impl MushtArgs {
    pub fn resolve_ports(&mut self) -> &mut Self{

        // Construct a new Resolver with default configuration options
        let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

        // get txt record
        match resolver.txt_lookup("i5.server.elliotnash.org."){
            Ok(srv) => {
                let data = srv.iter().next().unwrap().to_string();
                let json_result = serde_json::from_str(&data);
                let json: TxtData = if json_result.is_ok() {
                    json_result.unwrap()
                } else {
                    return self;
                };
    
                //only want to change ports if theyre empty to allow manually specifying ports
                if self.mosh_port == "" {self.mosh_port = json.mosh_ports;}
                if self.ssh_port == "" {self.ssh_port = json.ssh_port;}
                
            },
            Err(_) => {}
        }

        self
    }
}