use serde::Deserialize;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

fn main() -> Result<(), &'static str> {
    let interim_dir = PathBuf::from("interim");
    let out_dir =
        PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR environment variable not set"));

    let source_path = interim_dir.join("address.jsonl");
    let dest_path = out_dir.join("area_table.rs");
    generate_area_table(source_path, dest_path).expect("Failed to generate area table");

    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}

fn generate_area_table(source_path: PathBuf, dest_path: PathBuf) -> Result<(), &'static str> {
    #[derive(Debug, Clone, Deserialize)]
    struct Address {
        // region_id: String,
        zip: String,
        pref_kana: String,
        city_kana: String,
        town_kana: String,
    }

    let mut source_file = BufReader::new(File::open(&source_path).unwrap());
    let mut dest_file = BufWriter::new(File::create(&dest_path).unwrap());

    let mut area_map = phf_codegen::Map::new();
    let mut line = String::new();
    let mut zip_set: HashSet<String> = HashSet::new();
    while source_file.read_line(&mut line).unwrap() > 0 {
        let address: Address = serde_json::from_str(&line).unwrap();
        line.clear(); // Clear the line for the next iteration

        if zip_set.contains(&address.zip) {
            continue;
        }
        zip_set.insert(address.zip.clone());

        area_map.entry(
            address.zip,
            format!(
                r#"Area {{
                    prefecture: "{}",
                    city: "{}",
                    address: "{}",
                }}"#,
                address.pref_kana, address.city_kana, address.town_kana,
            )
            .as_str(),
        );
    }

    write!(
        &mut dest_file,
        "static AREA: phf::Map<&'static str, Area> = {}",
        area_map.build()
    )
    .unwrap();

    write!(&mut dest_file, ";\n").unwrap();

    Ok(())
}
