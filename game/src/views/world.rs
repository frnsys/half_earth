use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    sync::Arc,
};

use egui::Color32;
use egui_taffy::TuiBuilderLogic;
use enum_map::EnumMap;
use hes_engine::{
    Diff, EventPhase, ICON_EVENTS, IconEvent, Id, Output, State, Update as EngineUpdate,
};
use rust_i18n::t;
use web_time::Instant;

use crate::{
    audio,
    climate::Climate,
    consts,
    debug::DEBUG,
    display::{Icon, icons},
    parts::{button, center_center, fill_bar, set_full_bg_image_tinted},
    state::{GameState, StateExt},
    views::{
        events::{Events, Updates},
        globe::GlobeView,
    },
};

pub struct WorldEvents {
    phase: Subphase,
    year_timer: Timer,
    toasts: VecDeque<Toast>,
    disasters: Vec<Disaster>,
    events: Events,
    updates: Updates,
    skipping: bool,
    globe: GlobeView,
    climate: Climate,
    tgav: f32,
}
impl WorldEvents {
    pub fn new(state: &mut GameState, context: &Arc<three_d::context::Context>) -> Self {
        state.ui.cycle_start_snapshot(&state.core);

        let good = state.things_are_good();
        if good {
            audio::soundtrack(audio::Track::ReportGood);
        } else {
            audio::soundtrack(audio::Track::ReportBad);
        }

        let events = StateExt::roll_events(&mut state.core, EventPhase::WorldStart);

        let mut climate = Climate::new(state.ui.start_year);
        if !state.ui.emissions.is_empty() {
            climate.set_emissions_data(state.ui.emissions.clone());
        }

        Self {
            phase: Subphase::Events,
            year_timer: Timer::new(),
            skipping: DEBUG.always_skip_world,
            events: Events::new(events, state),
            globe: GlobeView::new(360, 250., context),
            toasts: VecDeque::default(),
            disasters: Vec::default(),
            updates: Updates::new(vec![], state),
            climate,
            tgav: state.world.temperature,
        }
    }

    pub fn is_done(&self) -> bool {
        self.phase == Subphase::Done
    }

    pub fn render(&mut self, ui: &mut egui::Ui, state: &mut GameState) {
        let tint = warming_colour(self.tgav);
        set_full_bg_image_tinted(
            ui,
            hes_images::background_image("globe.png"),
            egui::vec2(1048., 702.),
            tint,
        );

        center_center(ui, "globe", |tui| {
            tui.ui(|ui| ui.add(&mut self.globe));
        });

        match &self.phase {
            Subphase::Events => {
                self.events.render(ui, state);
                if self.events.is_finished {
                    self.next_phase(state);
                }
            }
            Subphase::Updates => {
                self.updates.render(ui, state);
                if self.updates.is_finished {
                    self.next_phase(state);
                }
            }
            Subphase::Disasters => {
                let p = self.year_timer.p_elapsed();
                self.tick_disasters(state, p);
                self.render_toasts(ui);

                let width = 320.;
                let offset = ui.available_width() / 2. - width / 2.;
                ui.horizontal(|ui| {
                    ui.add_space(offset);
                    ui.add(
                        fill_bar((width, 8.), p)
                            .back_color(Color32::TRANSPARENT)
                            .fill_color(Color32::WHITE),
                    );
                });

                if p >= 1. {
                    self.next_phase(state);
                }
            }
            Subphase::ComputeTgav => {
                if let Some(tgav) = self.climate.tgav(state.world.year + 1) {
                    self.tgav = tgav;
                    self.next_phase(state);
                }
            }
            Subphase::StepYear => self.next_phase(state),
            Subphase::Done => (),
        }

        egui::Area::new(egui::Id::new("world-skip"))
            .order(egui::Order::Foreground)
            .anchor(egui::Align2::RIGHT_BOTTOM, egui::Vec2::new(-10., -10.))
            .show(ui.ctx(), |ui| {
                if ui.add(button(t!("Skip"))).clicked() {
                    self.skipping = true;
                    self.year_timer.skipping = true;
                }
            });
    }

