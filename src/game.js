import state from './state';
import display from 'lib/display';
import {GameInterface, Difficulty} from 'half-earth-engine';

// TODO let player choose difficulty;
// also; this needs to be re-created for each run.
let game = GameInterface.new(Difficulty.Normal);

const intensities = {
  'land': {
    'energy': [0, 0.001, 0.01, 0.1],
    'calories': [0, 0.001, 0.002, 0.01],
  },
  'labor': {
    'energy': [0, 0.001, 0.01, 0.1], // TODO
    'calories': [0, 0.001, 0.002, 0.01], // TODO
  },
  'energy': {
    'energy': [0, 0.001, 0.01, 0.1], // TODO EROI
    'calories': [0, 0.00015, 0.0005, 0.001],
  },
  'water': {
    'energy': [0, 1, 2, 5],
    'calories': [0, 1, 2, 3],
  },
  'emissions': {
    'energy': [-2000, 0, 200, 800],
    'calories': [-1, 0, 0.5, 1],
  },
  'biodiversity': {
    'energy': [0, 1, 2, 3],
    'calories': [0, 1, 2, 3],
  }
};

function intensity(val, key, type) {
  let stops = intensities[key][type];
  for (let i = 0; i < stops.length - 1; i++) {
    if (val >= stops[i] && val < stops[i+1]) {
      return i+1;
    }
  }
  return stops.length;
}


// Get the updated game state,
// and compute some additional variables
function updateState() {
  state.gameState = game.state();

  let world = state.gameState.world;
  state.gameState.contentedness = world.regions.reduce((acc, r) => {
    return acc + r.outlook;
  }, 0);
  state.gameState.emissions = (world.co2_emissions + (world.n2o_emissions * 298.) + (world.ch4_emissions * 36.)) * 1e-15;
  state.gameState.population = world.regions.reduce((acc, r) => {
      return acc + r.population
    }, 0);

  // TODO is this the best place for this?
  // TODO add in industries as well
  let resourceRankings = {};
  ['land', 'water', 'energy', 'emissions', 'biodiversity'].forEach((k) => {
    let rankings = state.gameState.processes.map((p, i) => {
      let produced = state.gameState.produced_by_process[i];
      let base = 0;
      if (k == 'land' || k == 'water') {
        base = p.resources[k];
      } else if (k == 'energy') {
        base = (p.resources['electricity'] + p.resources['fuel']);
      } else if (k == 'emissions') {
        base = display.co2eq(p.byproducts);
      } else if (k == 'biodiversity') {
        // TODO tweak this
        base = p.byproducts[k] + (p.resources['land'] * 10);
      }

      let type =
        (p.output == 'Electricity' || p.output == 'Fuel')
        ? 'energy' : 'calories';

      let total = base * produced;
      let inten = intensity(base, k, type);

      return {
        name: p.name,
        produced,
        intensity: inten,
        output: p.output,
        amount: total
      }
    });
    rankings.sort((a, b) => a.amount > b.amount ? -1 : 1)
    resourceRankings[k] = rankings;
  });
  state.gameState.resourceRankings = resourceRankings;
}


// Start a new run
function newRun() {
  game = GameInterface.new(Difficulty.Normal);
  updateState();
}

// Step the game by one year
function stepUpdate() {
  let completedProjects = game.step();
  updateState();
  return completedProjects;
}

function changePoliticalCapital(amount) {
  game.change_political_capital(amount);
  updateState();
}

function changeLocalOutlook(amount, regionId) {
  game.change_local_outlook(amount, regionId);
  updateState();
}

// Set point allocation for a project
function setProjectPoints(projectId, points) {
  game.set_project_points(projectId, points);
  updateState();
}

function startProject(projectId) {
  game.start_project(projectId);
  updateState();
}

function stopProject(projectId) {
  game.stop_project(projectId);
  updateState();
}

function banProcess(processId) {
  game.ban_process(processId);
  updateState();
}

function unbanProcess(processId) {
  game.unban_process(processId);
  updateState();
}

function promoteProcess(processId) {
  game.promote_process(processId);
  updateState();
}

function unpromoteProcess(processId) {
  game.unpromote_process(processId);
  updateState();
}

// Select a response to an event
function selectChoice(eventId, regionId, choiceId) {
  game.set_event_choice(eventId, regionId, choiceId);
  updateState();
}

// Apply event effects
function applyEvent(eventId, regionId) {
  game.apply_event(eventId, regionId);
  updateState();
}

function upgradeProject(id) {
  game.upgrade_project(id);
  updateState();
}

function setTgav(tgav) {
  game.set_tgav(tgav);
  updateState();
}

function setPriority(priority) {
  game.set_priority(priority);
  updateState();
}


function simulate(years) {
  return game.simulate(years);
}

function industryDemand(industry) {
  return game.industry_demand(industry.id);
}

function regionDemand(region) {
  return game.region_demand(region.id);
}

function regionHabitability(region) {
  return game.region_habitability(region.id);
}

const roll = {
  planningEvents: () => game.roll_planning_events(),
  worldStartEvents: () => game.roll_world_start_events(),
}

updateState();

export default {
  newRun, stepUpdate,
  updateState, setTgav,
  setPriority,
  changePoliticalCapital,
  changeLocalOutlook,
  banProcess, unbanProcess,
  promoteProcess, unpromoteProcess,
  setProjectPoints, startProject, stopProject, upgradeProject,
  selectChoice, applyEvent, roll, simulate,
  industryDemand, regionDemand, regionHabitability};
