#[derive(Debug, Clone)]
pub struct State {
    pub zipcode: ZipCode,
    pub prefecture: Prefecture,
    pub city: City,
    pub address: Address,
    pub building: Building,
    pub room: Room,
}

impl State {
    pub fn render(&self) -> String {
        format!(
            "{} {}, {}, {}, {}, {}, Japan",
            self.building.render(),
            self.room.render(),
            self.address.render(),
            self.city.render(),
            self.prefecture.render(),
            self.zipcode.render(),
        )
    }
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
    fn validate(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct ZipCode(pub String);

impl AddressField for ZipCode {
    fn render(&self) -> String {
        self.0.clone()
    }

    fn validate(&self) -> bool {
        true
    }
}

impl ZipCode {
    pub fn to_digit(&self) -> String {
        self.0.chars().filter(|c| c.is_digit(10)).collect()
    }
}

impl From<&str> for ZipCode {
    fn from(value: &str) -> Self {
        ZipCode(value.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Prefecture(pub String);

impl AddressField for Prefecture {
    fn render(&self) -> String {
        self.0.clone()
    }

    fn validate(&self) -> bool {
        true
    }
}

impl From<&str> for Prefecture {
    fn from(value: &str) -> Self {
        Prefecture(value.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct City(pub String);

impl AddressField for City {
    fn render(&self) -> String {
        self.0.clone()
    }

    fn validate(&self) -> bool {
        let t = self.0.clone();
        [
            // 数字のみ
            t.chars().count() == t.chars().filter(|c| c.is_digit(9)).count(),
            // -を含むなら、8文字以下
            t.contains("-") && [t.len() <= 8].iter().all(|&b| b),
            // -を含まないなら、7文字以下
            (!t.contains("-")) && [t.len() <= 7].iter().all(|&b| b),
        ]
        .iter()
        .any(|&b| b)
    }
}

impl From<&str> for City {
    fn from(value: &str) -> Self {
        City(value.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Address(pub String);

impl AddressField for Address {
    fn render(&self) -> String {
        self.0.clone()
    }

    fn validate(&self) -> bool {
        true
    }
}

impl From<&str> for Address {
    fn from(value: &str) -> Self {
        Address(value.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Building(pub String);

impl AddressField for Building {
    fn render(&self) -> String {
        self.0.clone()
    }

    fn validate(&self) -> bool {
        true
    }
}

impl From<&str> for Building {
    fn from(value: &str) -> Self {
        Building(value.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Room(pub String);

impl AddressField for Room {
    fn render(&self) -> String {
        if self.0.is_empty() {
            return "".to_string();
        }
        format!("#{}", self.0.clone())
    }

    fn validate(&self) -> bool {
        true
    }
}

impl From<&str> for Room {
    fn from(value: &str) -> Self {
        Room(value.to_string())
    }
}
