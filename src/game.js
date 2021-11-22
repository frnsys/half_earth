import state from './state';
import display from 'lib/display';
import {GameInterface, Phase, Difficulty} from 'half-earth-engine';

// TODO let player choose difficulty;
// also; this needs to be re-created for each run.
let game = GameInterface.new(Difficulty.Normal);

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
}

function updateResourceRankings() {
  state.gameState.resourceRankings = display.resourceRankings();
}

// Start a new run
function newRun() {
  game = GameInterface.new(Difficulty.Normal);
  updateState();
  updateResourceRankings();
}

// Step the game by one year
function step() {
  let completedProjects = game.step();
  updateState();
  updateResourceRankings();
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
  updateResourceRankings();
}

function stopProject(projectId) {
  game.stop_project(projectId);
  updateState();
  updateResourceRankings();
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

// Apply event effects
function applyEvent(eventId, regionId) {
  game.apply_event(eventId, regionId);
  updateState();
  updateResourceRankings();
}

function applyBranchEffects(eventId, regionId, branchId) {
  game.apply_branch_effects(eventId, regionId, branchId);
  updateState();
  updateResourceRankings();
}

function evalBranchConditions(eventId, regionId, branchId) {
  return game.eval_branch_conditions(eventId, regionId, branchId);
}

function upgradeProject(id) {
  game.upgrade_project(id);
  updateState();
  updateResourceRankings();
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
  icon: () => {
    return _roll('Icon', '', null);
  },
}

updateState();
updateResourceRankings();

export default {
  newRun, step,
  updateState, setTgav,
  setPriority,
  changePoliticalCapital,
  changeLocalOutlook,
  checkRequests,
  banProcess, unbanProcess,
  promoteProcess, unpromoteProcess,
  setProjectPoints, startProject, stopProject, upgradeProject,
  applyEvent, roll, simulate,
  applyBranchEffects, evalBranchConditions,
  industryDemand, regionDemand, regionHabitability,
  yearsRemaining, updateResourceRankings};
