use crate::state::{Address, City, Prefecture, ZipCode};
use std::collections::HashMap;

pub struct MemoryDB {
    table: HashMap<String, Area>,
}

pub struct Area {
    pub prefecture: Prefecture,
    pub city: City,
    pub address: Address,
}

impl Area {
    pub fn by_zipcode(zipcode: ZipCode) -> Area {
        let zipcode = zipcode.to_digit();
        Area {
            prefecture: Prefecture("Tokyo".to_string()),
            city: City("Shinjuku".to_string()),
            address: Address("Nishi-shinjuku".to_string()),
        }
    }
}
