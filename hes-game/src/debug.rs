use leptos::*;
use leptos_router::{use_query, Params};

#[derive(Debug, Clone, Params, PartialEq)]
struct QueryParams {
    debug: Option<String>,
}

#[derive(Default, Debug)]
pub struct DebugOpts {
    pub skip_tutorial: bool,
    pub show_all_projects: bool,
    pub show_all_processes: bool,
    pub fast_years: bool,
    pub skip_to_planning: bool,
    pub check_events: bool,
}

pub fn get_debug_opts() -> DebugOpts {
    let params = use_query::<QueryParams>();
    if let Some(debug) = params.with_untracked(|params| {
        params.as_ref().ok().and_then(|q| q.debug.clone())
    }) {
        let opts: Vec<_> = debug.split(",").collect();
        let debug_all = opts.contains(&"all");
        DebugOpts {
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
            check_events: opts.contains(&"check-events"),
        }
    } else {
        DebugOpts::default()
    }
}
