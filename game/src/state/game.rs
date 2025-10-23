use super::ui::Points;
use crate::{
    DEBUG, consts,
    display::{self, DisplayEvent},
    state::update_factors,
};
use enum_map::EnumMap;
use extend::ext;
use hes_engine::*;
use std::collections::BTreeMap;

#[ext]
pub impl State {
    /// For total land use we have to take into account
    /// protected land and use starting land resources as the baseline,
    /// rather than available land (which is starting land minus protected land).
    fn land_use_percent(&self) -> String {
        let usage = self.resource_demand.of(Resource::Land)
            + (self.protected_land * self.world.starting_resources.land);
        let total_land = self.world.starting_resources.land;
        let percent = usage / total_land;
        format!("{}%", display::percent(percent.min(1.0), true))
    }

    fn water_use_percent(&self) -> String {
        let usage = self.resource_demand.of(Resource::Water);
        let total_water = self.resources.available.water;
        let percent = usage / total_water;
        format!("{}%", display::percent(percent.min(1.0), true))
    }

    fn temp_anomaly(&self) -> String {
        format!("{:+.1}C", self.world.temperature)
    }

    fn energy_pwh(&self) -> String {
        let energy = self.output_demand.total().energy();
        format!("{}PWh", display::pwh(energy).round())
    }

    fn energy_twh(&self) -> String {
        let energy = self.output_demand.total().energy();
        format!("{}TWh", display::twh(energy).round())
    }

    fn avg_income_level(&self) -> usize {
        let mut total = 0.;
        for region in self.world.regions.iter() {
            let income = region.income.level() as f32 + 1. + region.development;
            total += income;
        }
        let n_regions = self.world.regions.len();
        (total / n_regions as f32).round().max(0.) as usize
    }

    /// Cost for the next point for a project, taking into
    /// account discounts.
    fn next_point_cost(&self, kind: &ProjectType) -> usize {
        let mut discount = 0;
        if *kind == ProjectType::Research {
            if self.flags.contains(&Flag::HyperResearch) {
                discount += 1;
            }
            if self.npcs.is_ally("The Accelerationist") {
                discount += 1;
            }
        }
        consts::POINT_COST.saturating_sub(discount) as usize
    }

