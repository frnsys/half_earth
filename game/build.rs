use glob::glob;
use std::{collections::HashMap, env, path::PathBuf};
use xxhash_rust::xxh3::xxh3_64;

fn main() {
    let output = std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    let out = PathBuf::from(env::var("OUT_DIR").unwrap())
        .join("locales.rs");

    let mut map = phf_codegen::Map::new();
    for entry in glob("locales/*.json").unwrap() {
        let path = entry.unwrap();
        let lang = path.file_stem().unwrap().to_str().unwrap();
        let mut locale = phf_codegen::Map::new();

        let data = fs_err::read_to_string(&path).unwrap();
        let trans: HashMap<String, String> =
            serde_json::from_str(&data).unwrap();
        for (key, val) in trans {
            locale.entry(
                xxh3_64(key.as_bytes()),
                format!("{val:?}"),
            );
        }
        map.entry(
            format!("{lang}"),
            locale.build().to_string(),
        );
    }
    fs_err::write(
        &out,
        format!(
            "pub static LOCALES: phf::Map<&'static str, phf::Map<u64, &'static str>> = {};",
            map.build()
        ),
    )
    .unwrap();
}
