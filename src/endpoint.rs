
pub struct Endpoint {
    pub address: String,
    pub port: u16
}

impl Endpoint {
    pub fn new(address: &str) -> Endpoint {
        Endpoint {
            address: address.to_string(),
            port: 443
        }
    }
}

