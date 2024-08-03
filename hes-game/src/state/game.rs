use super::Points;
use crate::{consts, display, views::DisplayEvent};
use enum_map::EnumMap;
use extend::ext;
use hes_engine::{
    events::{Flag, IconEvent, Phase},
    game::Update,
    kinds::{Feedstock, Output},
    production::Process,
    projects::Status,
    Game,
    Id,
    ProjectType,
};
use std::collections::BTreeMap;

#[ext]
pub impl Game {
    fn things_are_good(&self) -> bool {
        self.world.temperature <= 1.
            || self.world.extinction_rate <= 20.
            || self.state.emissions_gt() <= 0.
    }

    fn emissions_gt(&self) -> String {
        display::emissions(self.state.emissions_gt())
    }

    fn land_use_percent(&self) -> String {
        let usage = self.state.resources_demand.land;
        let total_land =
            self.state.world.starting_resources.land;
        let percent = usage / total_land;
        display::percent(percent, true)
    }

    fn water_use_percent(&self) -> String {
        let usage = self.state.resources_demand.water;
        let total_water = self.state.resources.water;
        let percent = usage / total_water;
        display::percent(percent, true)
    }

    fn temp_anomaly(&self) -> String {
        format!("{:+.1}C", self.state.world.temperature)
    }

    fn energy_pwh(&self) -> String {
        let energy = self.state.demand_for_outputs().energy();
        format!("{}PWh", (display::twh(energy) / 1e3).round())
    }

    fn energy_twh(&self) -> String {
        let energy = self.state.demand_for_outputs().energy();
        format!("{}TWh", display::twh(energy).round())
    }

    fn avg_income_level(&self) -> usize {
        let mut total = 0.;
        for region in self.state.world.regions.iter() {
            let income = region.income_level() as f32
                + 1.
                + region.development;
            total += income;
        }
        let n_regions = self.state.world.regions.len();
        let avg = (total / n_regions as f32).round() as usize;
        avg
    }

    fn avg_habitability(&self) -> f32 {
        let mut total = 0.;
        for region in self.state.world.regions.iter() {
            total += region.habitability();
        }
        let n_regions = self.state.world.regions.len();
        (total / n_regions as f32).round()
    }

    /// Cost for the next point for a project, taking into
    /// account discounts.
    fn next_point_cost(&self, kind: &ProjectType) -> usize {
        let mut discount = 0;
        if *kind == ProjectType::Research {
            if self.state.flags.contains(&Flag::HyperResearch) {
                discount += 1;
            }
            if self.state.is_ally("The Accelerationist") {
                discount += 1;
            }
        }
        0.max(consts::POINT_COST - discount) as usize
    }

    fn player_seats(&self) -> f32 {
        self.state
            .npcs
            .iter()
            .filter(|npc| npc.is_ally())
            .map(|npc| npc.seats)
            .sum()
    }

    fn buy_point(
        &mut self,
        project_id: &Id,
        points: &mut Points,
    ) -> bool {
        let (kind, proj_points) = {
            let project = &self.world.projects[project_id];
            (project.kind, project.points)
        };
        let is_research = kind == ProjectType::Research;
        if proj_points >= consts::MAX_POINTS {
            false
        } else if is_research && points.research > 0 {
            true
        } else {
            let cost = self.next_point_cost(&kind) as isize;
            if cost <= self.state.political_capital {
                self.change_political_capital(-cost);
                match kind {
                    ProjectType::Research => {
                        points.research += 1
                    }
                    ProjectType::Initiative => {
                        points.initiative += 1
                    }
                    _ => (),
                }
                if is_research {
                    points.refundable_research += 1;
                }
                true
            } else {
                false
            }
        }
    }

    fn pay_points(&mut self, project_id: &Id) -> bool {
        // Only policies have points paid all at once,
        // rather than assigned.
        let project = &self.world.projects[project_id];
        let available = self.state.political_capital;
        if project.status == Status::Inactive
            && available >= project.cost as isize
        {
            self.change_political_capital(
                -(project.cost as isize),
            );
            true
        } else {
            false
        }
    }

    fn assign_point(
        &mut self,
        project_id: &Id,
        points: &mut Points,
    ) {
        let (kind, cur_points, status) = {
            let project = &self.world.projects[project_id];
            (project.kind, project.points, project.status)
        };
        let points = match kind {
            ProjectType::Research => &mut points.research,
            ProjectType::Initiative => &mut points.initiative,
            ProjectType::Policy => return,
        };
        if *points > 0 && cur_points < consts::MAX_POINTS {
            self.set_project_points(project_id, cur_points + 1);
            if status != Status::Building {
                self.start_project(project_id);
            }
            *points -= 1;
        }
    }

    fn unassign_points(
        &mut self,
        project_id: &Id,
        points: usize,
    ) {
        let (current_points, status) = {
            let project = &self.world.projects[project_id];
            (project.points, project.status)
        };
        let new_points = current_points - points;
        self.set_project_points(project_id, new_points);
        if status == Status::Building && new_points == 0 {
            self.stop_project(project_id);
        }
    }

    fn pass_policy(&mut self, project_id: &Id) {
        let kind = {
            let project = &self.world.projects[project_id];
            project.kind
        };
        if kind == ProjectType::Policy {
            self.start_project(project_id);
        }
    }

