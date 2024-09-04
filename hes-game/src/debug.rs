use leptos::*;
use leptos_router::{use_query, Params};

#[derive(Debug, Clone, Params, PartialEq)]
struct QueryParams {
    debug: Option<String>,
}

#[derive(Default, Debug)]
pub struct DebugOpts {
    pub skip_events: bool,
    pub skip_tutorial: bool,
    pub show_all_projects: bool,
    pub show_all_processes: bool,
    pub fast_years: bool,
    pub skip_to_planning: bool,
    pub always_skip_world: bool,
    pub check_events: bool,
    pub very_popular: bool,
    pub no_globe: bool,
    pub no_hector: bool,
}

pub fn get_debug_opts() -> DebugOpts {
    let params = use_query::<QueryParams>();
    if let Some(debug) = params.with_untracked(|params| {
        params.as_ref().ok().and_then(|q| q.debug.clone())
    }) {
        let opts: Vec<_> = debug.split(",").collect();
        let debug_all = opts.contains(&"all");
        DebugOpts {
            skip_events: opts.contains(&"skip-events")
                || debug_all,
            skip_tutorial: opts.contains(&"skip-tutorial")
                || debug_all,
            fast_years: opts.contains(&"fast-years")
                || debug_all,
            show_all_projects: opts.contains(&"all-projects")
                || debug_all,
            show_all_processes: opts.contains(&"all-processes")
                || debug_all,
            skip_to_planning: opts
                .contains(&"skip-to-planning")
                || debug_all,
            always_skip_world: opts
                .contains(&"always-skip-world")
                || debug_all,
            very_popular: opts.contains(&"i-am-the-state")
                || debug_all,
            check_events: opts.contains(&"check-events"),
            no_globe: opts.contains(&"no-globe"),
            no_hector: opts.contains(&"no-hector"),
        }
    } else {
        DebugOpts::default()
    }
}
