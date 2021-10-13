use crate::kinds::{Resource, Output};
use crate::production::ProcessFeature;
use crate::variables::{WorldVariable, LocalVariable};
use crate::projects::Status as ProjectStatus;

#[derive(Debug, Copy, Clone)]
pub enum Flag {
    IsMalthusian,
    IsFALC,
    IsHES
}

#[derive(Debug, Copy, Clone)]
pub enum Condition {
    LocalVariable(LocalVariable, Comparator, f32),
    WorldVariable(WorldVariable, Comparator, f32),
    ProcessMixShare(usize, Comparator, f32),
    ProcessMixShareFeature(ProcessFeature, Comparator, f32),
    Resource(Resource, Comparator, f32),
    ResourceDemandGap(Resource, Comparator, f32),
    OutputDemandGap(Output, Comparator, f32),
    ProjectStatus(usize, ProjectStatus),
    Flag(Flag),
    RunsPlayed(Comparator, usize),
}

#[derive(Debug, Copy, Clone)]
pub enum Comparator {
    Less,
    LessEqual,
    Equal,
    NotEqual,
    GreaterEqual,
    Greater
}