    fn stop_policy(&mut self, project_id: &Id) {
        let (kind, cost) = {
            let project = &self.world.projects[project_id];
            (project.kind, project.cost)
        };

        if kind == ProjectType::Policy {
            self.change_political_capital(cost as isize);
            self.stop_project(project_id);
        }
    }

    fn upgrade_project_x(
        &mut self,
        project_id: &Id,
        is_free: bool,
        queued_upgrades: &mut BTreeMap<Id, bool>,
    ) -> bool {
        let (kind, upgrade) = {
            let project = &self.world.projects[project_id];
            (project.kind, project.next_upgrade())
        };
        if let Some(upgrade) = upgrade {
            let available = self.state.political_capital;
            if is_free || available >= upgrade.cost as isize {
                if !is_free {
                    self.change_political_capital(
                        -(upgrade.cost as isize),
                    );
                }
            }

            match kind {
                // Policies upgraded instantly
                ProjectType::Policy => {
                    self.upgrade_project(project_id);
                }
                _ => {
                    queued_upgrades.insert(*project_id, true);
                }
            }
            true
        } else {
            false
        }
    }

    fn downgrade_project_x(
        &mut self,
        project_id: &Id,
        queued_upgrades: &mut BTreeMap<Id, bool>,
    ) {
        let (kind, prev_upgrade) = {
            let project = &self.world.projects[project_id];
            (project.kind, project.prev_upgrade())
        };

        if let Some(upgrade) = prev_upgrade {
            self.change_political_capital(
                upgrade.cost as isize,
            );
            if kind == ProjectType::Policy {
                self.downgrade_project(project_id);
            } else {
                queued_upgrades.insert(*project_id, false);
            }
        }
    }

    fn roll_events(
        &mut self,
        phase: Phase,
        limit: Option<usize>,
    ) -> Vec<DisplayEvent> {
        let events = self.roll_events_for_phase(phase, limit);

        // Icon events, aka disasters,
        // are handled differently, so we don't
        // apply their effects immediately here.
        if phase != Phase::Icon {
            for ev in &events {
                self.apply_event(
                    ev.id,
                    ev.region.as_ref().map(|(id, _)| *id),
                );
            }
        }

        events
            .into_iter()
            .map(|ev| DisplayEvent::new(ev, &self.state))
            .collect()
    }

    /// If we won the game.
    fn won(&self) -> bool {
        self.state.emissions_gt() <= consts::WIN_EMISSIONS
            && self.state.world.extinction_rate
                <= consts::WIN_EXTINCTION
            && self.state.world.temperature
                <= consts::WIN_TEMPERATURE
    }

    fn game_over(&self) -> bool {
        self.state.game_over
    }

    /// Maximum production share for a process.
    fn process_max_share(&self, process: &Process) -> usize {
        let mut max_share = 1.;
        let demand =
            self.state.demand_for_output(&process.output);

        // Hard-coded limit
        if let Some(limit) = process.limit {
            max_share = (limit / demand).min(1.);
        }

        // Limit based on feedstock supply
        let (feedstock, per_output) = process.feedstock;
        match feedstock {
            Feedstock::Other | Feedstock::Soil => {}
            _ => {
                let feedstock_limit = self.state.feedstocks
                    [feedstock]
                    / per_output;
                let feedstock_max_share =
                    (feedstock_limit / demand).min(1.);
                max_share = max_share.min(feedstock_max_share);
            }
        }

        (max_share * 100. / 5.).floor() as usize
    }

    fn is_planning_year(&self) -> bool {
        self.world.year + 1 % 5 == 0
    }

    fn upgrade_projects(
        &mut self,
        upgrades: &mut BTreeMap<Id, bool>,
    ) {
        // for (id, queued) in self.ui.queued_upgrades.iter_mut() {
        for (id, queued) in upgrades.iter_mut() {
            if *queued {
                *queued = false;
                self.upgrade_project(id);
            }
        }
    }

    fn step_year(&mut self) -> Vec<Update> {
        let mut updates = self.step();
        if self.is_planning_year() {
            let mut outcomes = self.roll_new_policy_outcomes();
            updates.append(&mut outcomes);
        }
        updates
    }

    fn apply_disaster(
        &mut self,
        event: &IconEvent,
        event_id: &Id,
        region_id: &Id,
    ) {
        let effect = event.intensity as f32
            * consts::EVENT_INTENSITY_TO_CONTENTEDNESS;

        self.change_habitability(
            -effect.round() as isize,
            region_id,
        );
        self.apply_event(*event_id, Some(*region_id));
    }

    fn update_processes(
        &mut self,
        changes: &mut EnumMap<Output, BTreeMap<Id, isize>>,
    ) {
        let mut rem_pts = consts::PROCESS_POINTS_PER_CYCLE;
        let mut add_pts = consts::PROCESS_POINTS_PER_CYCLE;

        for (_output, changes) in changes.iter_mut() {
            let mut total = changes
                .values()
                .map(|val| val.abs())
                .sum::<isize>();
            while rem_pts > 0 && add_pts > 0 && total > 0 {
                for (process_id, change) in changes.iter_mut() {
                    if *change < 0 && rem_pts > 0 {
                        rem_pts -= 1;
                        self.change_process_mix_share(
                            process_id, -1,
                        );
                        total -= 1;
                        *change += 1;
                    } else if *change > 0 && add_pts > 0 {
                        add_pts -= 1;
                        self.change_process_mix_share(
                            process_id, 1,
                        );
                        total -= 1;
                        *change -= 1;
                    }
                }
            }
        }
    }
}
