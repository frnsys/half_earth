use glob::glob;
use std::{env, path::PathBuf};

fn get_fnames(pat: &str) -> Vec<String> {
    glob(pat)
        .unwrap()
        .into_iter()
        .map(|entry| {
            let path = entry.unwrap();
            format!(
                "{:?}",
                path.display()
                    .to_string()
                    .replace("assets/", "images/")
            )
        })
        .collect()
}

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap())
        .join("sharing.rs");

    let wins = get_fnames("assets/sharing/win/*.jpg");
    let lose = get_fnames("assets/sharing/lose/generic/*.jpg");
    let coup = get_fnames("assets/sharing/lose/coup/*.jpg");
    let death = get_fnames("assets/sharing/lose/death/*.jpg");
    fs_err::write(
        &out,
        format!(
            concat!(
                "static WIN: &[&'static str] = &[{}];\n",
                "static LOSE: &[&'static str] = &[{}];\n",
                "static COUP: &[&'static str] = &[{}];\n",
                "static DEATH: &[&'static str] = &[{}];\n"
            ),
            wins.join(", "),
            lose.join(", "),
            coup.join(", "),
            death.join(", "),
        ),
    )
    .unwrap();
}
