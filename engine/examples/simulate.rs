use std::{env, fs, fs::File, path::Path};
use serde::Serialize;
use indicatif::ProgressBar;
use rand::{SeedableRng, rngs::SmallRng};
use hector_rs::{run_hector, emissions::get_emissions};
use half_earth_engine::{
    content, resources, byproducts,
    game::{Game, Difficulty},
    kinds::{Output, OutputMap, Resource, ResourceMap, ByproductMap},
    projects::Status,
    production::{ProductionOrder, Process, ProcessFeature, calculate_required},
    events::Phase, consts};

const CO2_REF: f32 = 43.16;     // Gt, 2022, from SSP2-Baseline
const CH4_REF: f32 = 570.;      // Mt, https://www.iea.org/reports/methane-tracker-2020
const N2O_REF: f32 = 9.99;      // Mt, 2022, from SSP2-Baseline
const CO2EQ_REF: f32 = 49.36;   // Gt, 2016, from https://ourworldindata.org/grapher/total-ghg-emissions?tab=chart&country=~OWID_WRL
const ELEC_REF: f32 = 22777.8;  // TWh, https://www.iea.org/data-and-statistics/charts/electricity-generation-by-fuel-and-scenario-2018-2040
const FUEL_REF: f32 = 93333.2;  // TWh, https://www.eia.gov/todayinenergy/detail.php?id=46596
const CALS_REF: f32 = 2870.0;   // kcals per day per person, 2011, from https://www.nationalgeographic.com/what-the-world-eats/
const WATER_REF: f32 = 4600.;   // km3, global water demand for 2016?, https://www.nature.com/articles/s41545-019-0039-9
const CALS_LAND_REF: f32 = 51000000.; // km2, https://ourworldindata.org/land-use#breakdown-of-global-land-use-today
const POP_REF: f32 = 10.87; // people in 2100 in bn, from the UN World Population Prospects (2019, medium fertility)

/*
 * Other calibration values:
 * - Projected 2030 emissions gap against poorest 50%:
 *  - Richest 1%: 67.7t CO2/capita (for reference, 1% of 8bn is 80,000,000)
 *  - Richest 10%: 18.7t CO2/capita
 *  - Middle 40%: 2.5t CO2/capita
 *  - Global average: 2.2t CO2/capita
 *  - Source: https://policy-practice.oxfam.org/resources/carbon-inequality-in-2030-per-capita-consumption-emissions-and-the-15c-goal-621305/
 * - Sea level rise
 *  - 2.5m in 2100
 *  - Source: http://www.globalchange.umd.edu/data/annual-meetings/2019/Vega-Westhoff_HectorBRICKSLR_20191105.pdf
 */


#[derive(Serialize)]
#[derive(Debug, PartialEq, Clone)]
enum Scenario {
    BanFossilFuels,
    Nuclear,
    Solar,
    Veganism,
    Vegetarianism,
    ProtectHalf,
    Electrification,
    DAC,
    GreenHydrogen,
    EnergyQuotas,
    OneChildPolicy,
    SRM,
    ClosedBorders,
}

#[derive(Serialize)]
struct Report {
    roll_events: bool,
    start_year: usize,
    scenario_start: usize,
    scenarios: Vec<Scenario>,
    events: Vec<Vec<(String, Option<String>)>>,
    icon_events: Vec<Vec<(String, Option<String>)>>,
    summary: Vec<Snapshot>,
}

#[derive(Serialize)]
struct Snapshot {
    temp: f32,
    land_use: f32,
    emissions: f32,
    extinction_rate: f32,
    habitability: f32,
    outlook: f32,
}

fn promote_process(processes: &mut Vec<Process>, process_id: usize) {
    let mut points = 0;
    let output = processes[process_id].output.clone();
    for process in processes.iter_mut() {
        if process.id != process_id && process.output == output && process.mix_share > 0 {
            points += 1;
            process.mix_share -= 1;
        }
    }
    processes[process_id].mix_share += points;
}

