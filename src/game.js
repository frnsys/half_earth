import state from './state';
import {GameInterface, Difficulty} from 'half-earth-engine';

// TODO let player choose difficulty;
// also; this needs to be re-created for each run.
let game = GameInterface.new(Difficulty.Normal);

function updateState() {
  state.gameState = game.state();

  let world = state.gameState.world;
  state.gameState.contentedness = world.regions.reduce((acc, r) => {
      return acc + r.base_contentedness + r.outlook;
    }, 0)/world.regions.length;
  state.gameState.emissions = (world.co2_emissions + (world.n2o_emissions * 298.) + (world.ch4_emissions * 36.)) * 1e-15;
}


function newRun() {
  game = GameInterface.new(Difficulty.Normal);
  updateState();
}

function step() {
  let events = game.step();
  updateState();
  return events;
}

function selectChoice(eventId, regionId, choiceId) {
  game.set_event_choice(eventId, regionId, choiceId);
  updateState();
}

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

function rollPlanningEvents() {
  let events = game.roll_planning_events();
  // updateState();
  return events;
}

function rollBreaksEvents() {
  let events = game.roll_breaks_events();
  // updateState();
  return events;
}

function rollIconEvents() {
  let events = game.roll_icon_events();
  // updateState();
  return events;
}

function applyEvent(eventId, regionId) {
  game.apply_event(eventId, regionId);
  updateState();
}

function setTgav(tgav) {
  game.set_tgav(tgav);
  updateState();
}

updateState();
export default {newRun, step, selectChoice,
  setProjectPoints, startProject, stopProject,
  banProcess, unbanProcess, setTgav,
  rollPlanningEvents, rollBreaksEvents, rollIconEvents,
  applyEvent};
