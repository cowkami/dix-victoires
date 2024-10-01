#[derive(Debug, Clone)]
pub struct State {
    pub zipcode: ZipCode,
    pub prefecture: Prefecture,
    pub city: City,
    pub address: Address,
    pub building: Building,
    pub room: Room,
}

#[derive(Debug, Clone)]
pub enum AddressFieldType {
    ZipCode,
    Prefecture,
    City,
    Address,
    Building,
    Room,
}

#[derive(Debug, Clone)]
pub struct ZipCode(pub String);

#[derive(Debug, Clone)]
pub struct Prefecture(pub String);

#[derive(Debug, Clone)]
pub struct City(pub String);

#[derive(Debug, Clone)]
pub struct Address(pub String);

#[derive(Debug, Clone)]
pub struct Building(pub String);

#[derive(Debug, Clone)]
pub struct Room(pub String);
