use polars::prelude::*;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::Arc;

fn main() -> Result<(), &'static str> {
    let interim_dir = PathBuf::from("interim");
    let out_dir =
        PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR environment variable not set"));

    let source_path = interim_dir.join("address.csv");
    let dest_path = out_dir.join("area_table.rs");
    generate_area_table(source_path, dest_path).expect("Failed to generate area table");

    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}

fn generate_area_table(source_path: PathBuf, dest_path: PathBuf) -> Result<(), &'static str> {
    let mut dest_file = BufWriter::new(File::create(&dest_path).unwrap());

    let mut area_map = phf_codegen::Map::new();
    let mut zip_set: HashSet<String> = HashSet::new();

    let df = load_address_csv(source_path).expect("Failed to load address CSV");
    // iter rows
    for i in 0..df.height() {
        let row = df.get_row(i).expect("Failed to get row");
        println!("{:?}", row);

        // capitalize
        let zipcode = remove_quotes(row.0[0].to_string());
        let pref = text_process(remove_to_fu_ken(row.0[4].to_string()));
        let city = text_process(connect_city_suffix(row.0[5].to_string()));
        let town = text_process(connect_city_suffix(row.0[6].to_string()));

        if zip_set.contains(&zipcode) {
            continue;
        }

        area_map.entry(
            zipcode.clone(),
            format!(
                r#"Area {{
                    prefecture: "{}",
                    city: "{}",
                    address: "{}",
                }}"#,
                pref, city, town,
            )
            .as_str(),
        );
        zip_set.insert(zipcode);
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

fn text_process(s: String) -> String {
    let s = remove_quotes(s);
    let s = capitalize_words(s);
    s
}

fn remove_quotes(s: String) -> String {
    s.replace("\"", "")
}

fn remove_to_fu_ken(s: String) -> String {
    s.replace(" TO", "").replace(" FU", "").replace(" KEN", "")
}

fn connect_city_suffix(s: String) -> String {
    s
    // s.replace(" SHI", "-shi")
    //     .replace(" KU", "-ku")
    //     .replace(" CHO", "-cho")
    //     .replace(" GUN", "-gun")
    //     .replace(" MURA", "-mura")
    //     .replace(" MACHI", "-machi")
}

fn capitalize_words(s: String) -> String {
    let ss = s.split_whitespace().collect::<Vec<&str>>();
    let s = ss
        .into_iter()
        .map(|s| capitalize(s.to_string()))
        .collect::<Vec<String>>()
        .join(" ");
    s
}

fn capitalize(s: String) -> String {
    s[0..1].to_uppercase() + &s[1..].to_lowercase()
}

fn load_address_csv(path: PathBuf) -> Result<DataFrame, &'static str> {
    let mut schema = Schema::with_capacity(7);
    schema.with_column("zipcode".into(), DataType::String);
    schema.with_column("pref".into(), DataType::String);
    schema.with_column("city".into(), DataType::String);
    schema.with_column("town".into(), DataType::String);
    schema.with_column("pref_roma".into(), DataType::String);
    schema.with_column("city_roma".into(), DataType::String);
    schema.with_column("town_roma".into(), DataType::String);
    println!("{:?}", schema);

    // load no header csv
    let df = CsvReadOptions::default()
        .with_has_header(false)
        .with_schema(Some(Arc::new(schema)))
        .try_into_reader_with_file_path(Some(path.into()))
        .expect("Failed to read CSV")
        .finish()
        .expect("Failed to finish CSV");

    println!("{:?}", df.head(Some(10)));
    Ok(df)
}
