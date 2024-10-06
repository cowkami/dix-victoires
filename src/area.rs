use crate::state::ZipCode;

#[derive(Debug, Clone)]
pub struct Area {
    pub prefecture: &'static str,
    pub city: &'static str,
    pub address: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/area_table.rs"));

// Example usage
pub fn by_zipcode(zipcode: &ZipCode) -> Option<&'static Area> {
    AREA.get(&zipcode.to_digit())
}
