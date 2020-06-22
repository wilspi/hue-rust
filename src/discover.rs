
use mdns::RecordKind;
// use igd;

// HueError
//
// TimeOutError


fn discover_via_npunp() {
    unimplemented!()
}

fn discover_via_upnp() {
    unimplemented!()
}

// Multicast DNS
fn discover_via_mdns() -> Result<std::net::Ipv4Addr, String> {
    const SERVICE_NAME: &'static str = "_hue._tcp.local";
    for response in mdns::discover::all(SERVICE_NAME).unwrap() {
        let response = response.unwrap();
        let addr: Option<std::net::IpAddr> = response.records()
            .filter_map(|s| match s.kind {
               RecordKind::A(addr) => Some(addr.into()),
               _ => None,
            })
            .next(); // Why next()
        match addr {
            std::net::IpAddr::V4(ipv4) => {
                return Ok(ipv4);
            },
            _ => {
                if timeout {
                    return Err("TimeOutError".into());
                }
            }
        }
    }
}

// Read: https://developers.meethue.com/documentation/hue-bridge-discovery
fn discover_bridge() -> Result<std::net::Ipv4Addr, String>  {
    return discover_via_mdns();
}
