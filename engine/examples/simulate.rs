use rand::{SeedableRng, rngs::SmallRng};
use half_earth_engine::{resources, byproducts, state::State, game::{Game, Difficulty}, kinds::{Output, Resource, ResourceMap, ByproductMap}, production::{ProductionOrder, ProcessStatus, calculate_required}, events::Phase, consts};

enum Scenario {
    BanFossilFuels,
    Veganism,
    Vegetarianism
}

impl Scenario {
    fn apply(&self, state: &mut State) {
        match self {
            Scenario::BanFossilFuels => {
                // TODO
            },
            Scenario::Veganism => {
                // TODO
            },
            Scenario::Vegetarianism => {
                // TODO
            }
        }
    }
}

fn main() {
    let difficulty = Difficulty::Normal;
    let mut game = Game::new(difficulty);
    // let mut rng: SmallRng = SeedableRng::seed_from_u64(0);
    let mut rng: SmallRng = SeedableRng::from_entropy();

    let co2_ref = 43.16;     // Gt, 2022, from SSP2-Baseline
    let ch4_ref = 570.;      // Mt, https://www.iea.org/reports/methane-tracker-2020
    let n2o_ref = 9.99;      // Mt, 2022, from SSP2-Baseline
    let co2eq_ref = 49.36;   // Gt, 2016, from https://ourworldindata.org/grapher/total-ghg-emissions?tab=chart&country=~OWID_WRL
    let elec_ref = 22777.8;  // TWh, https://www.iea.org/data-and-statistics/charts/electricity-generation-by-fuel-and-scenario-2018-2040
    let fuel_ref = 93333.2;  // TWh, https://www.eia.gov/todayinenergy/detail.php?id=46596
    let cals_ref = 2870.0;   // kcals per day per person, 2011, from https://www.nationalgeographic.com/what-the-world-eats/
    let water_ref = 4600.;   // km3, global water demand for 2016?, https://www.nature.com/articles/s41545-019-0039-9
    let cals_land_ref = 51000000.; // km2, https://ourworldindata.org/land-use#breakdown-of-global-land-use-today
    let pop_ref = 10.87; // people in 2100 in bn, from the UN World Population Prospects (2019, medium fertility)

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

    println!("Starting resources: {:?}", game.state.resources);
    println!("Starting feedstocks: {:?}", game.state.feedstocks);
    println!("==============================");

    let file_path = "/tmp/calibration.csv";
    let mut wtr = csv::Writer::from_path(file_path).unwrap();
    let base_cols = vec![
        "Year",
        "Temperature",
        "CO2 Emissions (Gt)",
        "CH4 Emissions (Mt)",
        "N2O Emissions (Mt)",
        "CO2eq Emissions",
        "Population (b)",
        "Outlook",
        "Habitability",
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
    cols.extend(game.state.processes.iter().map(|p| format!("{:?}:{}:CO2 Emissions (Gt)", p.output, p.name)));
    cols.extend(game.state.processes.iter().map(|p| format!("{:?}:{}:CH4 Emissions (Gt)", p.output, p.name)));
    cols.extend(game.state.processes.iter().map(|p| format!("{:?}:{}:N2O Emissions (Gt)", p.output, p.name)));
    wtr.write_record(&cols).unwrap();

    for i in 0..100 {
        let starting_resources = game.state.resources.clone();
        // let starting_feedstocks = game.state.feedstocks.clone();

        // if i == 20 {
        //     // Ind Ag (Livestock)
        //     game.state.processes[10].status = ProcessStatus::Banned;
        //     // Reg Ag (Livestock)
        //     game.state.processes[5].status = ProcessStatus::Promoted;
        //     // Cellular meat
        //     game.state.processes[2].status = ProcessStatus::Promoted;
        //     game.state.processes[2].locked = false;

        //     // Coal power generation
        //     game.state.processes[13].status = ProcessStatus::Banned;

        //     // Natural gas power generation
        //     game.state.processes[14].status = ProcessStatus::Banned;

        //     game.state.processes[17].status = ProcessStatus::Promoted;
        // }

        game.step(&mut rng);
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
            energy_byproducts += order.process.byproducts * order.amount;
        }
        let (calorie_required_resources, _) = calculate_required(&calorie_orders);
        let mut calorie_byproducts = byproducts!();
        for order in calorie_orders {
            calorie_byproducts += order.process.byproducts * order.amount;
        }

        println!("Year {:?}", game.state.world.year);

        let mut vals: Vec<String> = vec![
            game.state.world.year as f32,
            game.state.world.temperature,
            game.state.world.co2_emissions * 1e-15,
            game.state.world.ch4_emissions * 1e-12,
            game.state.world.n2o_emissions * 1e-12,
            game.state.world.emissions() * 1e-15,
            game.state.world.population() * 1e-9,
            game.state.world.outlook(),
            game.state.world.habitability(),
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
            game.state.consumed_resources[Resource::Land]/(starting_resources[Resource::Land]+1.) * 100.,
            game.state.consumed_resources[Resource::Water]/(starting_resources[Resource::Water]+1.) * 100.,
            game.state.world.income_level(),
            co2_ref,
            ch4_ref,
            n2o_ref,
            co2eq_ref,
            elec_ref,
            fuel_ref,
            cals_ref,
            cals_land_ref,
            water_ref,
            pop_ref,
        ].iter().map(|v| v.to_string()).collect();
        vals.extend(game.state.processes.iter().map(|p| p.mix_share.to_string()));
        vals.extend(game.state.processes.iter().map(|p| {
            let order = p.production_order(&agg_demand);
            ((p.resources.land * order.amount)/consts::STARTING_RESOURCES.land).to_string()
        }));
        vals.extend(game.state.processes.iter().map(|p| {
            let order = p.production_order(&agg_demand);
            (p.byproducts.co2 * order.amount * 1e-15).to_string()
        }));
        vals.extend(game.state.processes.iter().map(|p| {
            let order = p.production_order(&agg_demand);
            (p.byproducts.ch4 * order.amount * 1e-12).to_string()
        }));
        vals.extend(game.state.processes.iter().map(|p| {
            let order = p.production_order(&agg_demand);
            (p.byproducts.n2o * order.amount * 1e-12).to_string()
        }));
        wtr.write_record(&vals).unwrap();

        println!("  Events:");
        let events = game.roll_events_for_phase(Phase::WorldMain, Some(5), &mut rng);
        for (ev_id, region_id) in events {
            let ev = &game.event_pool.events[ev_id];
            match region_id {
                Some(id) => {
                    let region = &game.state.world.regions[id];
                    println!("    {:?} in {:?}", ev.name, region.name);
                },
                None => println!("    {:?}", ev.name),
            }
        }

        println!("------------------------------");
    }
}