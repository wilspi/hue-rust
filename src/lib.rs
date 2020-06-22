
extern crate mdns;
extern crate hyper;

use mdns::RecordKind;


use hyper::Client;
use hyper::rt::{self, Future, Stream};

enum HueError {
    TimeOutError {
        message: String
    },
    InternalError {
        message: String
    }
}

struct Hue;
impl Hue {
    // Multicast DNS
    fn discover_via_mdns() -> Result<std::net::Ipv4Addr, HueError> {
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
                Some(std::net::IpAddr::V4(ipv4)) => {
                    return Ok(ipv4);
                },
                _ => {
                    // if timeout {
                        return Err(HueError::TimeOutError{message:"timeout".into()});
                    // }
                }
            }
        }
        return Err(HueError::InternalError{message:"failed mdns discovery".into()});
    }

    pub fn discover() -> Result<Bridge, HueError> {
        let ip = Hue::discover_via_mdns()?;
        return Ok(Bridge{host_address:ip});
    }
}

struct Bridge {
    host_address: std::net::Ipv4Addr,
    client: String,
}


impl Bridge {
    fn fetch_url(&self, url: &str) -> impl Future<Item=(), Error=()> {
        let client = Client::new();
        let url = self.host_address.clone().to_string().push_str(url);
        let uri = url.parse::<hyper::Uri>().unwrap();

        client.get(url)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
