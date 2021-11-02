import state from './state';
import {GameInterface, Difficulty} from 'half-earth-engine';

// TODO let player choose difficulty;
// also; this needs to be re-created for each run.
let game = GameInterface.new(Difficulty.Normal);

// Get the updated game state,
// and compute some additional variables
function updateState() {
  state.gameState = game.state();

  let world = state.gameState.world;
  state.gameState.contentedness = world.regions.reduce((acc, r) => {
    return acc + r.base_contentedness + r.outlook;
  }, 0);
  state.gameState.emissions = (world.co2_emissions + (world.n2o_emissions * 298.) + (world.ch4_emissions * 36.)) * 1e-15;
  state.gameState.population = world.regions.reduce((acc, r) => {
      return acc + r.population
    }, 0);

  // Aggregate autoclicker effects into single probabilities
  let autoclicker_effects = game.active_autoclickers();
  let autoclickers = {};
  autoclicker_effects.forEach(({AutoClick}) => {
    let id = AutoClick[0];
    let chance = AutoClick[1]/100;
    if (!(id in autoclickers)) {
      autoclickers[id] = [];
    }
    autoclickers[id].push(chance);
  });
  Object.keys(autoclickers).forEach((id) => {
    autoclickers[id] = 1 - autoclickers[id].reduce((acc, p) => {
      return acc * (1 - p);
    }, 1);
  });
  state.gameState.autoclickers = autoclickers;
}


// Start a new run
function newRun() {
  game = GameInterface.new(Difficulty.Normal);
  updateState();
}

// Step the game by one year
function step() {
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

function rollPlanningEvents() {
  return game.roll_planning_events();
}

function rollReportEvents() {
  return game.roll_report_events();
}

function rollBreaksEvents() {
  return game.roll_breaks_events();
}

function rollIconEvents() {
  return game.roll_icon_events();
}

function rollWorldEvents() {
  return game.roll_world_events();
}

function rollWorldStartEvents() {
  return game.roll_world_start_events();
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

// Check what requests were filled
function checkRequests() {
  return game.check_requests();
}

function upgradeProject(id) {
  game.upgrade_project(id);
  updateState();
}

function setTgav(tgav) {
  game.set_tgav(tgav);
  updateState();
}

function totalIncomeLevel() {
  return game.total_income_level();
}

updateState();

export default {
  newRun, step,
  setTgav,
  totalIncomeLevel,
  changePoliticalCapital,
  changeLocalOutlook,
  banProcess, unbanProcess,
  promoteProcess, unpromoteProcess,
  setProjectPoints, startProject, stopProject, upgradeProject,
  rollPlanningEvents, rollBreaksEvents, rollIconEvents,
  rollWorldEvents, rollReportEvents, rollWorldStartEvents,
  selectChoice, applyEvent, checkRequests};
