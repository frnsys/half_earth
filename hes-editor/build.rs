use std::{env, fs, io::Write, path::Path};

fn main() {
    let contents = fs::read_to_string("help.md")
        .unwrap_or_else(|_| panic!("Couldn't read help file"));
    println!("{}", markdown::to_html("## Hello, *world*!"));

    let html = markdown::to_html(&contents);

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("help_text.rs");
    let mut file = fs::File::create(&dest_path).unwrap();
    writeln!(
        file,
        "pub const HTML_CONTENT: &str = r#\"{}\"#;",
        html
    )
    .unwrap();
}
