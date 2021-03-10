
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use serde::{Deserialize, Serialize};
use super::MushtAddress;

#[derive(Serialize, Deserialize, Debug)]
struct TxtData {
    ssh_port: String,
    mosh_ports: String
}

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

            //only want to change ports if theyre empty to allow manually specifying ports
            musht_address.mosh_port = Some(json.mosh_ports);
            musht_address.ssh_port = Some(json.ssh_port);
            
        },
        Err(_) => {}
    }

    musht_address

}
