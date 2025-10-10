use glob::glob;
use serde::Deserialize;
use std::{collections::HashMap, path::Path};

#[derive(Debug, Deserialize)]
struct Row {
    #[serde(rename = "English")]
    english: String,

    #[serde(rename = "Translation")]
    translation: String,
}

fn read_translation(path: &Path) -> HashMap<String, String> {
    let mut mapping = HashMap::default();
    let mut rdr = csv::Reader::from_path(path).unwrap();
    for result in rdr.deserialize() {
        let row: Row = result.unwrap();
        if !row.english.is_empty()
            && !row.translation.is_empty()
        {
            // Note that we treat "-" as a deliberate indicator
            // of an empty translation.
            let translation = if row.translation == "-" {
                "".to_string()
            } else {
                row.translation.trim().to_string()
            };
            mapping.insert(
                row.english.trim().to_string(),
                translation,
            );
        }
    }
    mapping
}

fn main() {
    for entry in glob("util/i18n/transl/*.csv").unwrap() {
        let path = entry.unwrap();
        let stem = path.file_stem().unwrap().to_str().unwrap();
        let mapping = read_translation(&path);

        println!("Saving {:?}", stem);
        let ser =
            serde_json::to_string_pretty(&mapping).unwrap();
        let output = format!("hes-new/locales/{stem}.json");
        std::fs::write(&output, ser).unwrap_or_else(|_| {
            panic!("Couldn't write file: {:?}", &output)
        });
    }
}
