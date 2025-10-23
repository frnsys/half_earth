use glob::glob;
use serde::Deserialize;
use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
    process::Command,
};
use xxhash_rust::xxh3::xxh3_64;

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
        if !row.english.is_empty() && !row.translation.is_empty() {
            // Note that we treat "-" as a deliberate indicator
            // of an empty translation.
            let translation = if row.translation == "-" {
                "".to_string()
            } else {
                row.translation.trim().to_string()
            };
            mapping.insert(row.english.trim().to_string(), translation);
        }
    }
    mapping
}

fn compile_translations() -> impl Iterator<Item = (String, HashMap<String, String>)> {
    glob("translations/*.csv").unwrap().map(|entry| {
        let path = entry.unwrap();
        let stem = path.file_stem().unwrap().to_str().unwrap();
        let mapping = read_translation(&path);
        (stem.to_string(), mapping)
    })
}

fn main() {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    // Translations
    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("locales.rs");
    let mut map = phf_codegen::Map::new();

    for (lang, trans) in compile_translations() {
        let mut locale = phf_codegen::Map::new();
        for (key, val) in trans {
            locale.entry(xxh3_64(key.as_bytes()), format!("{val:?}"));
        }
        map.entry(lang.to_string(), locale.build().to_string());
    }
    fs_err::write(
        &out,
        format!(
            "pub static LOCALES: phf::Map<&'static str, phf::Map<u64, &'static str>> = {};",
            map.build()
        ),
    )
    .unwrap();

    // For wasm, build JS
    let target = env::var("TARGET").unwrap_or_default();
    if target == "wasm32-unknown-unknown" {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

        // Prefer setting current_dir instead of "cd && ..."
        let status = Command::new("./build.sh")
            .current_dir(format!("{manifest_dir}/assets/js"))
            .status()
            .expect("failed to spawn script");

        if !status.success() {
            panic!("Failed to build JS");
        }
    }
}
