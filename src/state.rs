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

pub trait AddressField {
    fn render(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct ZipCode(pub String);

impl AddressField for ZipCode {
    fn render(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Prefecture(pub String);

impl AddressField for Prefecture {
    fn render(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct City(pub String);

impl AddressField for City {
    fn render(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Address(pub String);

impl AddressField for Address {
    fn render(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Building(pub String);

impl AddressField for Building {
    fn render(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Room(pub String);

impl AddressField for Room {
    fn render(&self) -> String {
        format!("#{}", self.0.clone())
    }
}
