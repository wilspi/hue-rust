extern crate mdns;
use mdns::{Record, RecordKind};
use std::net::IpAddr;

/// The hostname of the devices we are searching for.
/// Every Chromecast will respond to the service name in this example.
const SERVICE_NAME: &'static str = "_hue._tcp.local";

fn main() {
    // Iterate through responses from each Cast device.
    for response in mdns::discover::all(SERVICE_NAME).unwrap() {
        let response = response.unwrap();

        let addr = response.records()
                           .filter_map(self::to_ip_addr)
                           .next();

        if let Some(addr) = addr {
            println!("found cast device at {}", addr);
        } else {
            println!("cast device does not advertise address");
        }
    }
}

fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
        _ => None,
    }
}