    fn render_toasts(&self, ui: &mut egui::Ui) {
        egui::Area::new(egui::Id::new("world-toasts"))
            .order(egui::Order::Foreground)
            .anchor(egui::Align2::CENTER_BOTTOM, egui::Vec2::new(0., -10.))
            .show(ui.ctx(), |ui| {
                let n = self.toasts.len();
                for (i, toast) in self.toasts.iter().enumerate() {
                    let opacity = (i + 1) as f32 / n as f32;
                    ui.scope(|ui| {
                        ui.set_opacity(opacity);
                        egui::Frame::NONE.corner_radius(3).show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.add(toast.icon.size(20.));
                                ui.label(egui::RichText::new(&toast.desc).size(18.));
                            });
                        });
                    });
                }
            });
    }

    fn tick_disasters(&mut self, state: &mut GameState, progress: f32) {
        // Trigger any scheduled disasters.
        // Get events scheduled for at or earlier than the provided time.
        let popped: Vec<_> = self
            .disasters
            .extract_if(.., |ev| ev.when <= progress)
            .collect();

        let mut occurring = vec![];
        for ev_meta in popped {
            if let Disaster {
                event_id,
                region: Some((region_id, region_name, region_idx)),
                ..
            } = ev_meta
            {
                if let Some(ev) = ICON_EVENTS.get(&event_id) {
                    self.globe
                        .show_event(region_idx, icons::disaster_icon(&ev.icon), ev.intensity);
                    occurring.push((ev, event_id, region_id, region_name));
                }
            }
        }

        for (ev, event_id, region_id, region_name) in occurring {
            let region_events = state.ui.annual_region_events.entry(region_id).or_default();
            region_events.push(ev.clone());
            StateExt::apply_disaster(&mut state.core, ev, &event_id, &region_id);
            self.toasts.push_back(Toast::new(ev, &region_name));
            while self.toasts.len() > 3 {
                self.toasts.pop_front();
            }
        }
    }

    fn next_phase(&mut self, state: &mut GameState) {
        let mut next = match self.phase {
            Subphase::Disasters => Subphase::ComputeTgav,
            Subphase::ComputeTgav => Subphase::StepYear,
            Subphase::StepYear => Subphase::Updates,
            Subphase::Updates => Subphase::Events,
            Subphase::Events => Subphase::Disasters,
            Subphase::Done => Subphase::Done,
        };

        if next == Subphase::ComputeTgav {
            // Update emissions to compute the temp anomaly.
            let emissions = get_emissions(&state.core);
            self.climate.add_emissions(emissions);
            state.ui.emissions = self.climate.emissions_data();
        }

        if next == Subphase::StepYear {
            // Advance the year.
            let step_updates = state.step_year(self.tgav);
            let completed_projects = step_updates.iter().filter_map(|update| match update {
                EngineUpdate::Project { id } => Some(id),
                EngineUpdate::Policy { id } => Some(id),
                _ => None,
            });
            state
                .ui
                .cycle_start_state
                .completed_projects
                .extend(completed_projects);

            self.updates = Updates::new(step_updates, &state.core);
        }

        if next == Subphase::Updates {
            if self.updates.is_finished || self.skipping {
                next = Subphase::Events;
            }
        }

        if next == Subphase::Events {
            let evs = StateExt::roll_events(&mut state.core, EventPhase::WorldMain);
            for event in &evs {
                state.ui.world_events.push(event.clone());
            }

            if evs.is_empty() || self.skipping {
                next = Subphase::Disasters;
            } else {
                self.events.replace(evs, &state.core);
            }
        }

        // This phase is never skipped.
        if next == Subphase::Disasters {
            self.year_timer.reset();
            let cur_year = state.world.year;
            let cycle_start_year = state.ui.cycle_start_state.year;
            if cur_year > cycle_start_year && cur_year % 5 == 0 {
                state.finish_cycle();

                // This has to happen before we enter the report
                // phase and calculate changes
                // so the upgrades' effects are taken into account.
                state.core.upgrade_projects(&mut state.ui.queued_upgrades);
                // Apply process mix changes.
                state
                    .core
                    .update_processes(&mut state.ui.process_mix_changes);

                let changes = state.ui.session_start_state.diff(&state.core);
                let mixes = {
                    let mut mixes: EnumMap<Output, BTreeMap<String, usize>> = EnumMap::default();
                    for process in state.world.processes.iter() {
                        if process.mix_share > 0 {
                            mixes[process.output]
                                .insert(process.name.to_string(), process.mix_share);
                        }
                    }
                    mixes
                };
                state.ui.change_history.push((cur_year, changes));
                state.ui.process_mix_history.push((cur_year, mixes));

                next = Subphase::Done;
            } else {
                let region_lookup: HashMap<_, _> = state
                    .world
                    .regions
                    .iter()
                    .enumerate()
                    .map(|(i, region)| (region.id, i))
                    .collect();
                let evs: Vec<_> = StateExt::roll_events(&mut state.core, EventPhase::Icon)
                    .into_iter()
                    .map(|ev| Disaster {
                        event_id: ev.id,
                        region: ev.region.clone().map(|(id, name)| {
                            let idx = region_lookup.get(&id).unwrap();
                            (id, name, *idx)
                        }),
                        when: fastrand::f32(),
                    })
                    .collect();
                self.disasters = evs;
            }
        }

        self.phase = next;
    }
}

