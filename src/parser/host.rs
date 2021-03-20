
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use serde::{Deserialize, Serialize};
use super::MushtAddress;

#[derive(Serialize, Deserialize, Debug)]
struct TxtData {
    ssh_port: Option<String>,
    mosh_ports: Option<String>,
    server: Option<String>
}

// TODO add documentation about mosh-server option to readme
pub fn resolve(musht_address: &mut MushtAddress) -> &mut MushtAddress {

    // Construct a new Resolver with default configuration options
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    // get txt record
    match resolver.txt_lookup(format!("{}.", musht_address.host)) {
        Ok(srv) => {
            let data = srv.iter().next().unwrap().to_string();
            let json_result = serde_json::from_str(&data);
            let json: TxtData = if json_result.is_ok() {
                json_result.unwrap()
            } else {
                return musht_address;
            };

            //don't overwrite manually specified stuff
            if json.mosh_ports.is_some() {musht_address.mosh_port = json.mosh_ports;};
            if json.ssh_port.is_some() {musht_address.ssh_port = json.ssh_port;};
            if json.server.is_some() {musht_address.server = json.server;};
            
        },
        Err(_) => {}
    }

    musht_address

}
