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
                let port = response.port().to_string();
                self.mosh_port = port.clone();
                self.ssh_port = port;
            },
            Err(_) => {}
        }

        self
    }
}