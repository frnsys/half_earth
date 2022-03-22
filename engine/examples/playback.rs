use serde::Deserialize;
use rand::{SeedableRng, rngs::SmallRng};
use hector_rs::{run_hector, emissions::get_emissions};
use half_earth_engine::{
    game::{Game, Difficulty}, events::Phase,
    projects::Status as ProjectStatus};
use std::{env, fs::File, io::BufReader, collections::HashMap};

#[derive(Deserialize)]
struct PlaybackScript {
    processes: Vec<HashMap<String, usize>>,
    projects: Vec<HashMap<String, (ProjectStatus, usize, usize)>>,
    events: Vec<Vec<(usize, Option<usize>)>>,
}

impl PlaybackScript {
    fn years(&self) -> usize {
        self.processes.len()
    }
}

fn find_project_id(game: &Game, name: &String) -> usize {
    game.state.projects.iter().find(|p| p.name == name).expect(format!("No project named: {}", name).as_str()).id
}

fn find_process_id(game: &Game, name: &String) -> usize {
    game.state.processes.iter().find(|p| p.name == name).expect(format!("No process named: {}", name).as_str()).id
}

fn playback(rng: &mut SmallRng, script: &PlaybackScript) {
    let file = File::open("renames.json").unwrap();
    let reader = BufReader::new(file);
    let renames: HashMap<String, String> = serde_json::from_reader(reader).unwrap();

    let difficulty = Difficulty::Normal;
    let mut game = Game::new(difficulty);
    let mut emissions = get_emissions(game.state.world.year);

    let years = script.years();
    for i in 0..years {
        println!("Step {:?}", i);
        let project_changes = &script.projects[i];
        for (name, (status, points, level)) in project_changes {
            let name = renames.get(name).unwrap_or(name);
            let project_id = find_project_id(&game, &name);
            let status_change = {
                let project = &mut game.state.projects[project_id];
                project.points = *points;
                if project.status != *status {
                    Some(status.clone())
                } else {
                    None
                }
            };

            if let Some(status) = status_change {
                match status {
                    ProjectStatus::Building => {
                        game.start_project(project_id, rng);
                    },
                    ProjectStatus::Inactive => {
                        game.stop_project(project_id);
                    }
                    _ => ()
                }
            }

            while game.state.projects[project_id].level < *level {
                game.upgrade_project(project_id);
            }
            while game.state.projects[project_id].level > *level {
                game.downgrade_project(project_id);
            }

            println!("  Project Change: {:?} -> {:?} / {:?} / {:?}", name, status, points, level);
        }

        let process_mix_changes = &script.processes[i];
        for (name, mix_share) in process_mix_changes {
            let name = renames.get(name).unwrap_or(name);
            let process_id = find_process_id(&game, &name);
            game.state.processes[process_id].mix_share = *mix_share;
            println!("Process Mix Change: {:?} -> {:?}", name, mix_share);
        }

        let completed_projects = game.step(rng);
        println!("  Completed Projects: {:?}", completed_projects);
        println!("  Emissions: {:?}", game.state.world.emissions() * 1e-15);

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
        println!("  Tgav: {:?}", tgav);
        if tgav > 0. {
            game.state.set_tgav(tgav);
        }
        println!("  Temp: {:?}", game.state.world.temperature);

        let events = &script.events[i];
        for (event_id, region_id) in events {
            println!("  Event: {:?}", game.event_pool.events[*event_id].name);
            game.apply_event(*event_id, *region_id);
        }

        println!("\n");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let script: PlaybackScript = serde_json::from_reader(reader).unwrap();

    // let mut rng: SmallRng = SeedableRng::seed_from_u64(0);
    let mut rng: SmallRng = SeedableRng::from_entropy();
    playback(&mut rng, &script);
}

