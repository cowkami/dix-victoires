use crate::state::ZipCode;
use phf::phf_map;

#[derive(Debug, Clone)]
pub struct Area {
    pub prefecture: &'static str,
    pub city: &'static str,
    pub address: &'static str,
}

static AREA: phf::Map<&'static str, Area> = phf_map! {
    "1680031" => Area {
        prefecture: "Tokyo",
        city: "Suginami-ku",
        address: "Asagaya",
    },
    "1550081" => Area {
        prefecture: "Tokyo",
        city: "Chiyoda-ku",
        address: "Nagata-cho",
    },
};

// Example usage
pub fn by_zipcode(zipcode: &ZipCode) -> Option<&'static Area> {
    AREA.get(&zipcode.to_digit())
}