impl Scenario {
    fn apply(&self, game: &mut Game, rng: &mut SmallRng) -> String {
        match self {
            Scenario::EnergyQuotas => {
                let p_id = find_project_id(game, "Energy Quotas");
                game.start_project(p_id, rng);
                "🔷 Implemented Energy Quotas".to_string()
            },
            Scenario::BanFossilFuels => {
                let mut to_ban = vec![];
                for process in &mut game.state.processes {
                    if process.features.contains(&ProcessFeature::IsFossil) {
                        to_ban.push(process.id);
                    }
                }

                let mut points: OutputMap<usize> = OutputMap::default();
                for process_id in &to_ban {
                    let process = &mut game.state.processes[*process_id];
                    let output = process.output.clone();
                    points[output] += process.mix_share;
                    process.mix_share = 0;
                }
                for (output, pts) in points.items_mut() {
                    while *pts > 0 {
                        for process in &mut game.state.processes {
                            if !to_ban.contains(&process.id) && process.output == output && !process.locked {
                                *pts -= 1;
                                process.mix_share += 1;
                                if *pts <= 0 {
                                    break;
                                }
                            }
                        }
                    }
                }

                "🔷 Banned Fossil Fuels".to_string()
            },
            Scenario::Nuclear => {
                let mut to_promote = vec![];
                for process in &mut game.state.processes {
                    let is_nuclear = process.features.contains(&ProcessFeature::CanMeltdown)
                        || process.features.contains(&ProcessFeature::MakesNuclearWaste);
                    if is_nuclear {
                        to_promote.push(process.id);
                    }
                }
                for p_id in to_promote {
                    promote_process(&mut game.state.processes, p_id);
                }
                "🔷 Promoted Nuclear Power".to_string()
            },
            Scenario::Solar => {
                let p_id = find_process_id(game, "Solar PV");
                promote_process(&mut game.state.processes, p_id);
                let p_id = find_project_id(game, "Next-Gen Solar PV");
                game.start_project(p_id, rng);
                game.state.projects[p_id].set_points(10);
                "🔷 Researching Next-Gen Solar PV/Promoted Solar PV".to_string()
            },
            Scenario::Veganism => {
                let p_id = find_project_id(game, "Veganism Mandate");
                game.start_project(p_id, rng);
                "🔷 Implemented Veganism Mandate".to_string()
            },
            Scenario::Vegetarianism => {
                let p_id = find_project_id(game, "Vegetarian Mandate");
                game.start_project(p_id, rng);
                "🔷 Implemented Vegetarian Mandate".to_string()
            },
            Scenario::ProtectHalf => {
                let p_id = find_project_id(game, "Land Protection");
                game.start_project(p_id, rng);
                for _ in 0..4 {
                    game.upgrade_project(p_id);
                }
                "🔷 Implemented Land Protection".to_string()
            },
            Scenario::Electrification => {
                let p_id = find_project_id(game, "Mass Electrification");
                game.start_project(p_id, rng);
                game.state.projects[p_id].set_points(10);
                "🔷 Started Mass Electrification".to_string()
            },
            Scenario::DAC => {
                let p_id = find_project_id(game, "Direct Air Capture");
                game.start_project(p_id, rng);
                game.state.projects[p_id].set_points(10);
                "🔷 Started Direct Air Capture".to_string()
            },
            Scenario::OneChildPolicy => {
                let p_id = find_project_id(game, "One-Child Policy");
                game.start_project(p_id, rng);
                "🔷 Implemented One-Child Policy".to_string()
            },
            Scenario::GreenHydrogen => {
                let p_id = find_process_id(game, "Green Hydrogen");
                promote_process(&mut game.state.processes, p_id);
                let p_id = find_project_id(game, "Green Hydrogen");
                game.start_project(p_id, rng);
                game.state.projects[p_id].set_points(10);
                "🔷 Promoted Green Hydrogen/Researching Green Hydrogen".to_string()
            },
            Scenario::SRM => {
                let p_id = find_project_id(game, "Solar Radiation Management");
                game.start_project(p_id, rng);
                game.state.projects[p_id].set_points(10);
                "🔷 Started Solar Radiation Management".to_string()
            },
            Scenario::ClosedBorders => {
                let p_id = find_project_id(game, "Closed Borders");
                game.start_project(p_id, rng);
                "🔷 Implemented Closed Borders".to_string()
            },
        }
    }
}

