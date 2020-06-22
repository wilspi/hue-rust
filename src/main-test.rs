
extern crate mdns;
extern crate igd;

use mdns::RecordKind;


fn discover_bridge2() {
    use igd;
    let a = igd::search_gateway();
    println!("{:#?}", a);
}

fn discover_bridge() -> Result<std::net::Ipv4Addr, TimeOutError>{
    const SERVICE_NAME: &'static str = "_hue._tcp.local";
    for response in mdns::discover::all(SERVICE_NAME).unwrap() {
        let response = response.unwrap();
        let addr: Option<std::net::IpAddr> = response.records()
            .filter_map(|s| match s.kind {
               RecordKind::A(addr) => Some(addr.into()),
               _ => None,
            })
            .next();

        match addr {
            std::net::IpAddr::V4(ipv4) => {
                return Ok(ipv4);
            },
            _ => {
                if timeout {
                    return Err(TimeOutError);
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    discover_bridge();
    // discover_bridge2();
}
