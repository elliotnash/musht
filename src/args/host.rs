use super::MushtArgs;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;

impl MushtArgs {
    pub fn resolve_ports(&mut self) -> &mut Self{

        // Construct a new Resolver with default configuration options
        let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

        // Lookup the IP addresses associated with a name.
        match resolver.srv_lookup(format!("_ssh._tcp.{}.", self.host)){
            Ok(srv) => {
                let response = srv.iter().next().unwrap();
                self.host = response.target().to_string();
                //only want to change ports if theyre empty to allow manually specifying ports
                let port = response.port().to_string();
                if self.mosh_port == "" {self.mosh_port = port.clone();}
                if self.ssh_port == "" {self.ssh_port = port;}
            },
            Err(_) => {}
        }

        self
    }
}