fn find_project_id(game: &Game, name: &'static str) -> usize {
    let p = game.state.projects.iter().find(|p| p.name == name).unwrap();
    p.id
}

fn find_process_id(game: &Game, name: &'static str) -> usize {
    let p = game.state.processes.iter().find(|p| p.name == name).unwrap();
    p.id
}

fn parse_scenarios() -> Vec<Scenario> {
    let args: Vec<String> = env::args().collect();
    let mut scenarios = vec![];
    if args.len() > 1 {
        for arg in args[1].split(',').filter(|s| s.len() > 0) {
            let scenario = match arg {
                "BanFossilFuels" => Scenario::BanFossilFuels,
                "Nuclear" => Scenario::Nuclear,
                "Veganism" => Scenario::Veganism,
                "Vegetarianism" => Scenario::Vegetarianism,
                "ProtectHalf" => Scenario::ProtectHalf,
                "Electrification" => Scenario::Electrification,
                "OneChildPolicy" => Scenario::OneChildPolicy,
                "EnergyQuotas" => Scenario::EnergyQuotas,
                "DAC" => Scenario::DAC,
                "Solar" => Scenario::Solar,
                "GreenHydrogen" => Scenario::GreenHydrogen,
                "SRM" => Scenario::SRM,
                "ClosedBorders" => Scenario::ClosedBorders,
                _ => panic!("Unknown scenario: {:?}", arg)
            };
            scenarios.push(scenario);
        }
    }
    scenarios
}

struct CalibrationData {
    wtr: csv::Writer<File>,
}

impl CalibrationData {
    pub fn new(game: &Game, path: String) -> CalibrationData {
        let mut wtr = csv::Writer::from_path(path).unwrap();
        let base_cols = vec![
            "Year",
            "Events",
            "Temperature",
            "CO2 Emissions (Gt)",
            "CH4 Emissions (Mt)",
            "N2O Emissions (Mt)",
            "CO2eq Emissions",
            "Population (b)",
            "World Outlook",
            "Habitability",
            "Extinction Rate",
            "Sea Level Rise",
            "Base Animal Cal Demand (Tcals)",
            "Base Plant Cal Demand (Tcals)",
            "Cal/Capita/Day",
            "Industry Fuel Demand (TWh)",
            "Industry Elec Demand (TWh)",
            "Industry Water Demand (km3)",
            "Agg Animal Cal Demand (Tcals)",
            "Agg Plant Cal Demand (Tcals)",
            "Agg Fuel Demand (TWh)",
            "Agg Elec Demand (TWh)",
            "Energy Land Req. (km2)",
            "Energy Water Req. (km3)",
            "Energy CO2 Emissions (Gt)",
            "Energy CH4 Emissions (Mt)",
            "Energy N2O Emissions (Mt)",
            "Calorie Land Req. (km2)",
            "Calorie Water Req. (km3)",
            "Calorie CO2 Emissions (Gt)",
            "Calorie CH4 Emissions (Mt)",
            "Calorie N2O Emissions (Mt)",
            "Industry CO2 Emissions (Gt)",
            "Industry CH4 Emissions (Mt)",
            "Industry N2O Emissions (Mt)",
            "Produced Fuel (TWh)",
            "Produced Elec (TWh)",
            "Produced Animal Cals (Tcals)",
            "Produced Plant Cals (Tcals)",
            "Produced Fuel (% Demand)",
            "Produced Elec (% Demand)",
            "Produced Animal Cals (% Demand)",
            "Produced Plant Cals (% Demand)",
            "Consumed Land (%)",
            "Consumed Water (%)",
            "Mean Income Level",
            "CO2 Ref (Gt)",
            "CH4 Ref (Mt)",
            "N2O Ref (Mt)",
            "CO2eq Ref (Gt)",
            "Elec Ref (TWh)",
            "Fuel Ref (TWh)",
            "Cals Ref (kcal/person/day)",
            "Cals Land Ref (km2)",
            "Water Ref (km3)",
            "Pop Ref (2100, bn people)",
        ];
        let mut cols: Vec<String> = base_cols.iter().map(|c| c.to_string()).collect();
        cols.extend(game.state.processes.iter().map(|p| format!("{:?}:{}:Mix Share", p.output, p.name)));
        cols.extend(game.state.processes.iter().map(|p| format!("{:?}:{}:Land Use", p.output, p.name)));
        cols.extend(game.state.processes.iter().map(|p| format!("{:?}:{}:Fuel Use", p.output, p.name)));
        cols.extend(game.state.processes.iter().map(|p| format!("{:?}:{}:Electricity Use", p.output, p.name)));
        cols.extend(game.state.processes.iter().map(|p| format!("{:?}:{}:CO2 Emissions (Gt)", p.output, p.name)));
        cols.extend(game.state.processes.iter().map(|p| format!("{:?}:{}:CH4 Emissions (Gt)", p.output, p.name)));
        cols.extend(game.state.processes.iter().map(|p| format!("{:?}:{}:N2O Emissions (Gt)", p.output, p.name)));
        cols.extend(game.state.world.regions.iter().map(|r| format!("Outlook:{}", r.name)));
        cols.extend(game.state.feedstocks.keys().iter().map(|k| format!("Feedstock:{}", format!("{:?}", k))));
        wtr.write_record(&cols).unwrap();

        CalibrationData { wtr }
    }