    fn buy_point(&mut self, project_id: &Id, points: &mut Points) -> bool {
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
            if cost <= self.political_capital {
                self.change_political_capital(-cost);
                match kind {
                    ProjectType::Research => points.research += 1,
                    ProjectType::Initiative => points.initiative += 1,
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
        let available = self.political_capital;
        if project.status == Status::Inactive && available >= project.cost as isize {
            self.change_political_capital(-(project.cost as isize));
            true
        } else {
            false
        }
    }

    fn assign_point(&mut self, project_id: &Id, points: &mut Points) {
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

    fn unassign_points(&mut self, project_id: &Id, points: usize) {
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

    fn upgrade_project_ext(
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
            let available = self.political_capital;
            if !is_free && available >= upgrade.cost as isize {
                self.change_political_capital(-(upgrade.cost as isize));
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

    fn downgrade_project_ext(&mut self, project_id: &Id, queued_upgrades: &mut BTreeMap<Id, bool>) {
        let (kind, prev_upgrade) = {
            let project = &self.world.projects[project_id];
            (project.kind, project.prev_upgrade())
        };

        if let Some(upgrade) = prev_upgrade {
            self.change_political_capital(upgrade.cost as isize);
            if kind == ProjectType::Policy {
                self.downgrade_project(project_id);
            } else {
                queued_upgrades.insert(*project_id, false);
            }
        }
    }

    fn roll_events(&mut self, phase: EventPhase) -> Vec<DisplayEvent> {
        if DEBUG.skip_events {
            vec![]
        } else {
            let events = self
                .roll_events(phase)
                .into_iter()
                .map(|ev| DisplayEvent::new(ev, self))
                .collect();
            update_factors(self);
            events
        }
    }

    fn upgrade_projects(&mut self, upgrades: &mut BTreeMap<Id, bool>) {
        // for (id, queued) in self.ui.queued_upgrades.iter_mut() {
        for (id, queued) in upgrades.iter_mut() {
            if *queued {
                *queued = false;
                self.upgrade_project(id);
            }
        }
    }

    fn apply_disaster(&mut self, event: &IconEvent, event_id: &Id, region_id: &Id) {
        let effect = event.intensity as f32 * consts::EVENT_INTENSITY_TO_CONTENTEDNESS;

        self.apply_disaster(-effect.round() as isize, region_id);
        self.apply_event(*event_id, Some(*region_id));
    }

    fn update_processes(&mut self, changes: &mut EnumMap<Output, BTreeMap<Id, isize>>) {
        for (_output, changes) in changes.iter_mut() {
            let mut rem_pts = consts::PROCESS_POINTS_PER_CYCLE;
            let mut add_pts = consts::PROCESS_POINTS_PER_CYCLE;
            let mut total = changes.values().map(|val| val.abs()).sum::<isize>();
            while (rem_pts > 0 || add_pts > 0) && total > 0 {
                for (process_id, change) in changes.iter_mut() {
                    if *change < 0 && rem_pts > 0 {
                        rem_pts -= 1;
                        self.change_process_mix_share(process_id, -1);
                        total -= 1;
                        *change += 1;
                    } else if *change > 0 && add_pts > 0 {
                        add_pts -= 1;
                        self.change_process_mix_share(process_id, 1);
                        total -= 1;
                        *change -= 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_process_mix() {
        let mut state = State::default();
        let mut changes: EnumMap<Output, BTreeMap<Id, isize>> = EnumMap::default();

        let solar_pv = state
            .world
            .processes
            .iter()
            .find(|p| p.name == "Solar PV")
            .unwrap()
            .id;
        let hydro = state
            .world
            .processes
            .iter()
            .find(|p| p.name == "Hydropower")
            .unwrap()
            .id;
        let coal = state
            .world
            .processes
            .iter()
            .find(|p| p.name == "Coal Power Generation")
            .unwrap()
            .id;
        let ind_ag = state
            .world
            .processes
            .iter()
            .find(|p| p.name == "Industrial Crop Ag")
            .unwrap()
            .id;
        let org_ag = state
            .world
            .processes
            .iter()
            .find(|p| p.name == "Organic Crop Ag")
            .unwrap()
            .id;

        let coal_mix = state.world.processes[&coal].mix_share;
        assert_eq!(coal_mix, 7);

        // We're specifically testing changes that require multiple cycles.
        assert!(coal_mix > consts::PROCESS_POINTS_PER_CYCLE);

        changes[Output::Electricity].insert(coal, -7);
        changes[Output::Electricity].insert(solar_pv, 5);
        changes[Output::Electricity].insert(hydro, 2);

        let ind_ag_mix = state.world.processes[&ind_ag].mix_share;
        assert_eq!(ind_ag_mix, 14);
        assert!(ind_ag_mix > consts::PROCESS_POINTS_PER_CYCLE);

        changes[Output::PlantCalories].insert(ind_ag, -12);
        changes[Output::PlantCalories].insert(org_ag, 12);

        state.update_processes(&mut changes);

        // Assuming 5 points change per cycle for each output.
        assert_eq!(consts::PROCESS_POINTS_PER_CYCLE, 5);
        assert_eq!(changes[Output::Electricity][&coal], -2);
        assert_eq!(changes[Output::Electricity][&solar_pv], 2);
        assert_eq!(changes[Output::Electricity][&hydro], 0);
        assert_eq!(changes[Output::PlantCalories][&ind_ag], -7);
        assert_eq!(changes[Output::PlantCalories][&org_ag], 7);
    }
}
