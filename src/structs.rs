
pub struct HueConfig {
    pub name: String,
    // pub zigbeechannel,
    pub bridgeid: String,
    pub mac: String,
    pub dhcp: bool,
    pub ipaddress: String,
    pub netmask: String,
    pub gateway: String,
    pub proxyaddress: String,
    pub proxyport: u16,
    pub UTC: String,
    pub localtime: String,
    pub timezone: String,
    pub modelid: String,
    pub datastoreversion: String,
    pub swversion: String,
    pub apiversion: String,
    // pub swupdate:
    // pub swupdate2,
    pub linkbutton: bool,
    pub portalservices: bool,
    pub portalconnection: String,
    // pub portalState:
    // pub internetservices:
    pub factorynew: bool,
    // pub replacesbridgeid:
    // pub backup
    pub starterkitid: String,
    pub whitelist: Vec<HueUser>
}

pub struct HueUser {
    pub last_use_date: String,
    pub create_date: String,
    pub name: String
}