    pub fn snapshot(&mut self, game: &Game, events: &Vec<(String, Option<String>)>) {
        let pop_demand = game.state.world.demand();
        let agg_demand = game.state.output_demand;
        let produced = game.state.produced;

        let lic_pop = game.state.world.lic_population();
        let ind_demand = game.state.industries.iter().fold(resources!(), |acc, ind| acc + ind.resources) * lic_pop;
        let industry_byproducts = game.state.industries.iter().fold(byproducts!(), |acc, ind| acc + ind.byproducts) * lic_pop;

        let energy_orders: Vec<ProductionOrder> = game.state.processes.iter()
            .filter(|p| p.output == Output::Electricity || p.output == Output::Fuel)
            .map(|p| p.production_order(&agg_demand)).collect();
        let calorie_orders: Vec<ProductionOrder> = game.state.processes.iter()
            .filter(|p| p.output == Output::PlantCalories || p.output == Output::AnimalCalories)
            .map(|p| p.production_order(&agg_demand)).collect();

        let (energy_required_resources, _) = calculate_required(&energy_orders);
        let mut energy_byproducts = byproducts!();
        for order in energy_orders {
            energy_byproducts += order.process.byproducts * order.amount/order.process.output_modifier;
        }
        let (calorie_required_resources, _) = calculate_required(&calorie_orders);
        let mut calorie_byproducts = byproducts!();
        for order in calorie_orders {
            calorie_byproducts += order.process.byproducts * order.amount/order.process.output_modifier;
        }

        let mut vals: Vec<String> = vec![
            game.state.world.year as f32,
            events.len() as f32,
            game.state.world.temperature,
            game.state.world.co2_emissions * 1e-15,
            game.state.world.ch4_emissions * 1e-12,
            game.state.world.n2o_emissions * 1e-12,
            game.state.world.emissions() * 1e-15,
            game.state.world.population() * 1e-9,
            game.state.world.outlook(),
            game.state.world.habitability(),
            game.state.world.extinction_rate,
            game.state.world.sea_level_rise,
            pop_demand.animal_calories * 1e-9,
            pop_demand.plant_calories * 1e-9,
            (pop_demand.animal_calories + pop_demand.plant_calories)/game.state.world.population()/365.,
            ind_demand.fuel * 1e-9,
            ind_demand.electricity * 1e-9,
            ind_demand.water * 1e-12,
            agg_demand.animal_calories * 1e-9,
            agg_demand.plant_calories * 1e-9,
            agg_demand.fuel * 1e-9,
            agg_demand.electricity * 1e-9,
            energy_required_resources.land * 1e-6,
            energy_required_resources.water * 1e-12,
            energy_byproducts.co2 * 1e-15,
            energy_byproducts.ch4 * 1e-12,
            energy_byproducts.n2o * 1e-12,
            calorie_required_resources.land * 1e-6,
            calorie_required_resources.water * 1e-12,
            calorie_byproducts.co2 * 1e-15,
            calorie_byproducts.ch4 * 1e-12,
            calorie_byproducts.n2o * 1e-12,
            industry_byproducts.co2 * 1e-15,
            industry_byproducts.ch4 * 1e-12,
            industry_byproducts.n2o * 1e-12,
            produced.fuel * 1e-9,
            produced.electricity * 1e-9,
            produced.animal_calories * 1e-9,
            produced.plant_calories * 1e-9,
            produced.fuel/agg_demand.fuel * 100.,
            produced.electricity/agg_demand.electricity * 100.,
            produced.animal_calories/agg_demand.animal_calories * 100.,
            produced.plant_calories/agg_demand.plant_calories * 100.,
            game.state.consumed_resources[Resource::Land]/consts::STARTING_RESOURCES.land * 100.,
            game.state.consumed_resources[Resource::Water]/consts::STARTING_RESOURCES.water * 100.,
            game.state.world.income_level(),
            CO2_REF,
            CH4_REF,
            N2O_REF,
            CO2EQ_REF,
            ELEC_REF,
            FUEL_REF,
            CALS_REF,
            CALS_LAND_REF,
            WATER_REF,
            POP_REF,
        ].iter().map(|v| v.to_string()).collect();
        vals.extend(game.state.processes.iter().map(|p| p.mix_percent().to_string()));
        vals.extend(game.state.processes.iter().map(|p| {
            let order = p.production_order(&agg_demand);
            ((p.adj_resources().land * order.amount)/consts::STARTING_RESOURCES.land).to_string()
        }));
        vals.extend(game.state.processes.iter().map(|p| {
            let order = p.production_order(&agg_demand);
            (p.adj_resources().fuel * order.amount).to_string()
        }));
        vals.extend(game.state.processes.iter().map(|p| {
            let order = p.production_order(&agg_demand);
            (p.adj_resources().electricity * order.amount).to_string()
        }));
        vals.extend(game.state.processes.iter().map(|p| {
            let order = p.production_order(&agg_demand);
            (p.adj_byproducts().co2 * order.amount * 1e-15).to_string()
        }));
        vals.extend(game.state.processes.iter().map(|p| {
            let order = p.production_order(&agg_demand);
            (p.adj_byproducts().ch4 * order.amount * 1e-12).to_string()
        }));
        vals.extend(game.state.processes.iter().map(|p| {
            let order = p.production_order(&agg_demand);
            (p.adj_byproducts().n2o * order.amount * 1e-12).to_string()
        }));
        vals.extend(game.state.world.regions.iter().map(|r| {
            r.outlook.to_string()
        }));
        vals.extend(game.state.feedstocks.values().iter().map(|v| {
            v.to_string()
        }));
        self.wtr.write_record(&vals).unwrap();
    }
}

