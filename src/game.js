import state from './state';
import display from 'lib/display';
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
function stepUpdate() {
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

// Select a response to an event
function selectChoice(eventId, regionId, choiceId) {
  game.set_event_choice(eventId, regionId, choiceId);
  updateState();
}

// Apply event effects
function applyEvent(eventId, regionId) {
  game.apply_event(eventId, regionId);
  updateState();
  updateResourceRankings();
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

const roll = {
  planningEvents: () => game.roll_planning_events(),
  worldStartEvents: () => game.roll_world_start_events(),
}

updateState();
updateResourceRankings();

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
  industryDemand, regionDemand, regionHabitability,
  yearsRemaining, updateResourceRankings};