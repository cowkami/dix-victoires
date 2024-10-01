#[derive(Debug, Clone)]
pub struct State {
    pub zipcode: String,
    pub prefecture: String,
    pub city: String,
    pub address: String,
    pub building: String,
    pub room: String,
}

#[derive(Debug, Clone)]
pub enum AddressType {
    ZipCode,
    Prefecture,
    City,
    Address,
    Building,
    Room,
}