fn single_run(id: usize, rng: &mut SmallRng, scenarios: &Vec<Scenario>, dir: &str) {
    let difficulty = Difficulty::Normal;
    let mut game = Game::new(difficulty);
    let mut emissions = get_emissions(game.state.world.year);

    let mut data = CalibrationData::new(&game, format!("{}/{:?}.csv", dir, id));
    let mut report = Report {
        roll_events: true,
        start_year: game.state.world.year,
        scenario_start: 10,
        scenarios: scenarios.to_vec(),
        events: vec![],
        icon_events: vec![],
        summary: vec![]
    };

    let pb = ProgressBar::new(100);
    for i in 0..100 {
        let mut year_events = vec![];

        if i == report.scenario_start {
            for scenario in &report.scenarios {
                let desc = scenario.apply(&mut game, rng);
                year_events.push((desc, None));
            }
        }

        let completed_projects = game.step(rng);

        if report.scenarios.contains(&Scenario::DAC) {
            let p_id = find_project_id(&game, "Direct Air Capture");
            if game.state.projects[p_id].status == Status::Active {
                for _ in 0..game.state.projects[p_id].upgrades.len() {
                    game.upgrade_project(p_id);
                }
            }
        }

        // Hector separates out FFI and LUC emissions
        // but we lump them together
        // Units: <https://github.com/JGCRI/hector/wiki/Hector-Units>
        // 'ffi_emissions': world.co2_emissions * 12/44 * 1e-15, // Pg C/y
        // 'CH4_emissions': world.ch4_emissions * 1e-12, // Tg/y
        // 'N2O_emissions': world.n2o_emissions * 1e-12, // Tg/y
        emissions.get_mut("simpleNbox").unwrap().get_mut("ffi_emissions").unwrap().push((game.state.world.co2_emissions * 12./44. * 1e-15) as f64);
        emissions.get_mut("CH4").unwrap().get_mut("CH4_emissions").unwrap().push((game.state.world.ch4_emissions * 1e-12) as f64);
        emissions.get_mut("N2O").unwrap().get_mut("N2O_emissions").unwrap().push((game.state.world.n2o_emissions * 1e-12) as f64);
        let tgav = unsafe {
            run_hector(game.state.world.year, &emissions) as f32
        };
        if tgav > 0. {
            game.state.set_tgav(tgav);
        }

        let events = if report.roll_events {
            game.roll_events_for_phase(Phase::WorldMain, Some(5), rng)
        } else {
            vec![]
        };
        for (ev_id, region_id) in events {
            let ev = &game.event_pool.events[ev_id];
            match region_id {
                Some(id) => {
                    let region = &game.state.world.regions[id];
                    year_events.push((ev.name.to_string(), Some(region.name.to_string())));
                },
                None => {
                    year_events.push((ev.name.to_string(), None));
                }
            }
            game.apply_event(ev_id, region_id);
        }

        // Main events
        for p_id in completed_projects {
            let project = &game.state.projects[p_id];
            year_events.push(
                (format!("🔶 Finished {}", project.name), None));
        }

        // Icon events
        let mut year_icon_events = vec![];
        let icon_events = if report.roll_events {
            game.roll_events_for_phase(Phase::Icon, None, rng)
        } else {
            vec![]
        };
        for (ev_id, region_id) in icon_events {
            let ev = &game.event_pool.events[ev_id];
            match region_id {
                Some(id) => {
                    let mut region = &mut game.state.world.regions[id];
                    year_icon_events.push((ev.name.to_string(), Some(region.name.to_string())));

                    // Apply outlook effect
                    // region.outlook -= ev.intensity as f32 * 0.05;
                    region.base_habitability -= ev.intensity as f32 * 0.1;
                },
                None => {
                    year_icon_events.push((ev.name.to_string(), None));
                }
            }
            game.apply_event(ev_id, region_id);
        }

        data.snapshot(&game, &year_events);

        report.summary.push(Snapshot {
            temp: game.state.world.temperature,
            emissions: game.state.world.emissions() * 1e-15,
            outlook: game.state.world.outlook(),
            extinction_rate: game.state.world.extinction_rate,
            habitability: game.state.world.habitability(),
            land_use: game.state.consumed_resources[Resource::Land]/consts::STARTING_RESOURCES.land * 100.,
        });
        report.events.push(year_events);
        report.icon_events.push(year_icon_events);

        pb.inc(1);
    }

    serde_json::to_writer(
        &File::create(format!("{}/{:?}.json", dir, id)).unwrap(),
        &report).unwrap();

    let event_names: Vec<&str> = content::events().iter().filter(|ev| {
        match ev.phase {
            Phase::WorldMain | Phase::Icon => true,
            _ => false
        }
    }).map(|ev| ev.name).collect();
    serde_json::to_writer(
        &File::create(format!("{}/all_events.json", dir)).unwrap(),
        &event_names).unwrap();
}

fn main() {
    // let mut rng: SmallRng = SeedableRng::seed_from_u64(0);
    let mut rng: SmallRng = SeedableRng::from_entropy();
    let scenarios = parse_scenarios();

    let output_dir = if scenarios.len() == 0 {
        "NoScenarios".to_string()
    } else {
        let scenario_names: Vec<String> = scenarios.iter().map(|s| format!("{:?}", s)).collect();
        scenario_names.join("_")
    };
    let dir = format!("/tmp/hes/{}/calibration", output_dir);
    if Path::new(&dir).is_dir() {
        fs::remove_dir_all(&dir).unwrap();
    }
    fs::create_dir_all(&dir).unwrap();

    let n: usize = 100;
    let pb = ProgressBar::new(n as u64);
    for i in 0..n {
        single_run(i, &mut rng, &scenarios, &dir);
        pb.inc(1);
    }

    println!("{}", dir);
}