import state from './state';
import factors from '/src/display/factors';
import {GameInterface, Phase, Difficulty} from 'half-earth-engine';

// TODO let player choose difficulty;
// also; this needs to be re-created for each run.
let game;
newRun();

// Get the updated game state,
// and compute some additional variables
function updateState() {
  state.gameState = game.state();

  let world = state.gameState.world;
  state.gameState.contentedness = game.world_outlook();
  state.gameState.emissions = (world.co2_emissions + (world.n2o_emissions * 298.) + (world.ch4_emissions * 36.)) * 1e-15;
  state.gameState.population = world.regions.reduce((acc, r) => {
      return acc + r.population
    }, 0);
  state.gameState.industries.forEach((ind) => {
    ind.demand = game.industry_demand(ind.id);

    // Apply modifiers
    ind.resources = game.industry_resources(ind.id);
    ind.byproducts = game.industry_byproducts(ind.id);
  });
  state.gameState.world.regions.forEach((region) => {
    region.demand = game.region_demand(region.id);
  });
}

function updateFactors() {
  state.factors = factors.rank();
}

// Start a new run
function newRun() {
  game = GameInterface.new(Difficulty.Normal);
  state.endYear = game.state().world.year + 100;
  state.history = {
    emissions: [],
    land_use: [],
  };
  state.points = {
    research: 0,
    initiative: 0,
  };
  updateState();
  updateFactors();
  loadMeta();
  return game
}

// Step the game by one year
function step() {
  let completedProjects = game.step();
  updateState();
  // updateFactors();
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

function changeHabitability(amount, regionId) {
  game.change_habitability(amount, regionId);
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
  updateFactors();
}

function stopProject(projectId) {
  game.stop_project(projectId);
  updateState();
  updateFactors();
}

function changeProcessMixShare(processId, amount) {
  game.change_process_mix_share(processId, amount);
  updateState();
}

// Apply event effects
function applyEvent(eventId, regionId) {
  game.apply_event(eventId, regionId);
  updateState();
  updateFactors();
}

function applyBranchEffects(eventId, regionId, branchId) {
  game.apply_branch_effects(eventId, regionId, branchId);
  updateState();
  updateFactors();
}

function evalBranchConditions(eventId, regionId, branchId) {
  return game.eval_branch_conditions(eventId, regionId, branchId);
}

function upgradeProject(id) {
  game.upgrade_project(id);
  updateState();
  updateFactors();
}

function setTgav(tgav) {
  game.set_tgav(tgav);
  updateState();
}

function simulate(years) {
  return game.simulate(years);
}

function regionDemand(region) {
  return game.region_demand(region.id);
}

function regionHabitability(region) {
  return game.region_habitability(region.id);
}

function yearsRemaining(project) {
  return game.years_remaining(project.progress, project.points, project.cost);
}

function checkRequests() {
  return game.check_requests();
}

function _roll(phase, subphase, limit) {
  let p = Phase[`${phase}${subphase}`];
  if (p === undefined) {
    console.error(`Event phase "${phase}${subphase}" is not defined as an enum variant!`);
    return [];
  } else {
    return game.roll_events(p, limit);
  }
}

const roll = {
  planning: (subphase) => {
    return _roll('Planning', subphase, null);
  },
  world: (subphase) => {
    return _roll('World', subphase, 5);
  },
  report: (subphase) => {
    return _roll('Report', subphase, null);
  },
  break: (subphase) => {
    return _roll('Break', subphase, null);
  },
  end: (subphase) => {
    return _roll('End', subphase, null);
  },
  icon: () => {
    return _roll('Icon', '', null);
  },
}

// Save/load game metadata
function saveMeta() {
  let data = {
    runsPlayed: state.gameState.runs,
  };
  localStorage.setItem('gameData', JSON.stringify(data));
}

function loadMeta() {
  let data = localStorage.getItem('gameData');
  if (data !== null) {
    let parsed = JSON.parse(data);
    game.set_runs_played(parsed.runsPlayed || 0);
  }
}

updateState();
updateFactors();

export default {
  newRun, saveMeta, step,
  updateState, setTgav,
  changePoliticalCapital,
  changeLocalOutlook,
  changeHabitability,
  checkRequests,
  changeProcessMixShare,
  setProjectPoints, startProject, stopProject, upgradeProject,
  applyEvent, roll, simulate,
  applyBranchEffects, evalBranchConditions,
  regionDemand, regionHabitability,
  yearsRemaining, updateFactors};
