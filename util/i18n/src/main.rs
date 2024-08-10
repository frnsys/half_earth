use glob::glob;
use hes_engine::*;
use hes_game::{display::AsText, Badge, Var, LOCALES};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::char,
    error::{Error, ErrorKind, ParseError},
    multi::many1,
    sequence::{delimited, preceded},
    Err,
    IResult,
};
use serde::Deserialize;
use std::{collections::HashMap, path::Path};
use strum::IntoEnumIterator;

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

// Nabbed from <https://gitlab.com/getreu/parse-hyperlinks>
fn take_until_unbalanced(
    opening_bracket: char,
    closing_bracket: char,
) -> impl Fn(&str) -> IResult<&str, &str> {
    move |i: &str| {
        let mut index = 0;
        let mut bracket_counter = 0;
        while let Some(n) = &i[index..]
            .find(&[opening_bracket, closing_bracket, '\\'][..])
        {
            index += n;
            let mut it = i[index..].chars();
            match it.next() {
                Some(c) if c == '\\' => {
                    // Skip the escape char `\`.
                    index += '\\'.len_utf8();
                    // Skip also the following char.
                    if let Some(c) = it.next() {
                        index += c.len_utf8();
                    }
                }
                Some(c) if c == opening_bracket => {
                    bracket_counter += 1;
                    index += opening_bracket.len_utf8();
                }
                Some(c) if c == closing_bracket => {
                    // Closing bracket.
                    bracket_counter -= 1;
                    index += closing_bracket.len_utf8();
                }
                // Can not happen.
                _ => unreachable!(),
            };
            // We found the unmatched closing bracket.
            if bracket_counter == -1 {
                // We do not consume it.
                index -= closing_bracket.len_utf8();
                return Ok((&i[index..], &i[0..index]));
            };
        }

        if bracket_counter == 0 {
            Ok(("", i))
        } else {
            Err(Err::Error(Error::from_error_kind(
                i,
                ErrorKind::TakeUntil,
            )))
        }
    }
}

fn find_text(input: &str) -> IResult<&str, &str> {
    preceded(
        tag("i18n::t"),
        delimited(
            char('('),
            take_until_unbalanced('(', ')'),
            char(')'),
        ),
    )(input)
}

fn find_start(input: &str) -> IResult<&str, &str> {
    take_until("i18n::t")(input)
}

fn extract_strings(from: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(from).unwrap();
    let (_rest, matches) =
        many1(preceded(find_start, find_text))(&contents)
            .unwrap();

    let mut literals = vec![];
    let mut other = vec![];
    for m in matches {
        // String literals
        if m.starts_with('"') {
            literals
                .push(m[1..m.len() - 1].replace("\\'", "'"));

        // Raw string literals
        } else if m.starts_with("r#") {
            literals
                .push(m[3..m.len() - 2].replace("\\'", "'"));

        // Skip macro invocations
        } else if !m.starts_with("$") {
            other.push(m);
        }
    }

    literals.sort_unstable();
    literals.dedup();

    {
        let mut add = |s: &str| {
            literals.push(s.to_string());
        };

        let state = State::new(World::default());
        for npc in state.npcs.iter() {
            add(&npc.name);
            add(&npc.flavor.description);
            add(&npc.flavor.effects);
            add(&npc.flavor.likes);
        }
        for industry in state.world.industries.iter() {
            add(&industry.name);
            add(&industry.flavor.description);
        }
        for process in state.world.processes.iter() {
            add(&process.name);
            add(&process.flavor.description);
        }
        for project in state.world.projects.iter() {
            add(&project.name);
            add(&project.flavor.description);
            for dialogue in &project.flavor.outcomes {
                for line in &dialogue.lines {
                    add(&line.text);
                    if let Some(
                        flavor::DialogueNext::Responses(
                            responses,
                        ),
                    ) = &line.next
                    {
                        for resp in responses {
                            add(&resp.text);
                        }
                    }
                }
            }
        }
        for region in state.world.regions.iter() {
            add(&region.name);
        }
        for event in state.world.events.iter() {
            add(&event.name);
            add(&event.flavor.arc);
            for line in &event.flavor.dialogue.lines {
                add(&line.text);
                if let Some(flavor::DialogueNext::Responses(
                    responses,
                )) = &line.next
                {
                    for resp in responses {
                        add(&resp.text);
                    }
                }
            }
        }
        for output in Output::iter() {
            add(&output.title());
            add(&output.lower());
        }
        for feedstock in Feedstock::iter() {
            add(&feedstock.title());
            add(&feedstock.lower());
        }
        for kind in ProjectType::iter() {
            add(&kind.title());
            add(&kind.lower());
        }
        for group in Group::iter() {
            add(&group.to_string());
        }
        for byproduct in Byproduct::iter() {
            add(&byproduct.title());
            add(&byproduct.lower());
        }
        for resource in Resource::iter() {
            add(&resource.title());
            add(&resource.lower());
        }
        for income in Income::iter() {
            add(&income.title());
            add(&income.lower());
        }
        for flag in Flag::iter() {
            add(&flag.to_string());
        }
        for feat in ProcessFeature::iter() {
            add(&feat.title());
        }
        for lat in Latitude::iter() {
            add(&lat.lower());
        }
        for badge in Badge::iter() {
            add(&badge.to_string());
        }
        for var in Var::iter() {
            add(&var.title());
        }
        for rel in NPCRelation::iter() {
            add(&rel.to_string());
        }
        add("Friendly");

        for speaker in flavor::Speaker::iter() {
            add(&speaker.to_string());
        }
        // for locale in LOCALES {
        //     add(&locale.name);
        // }

        for v in ["makes", "causes", "uses"] {
            add(&v);
        }
    }

    literals.dedup();

    // Uncomment this to see variables and expressions
    // passed to the translate macro.
    //
    // println!("Variables and Expressions:");
    // other.sort_unstable();
    // other.dedup();
    // for m in other {
    //     add(&m);
    // }

    literals.into_iter().filter(|lit| !lit.is_empty()).collect()
}

fn main() {
    let expected = extract_strings("/tmp/expanded");
    for entry in glob("util/i18n/transl/*.csv").unwrap() {
        let path = entry.unwrap();
        let stem = path.file_stem().unwrap().to_str().unwrap();

        let mapping = read_translation(&path);

        let mut missing = vec![];
        let mut extra = vec![];
        for key in &expected {
            if !mapping.contains_key(key) {
                missing.push(key);
            }
        }
        for key in mapping.keys() {
            if !expected.contains(key) {
                extra.push(key);
            }
        }

        if !missing.is_empty() || !extra.is_empty() {
            println!(
                "============{}============",
                path.display()
            );

            if !missing.is_empty() {
                println!(
                    "  --[{}] MISSING--------------",
                    path.display()
                );
                for key in &missing {
                    println!("    {}", key);
                }
            }

            if !extra.is_empty() {
                println!(
                    "  --[{}] EXTRA--------------",
                    path.display()
                );
                for key in &extra {
                    println!("    {}", key);
                }
            }

            println!(
                "  Missing: {}, Extra {}",
                missing.len(),
                extra.len()
            );
        }

        println!("Saving {:?}", stem);
        let ser =
            serde_json::to_string_pretty(&mapping).unwrap();
        let output =
            format!("hes-game/public/assets/lang/{stem}.json");
        std::fs::write(&output, ser).unwrap_or_else(|_| {
            panic!("Couldn't write file: {:?}", &output)
        });
    }
}