struct Toast {
    icon: Icon,
    desc: String,
}
impl Toast {
    fn new(ev: &IconEvent, region_name: &str) -> Self {
        Toast {
            icon: icons::disaster_icon(&ev.icon),
            desc: t!(
                "%{disaster} in %{region}",
                disaster = t!(&ev.name),
                region = t!(region_name)
            )
            .to_string(),
        }
    }
}

struct Timer {
    start: Instant,
    target: f32,
    skipping: bool,
}
impl Timer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            target: if DEBUG.fast_years {
                500.
            } else {
                consts::MS_PER_YEAR
            },
            skipping: false,
        }
    }

    fn p_elapsed(&mut self) -> f32 {
        let duration = self.start.elapsed();
        let target = if self.skipping { 10. } else { self.target };
        (duration.as_millis() as f32 / target).min(1.)
    }

    fn reset(&mut self) {
        self.start = Instant::now();
    }
}

#[derive(Debug)]
struct Disaster {
    event_id: Id,
    region: Option<(Id, String, usize)>,

    /// When in the year the event occurs.
    when: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Subphase {
    Events,
    Disasters,
    ComputeTgav,
    StepYear,
    Updates,
    Done,
}

fn warming_colour(mut temp: f32) -> Color32 {
    if temp <= 0. {
        temp = 0.1;
    }

    let mut r = 250;
    let mut g = (255. / temp).round();
    let mut b = (230. / temp).round();
    if g >= 255. {
        g = 255.;
        r = 240;
    }
    if b >= 255. {
        b = 255.;
        r = 240;
    }
    Color32::from_rgb(r, g.round() as u8, b.round() as u8)
}

fn get_emissions(state: &State) -> HashMap<&'static str, f64> {
    // Set an upper cap to the amount of emissions we pass to hector,
    // because very large numbers end up breaking it.
    let emissions_factor = (consts::MAX_EMISSIONS / state.emissions.as_gtco2eq().abs()).min(1.0);

    let (co2, ch4, n2o) = state.emissions.for_hector();

    let mut emissions = HashMap::default();
    emissions.insert("ffi_emissions", (co2 * emissions_factor) as f64);
    emissions.insert("CH4_emissions", (ch4 * emissions_factor) as f64);
    emissions.insert("N2O_emissions", (n2o * emissions_factor) as f64);
    emissions
}
