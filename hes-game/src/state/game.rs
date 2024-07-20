use super::Points;
use crate::{consts, display, views::DisplayEvent};
use extend::ext;
use hes_engine::{
    events::{Flag, Phase},
    projects::{Project, Status},
    Game,
    ProjectType,
};
use std::collections::HashMap;

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
        let energy = self.state.output_demand.energy();
        format!("{}PWh", (display::twh(energy) / 1e3).round())
    }

    fn energy_twh(&self) -> String {
        let energy = self.state.output_demand.energy();
        format!("{}TWh", display::twh(energy).round())
    }

    fn avg_income_level(&self) -> usize {
        let mut total = 0.;
        for region in &self.state.world.regions {
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
        for region in &self.state.world.regions {
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
        project_id: usize,
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

    fn pay_points(&mut self, project_id: usize) -> bool {
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
        project_id: usize,
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
        project_id: usize,
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

    fn pass_policy(&mut self, project_id: usize) {
        let kind = {
            let project = &self.world.projects[project_id];
            project.kind
        };
        if kind == ProjectType::Policy {
            self.start_project(project_id);
        }
    }

    fn stop_policy(&mut self, project_id: usize) {
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
        project_id: usize,
        is_free: bool,
        queued_upgrades: &mut HashMap<usize, bool>,
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
                    queued_upgrades.insert(project_id, true);
                }
            }
            true
        } else {
            false
        }
    }

    fn downgrade_project_x(
        &mut self,
        project_id: usize,
        queued_upgrades: &mut HashMap<usize, bool>,
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
                queued_upgrades.insert(project_id, false);
            }
        }
    }

    fn roll_events(
        &mut self,
        phase: Phase,
        limit: Option<usize>,
    ) -> Vec<DisplayEvent> {
        let events = self.roll_events_for_phase(phase, limit);
        events
            .into_iter()
            .map(|ev| DisplayEvent::new(ev, &self.state))
            .collect()
    }
}
