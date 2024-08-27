use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    Collection,
    Event,
    Flag,
    HasId,
    Process,
    Project,
    State,
    World,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Change {
    Simple(String),
    Nested(String, Vec<Change>),
}
impl Change {
    fn print(&self, depth: usize) -> String {
        const INDENT: &str = "  ";
        match self {
            Change::Simple(s) => {
                format!("{}{s}", INDENT.repeat(depth))
            }
            Change::Nested(field, changes) => {
                let lines = changes
                    .iter()
                    .map(|change| change.print(depth + 1))
                    .collect::<Vec<_>>();
                format!(
                    "{}{field}:\n{}",
                    INDENT.repeat(depth),
                    lines.join("\n")
                )
            }
        }
    }
}

impl Display for Change {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.print(0))
    }
}

pub trait Diff {
    fn diff(&self, other: &Self) -> Vec<Change>;
}

pub trait DiffLabel {
    fn label(&self) -> String;
}

macro_rules! diffs {
    ($a:ident, $b:ident, { $($($path:ident).+),* }) => {{
        let mut changes = vec![];
        $(if $a.$($path).+ != $b.$($path).+ {
            let change = format!(
                concat!(stringify!($($path).+), ": {:?} -> {:?}"),
                $a.$($path).+,
                $b.$($path).+
            );
            changes.push(Change::Simple(change));
        })*
        changes
    }}
}

macro_rules! diff_methods {
    ($a:ident, $b:ident, { $($($path:ident).+),* }) => {{
        let mut changes = vec![];
        $(if $a.$($path).+() != $b.$($path).+() {
            let change = format!(
                concat!(stringify!($($path).+), ": {:?} -> {:?}"),
                $a.$($path).+(),
                $b.$($path).+()
            );
            changes.push(Change::Simple(change));
        })*
        changes
    }}
}

macro_rules! diff_vecs {
    ($a:ident, $b:ident, { $($($path:ident).+),* }) => {{
        let mut changes = vec![];
        $(
            let diffs = $a.$($path).+.diff(&$b.$($path).+);
            if !diffs.is_empty() {
                let label = stringify!($($path).+);
                changes.push(Change::Nested(label.to_string(), diffs));
            }
        )*
        changes
    }}
}

impl<T: DiffLabel + PartialEq> Diff for Vec<T> {
    fn diff(&self, other: &Self) -> Vec<Change> {
        let mut changes = vec![];
        for item in other {
            if !self.contains(item) {
                changes.push(Change::Simple(format!(
                    "Added {}",
                    item.label()
                )))
            }
        }

        for item in self {
            if !other.contains(item) {
                changes.push(Change::Simple(format!(
                    "Removed {}",
                    item.label()
                )))
            }
        }

        changes
    }
}

impl Diff for State {
    fn diff(&self, other: &Self) -> Vec<Change> {
        let mut changes = diffs!(self, other, {
            political_capital, protected_land });
        changes.extend(diff_methods!(self, other, {
            emissions.as_gtco2eq
        }));
        changes
            .extend(diff_vecs!(self, other, { flags, events }));
        changes.extend(self.world.diff(&other.world));
        changes
    }
}
impl Diff for World {
    fn diff(&self, other: &Self) -> Vec<Change> {
        let mut changes = diffs!(self, other, {
            extinction_rate, temperature });

        changes.extend(diff_methods!(self, other, { outlook }));

        let diffs = self.projects.diff(&other.projects);
        if !diffs.is_empty() {
            changes
                .push(Change::Nested("projects".into(), diffs));
        }

        let diffs = self.processes.diff(&other.processes);
        if !diffs.is_empty() {
            changes.push(Change::Nested(
                "processes".into(),
                diffs,
            ));
        }

        changes
    }
}

impl Diff for Project {
    fn diff(&self, other: &Self) -> Vec<Change> {
        diffs!(self, other, { status, level, active_outcome })
    }
}
impl DiffLabel for Project {
    fn label(&self) -> String {
        self.name.clone()
    }
}

impl Diff for Process {
    fn diff(&self, other: &Self) -> Vec<Change> {
        diffs!(self, other, { mix_share })
    }
}
impl DiffLabel for Process {
    fn label(&self) -> String {
        self.name.clone()
    }
}

// NOTE: Here we assume the collection has no add/remove
// changes, just the properties of the items can change.
impl<T: HasId + Diff + DiffLabel> Diff for Collection<T> {
    fn diff(&self, other: &Self) -> Vec<Change> {
        self.values()
            .iter()
            .zip(other.values())
            .filter_map(|(a, b)| {
                let diffs = a.diff(b);
                if diffs.is_empty() {
                    None
                } else {
                    Some(Change::Nested(a.label(), diffs))
                }
            })
            .collect()
    }
}

impl DiffLabel for Flag {
    fn label(&self) -> String {
        format!(r#""{:?}""#, self)
    }
}
impl DiffLabel for Event {
    fn label(&self) -> String {
        format!("{:?}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::Status;

    use super::*;

    #[test]
    fn test_diff() {
        let state_a = State::default();
        let mut state_b = State::default();
        state_b.emissions.co2 += 1200000000000.;
        state_b.political_capital = 500;
        state_b.protected_land *= 2.;
        state_b.flags.push(Flag::EcosystemModeling);
        state_b.world.projects.by_idx_mut(5).status =
            Status::Finished;
        let diffs = state_a.diff(&state_b);
        for diff in diffs {
            println!("{}", diff);
        }
    }
}
