use base64::{Engine, prelude::BASE64_STANDARD};
use miniz_oxide::{deflate::compress_to_vec, inflate::decompress_to_vec};
use std::{env, sync::LazyLock};

use hes_engine::{ByproductMap, Flag, ProjectType, State};

pub static DEBUG: LazyLock<DebugOpts> = LazyLock::new(DebugOpts::default);

#[derive(Debug)]
pub enum DebugView {
    Plan,
    Regions,
    Parliament,
    Stats,
    World,
    Report,
    GameOver,
    GameWin,
}

pub struct DebugOpts {
    /// Directly open the editor.
    pub open_editor: bool,

    /// Skip all events (region and project updates still trigger).
    pub skip_events: bool,

    /// Skip the tutorial.
    pub skip_tutorial: bool,

    /// Start with all projects unlocked.
    pub show_all_projects: bool,

    /// Start with all processes unlocked.
    pub show_all_processes: bool,

    /// Make years in the world event view advance very quickly.
    pub fast_years: bool,

    /// Always skip the world event view.
    pub always_skip_world: bool,

    /// Start with a lot of political capital.
    pub very_popular: bool,

    /// Start building a project of each type.
    pub with_projects: bool,

    /// An initial view to start with.
    pub view: Option<DebugView>,

    /// Start with parliament suspended.
    pub parliament_suspended: bool,

    /// Start ready to win.
    pub pre_win: bool,

    /// Start ready to lose.
    pub pre_lose: bool,

    /// Start with a production shortage.
    pub production_shortage: bool,

    /// Start with a feedstock shortage.
    pub feedstock_shortage: bool,

    /// Start with a seceded region.
    pub region_seceded: bool,

    /// State to load.
    pub state: Option<State>,
}
impl DebugOpts {
    pub fn apply(&self, state: &mut State) {
        if let Some(use_state) = &self.state {
            *state = use_state.clone();
        }

        if self.parliament_suspended {
            state.flags.push(Flag::ParliamentSuspended);
        }
        if self.very_popular {
            state.political_capital = 500;
        } else if self.pre_lose {
            state.political_capital = 0;
        } else if self.pre_win {
            state.emissions.update(ByproductMap {
                co2: 0.,
                ch4: 0.,
                n2o: 0.,
                biodiversity: 0.,
            });
            state.world.extinction_rate = 0.;
            state.world.temperature = 0.;
        }
        if self.region_seceded {
            let region = state.world.regions.get_mut(0).unwrap();
            region.seceded = true;
        }
        if self.feedstock_shortage {
            state.world.feedstock_reserves.oil = 0.;
        }
        if self.production_shortage {
            state.world.per_capita_demand[0].modifier.electricity = 10.;
        }

        if self.with_projects {
            let policy = state
                .world
                .projects
                .iter()
                .find(|p| p.kind == ProjectType::Policy)
                .map(|p| p.id)
                .unwrap();
            state.start_project(&policy);

            let research = state
                .world
                .projects
                .iter()
                .find(|p| p.kind == ProjectType::Research)
                .map(|p| p.id)
                .unwrap();
            state.start_project(&research);
            state.set_project_points(&research, 20);

            let infrastructure = state
                .world
                .projects
                .iter()
                .find(|p| p.kind == ProjectType::Initiative)
                .map(|p| p.id)
                .unwrap();
            state.start_project(&infrastructure);
            state.set_project_points(&infrastructure, 20);
        }
    }
}

impl Default for DebugOpts {
    /// Initialize debug options from env variables.
    fn default() -> Self {
        let d = env::var("DEBUG").unwrap_or_default();
        let debug: Vec<_> = d.split(',').collect();

        let mut view = env::var("DEBUG_VIEW")
            .map(|view| match view.as_str() {
                "Plan" => Some(DebugView::Plan),
                "Regions" => Some(DebugView::Regions),
                "Govt" => Some(DebugView::Parliament),
                "Stats" => Some(DebugView::Stats),
                "World" => Some(DebugView::World),
                "Report" => Some(DebugView::Report),
                "GameOver" => Some(DebugView::GameOver),
                "GameWin" => Some(DebugView::GameWin),
                _ => None,
            })
            .ok()
            .flatten();
        let state = {
            if cfg!(not(target_arch = "wasm32")) {
                env::var("DEBUG_STATE")
                    .map(|path| match std::fs::read_to_string(&path) {
                        Err(err) => {
                            eprintln!("Failed to read state file: {path:?}");
                            eprintln!("{err}");
                            None
                        }
                        Ok(ser) => Some(deserialize_state(&ser)),
                    })
                    .ok()
                    .flatten()

            // Not supported on web
            } else {
                None
            }
        };

        // If a debug state is provided and a debug view
        // isn't specified, go straight to the plan view.
        if state.is_some() && view.is_none() {
            view = Some(DebugView::Plan);
        }

        Self {
            open_editor: debug.contains(&"EDITOR"),
            skip_events: debug.contains(&"SKIP_EVENTS"),
            skip_tutorial: debug.contains(&"SKIP_TUTORIAL"),
            show_all_projects: debug.contains(&"ALL_PROJECTS"),
            show_all_processes: debug.contains(&"ALL_PROCESSES"),
            with_projects: debug.contains(&"WITH_PROJECTS"),
            fast_years: debug.contains(&"FAST_YEARS"),
            always_skip_world: debug.contains(&"SKIP_WORLD"),
            very_popular: debug.contains(&"VERY_POPULAR"),
            parliament_suspended: debug.contains(&"SUSPENDED"),
            pre_win: debug.contains(&"PRE_WIN"),
            pre_lose: debug.contains(&"PRE_LOSE"),
            production_shortage: debug.contains(&"PRODUCTION_SHORTAGE"),
            feedstock_shortage: debug.contains(&"FEEDSTOCK_SHORTAGE"),
            region_seceded: debug.contains(&"SECEDED"),
            view,
            state,
        }
    }
}

pub fn serialize_state(state: &State) -> Option<String> {
    serde_json::to_string(state)
        .map(|json| {
            let compressed = compress_to_vec(json.as_bytes(), 6);
            let b64 = BASE64_STANDARD.encode(compressed);
            b64.as_bytes()
                .chunks(76)
                .map(|c| std::str::from_utf8(c).unwrap())
                .collect::<Vec<_>>()
                .join("\n")
        })
        .ok()
}

pub fn deserialize_state(b64: &str) -> State {
    let compressed = BASE64_STANDARD.decode(b64.replace("\n", "")).unwrap();
    let decompressed = decompress_to_vec(&compressed).unwrap();
    let str = String::from_utf8(decompressed).unwrap();
    serde_json::from_str(&str).unwrap()
}
