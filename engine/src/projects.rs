use crate::effects::Effect;
use crate::production::ProductionOrder;
use crate::kinds::{ResourceMap, ByproductMap, Output};

const MAINTENANCE_POINTS: f32 = 100.;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    Inactive,
    Building(f32),
    Active,
    Stalled,
    Halted,
    Finished,
}

impl Default for Status {
    fn default() -> Self {
        Status::Inactive
    }
}


#[derive(Default)]
pub struct Project {
    id: usize,
    name: &'static str,
    years: usize,
    pub status: Status,
    ongoing: bool,
    locked: bool,
    resources: ResourceMap<f32>,
    byproducts: ByproductMap<f32>,
    pub effects: Vec<Effect>
}

impl Project {
    /// Projects are constructed and maintained as part of the
    /// general production system. Points are "produced" and
    /// go towards their construction or maintenance.
    pub fn production_order(&self) -> Option<ProductionOrder> {
        match self.status {
            Status::Building(points) => {
                Some(ProductionOrder {
                    output: Output::Project,
                    amount: self.req_points - points,
                    reqs: self.build_reqs,
                    byproducts: self.byproducts,
                })
            },
            Status::Active|Status::Stalled => {
                Some(ProductionOrder {
                    output: Output::Project,
                    amount: MAINTENANCE_POINTS,
                    reqs: self.maintenance_reqs,
                    byproducts: self.byproducts,
                })
            },
            _ => None
        }
    }

    pub fn apply_points(&mut self, points: f32) {
        match &mut self.status {
            Status::Building(p) => {
                *p += points;
                if *p >= self.req_points {
                    if self.ongoing {
                        self.status = Status::Active;
                    } else {
                        self.status = Status::Finished;
                    }
                }
            },
            Status::Active|Status::Stalled => {
                if points < MAINTENANCE_POINTS {
                    self.status = Status::Stalled;
                } else {
                    self.status = Status::Active;
                }
            },
            _ => ()
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_apply_points_ongoing() {
        let mut p = Project {
            status: Status::Building(0.),
            req_points: 20.,
            ongoing: true,
            build_reqs: resources!(),
            maintenance_reqs: resources!(),
            byproducts: byproducts!(),
            effects: vec![]
        };

        p.apply_points(12.);
        assert_eq!(p.status, Status::Building(12.));

        p.apply_points(12.);
        assert_eq!(p.status, Status::Active);

        p.apply_points(MAINTENANCE_POINTS/2.);
        assert_eq!(p.status, Status::Stalled);

        p.apply_points(MAINTENANCE_POINTS);
        assert_eq!(p.status, Status::Active);
    }

    #[test]
    fn test_apply_points_not_ongoing() {
        let mut p = Project {
            status: Status::Building(0.),
            req_points: 20.,
            ongoing: false,
            build_reqs: resources!(),
            maintenance_reqs: resources!(),
            byproducts: byproducts!(),
            effects: vec![]
        };

        p.apply_points(12.);
        assert_eq!(p.status, Status::Building(12.));

        p.apply_points(12.);
        assert_eq!(p.status, Status::Finished);

        // Shouldn't make a difference
        p.apply_points(MAINTENANCE_POINTS/2.);
        assert_eq!(p.status, Status::Finished);

        p.apply_points(MAINTENANCE_POINTS);
        assert_eq!(p.status, Status::Finished);
    }
}
