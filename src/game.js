import state from './state';
import {GameInterface, Difficulty} from 'half-earth-engine';

// TODO let player choose difficulty;
// also; this needs to be re-created for each run.
let game = GameInterface.new(Difficulty.Normal);
state.gameState = game.state();

function newRun() {
  game = GameInterface.new(Difficulty.Normal);
  state.gameState = game.state();
}

function step() {
  let events = game.step();
  state.gameState = game.state();
  return events;
}

function selectChoice(eventId, regionId, choiceId) {
  game.set_event_choice(eventId, regionId, choiceId);
  state.gameState = game.state();
}

function setProjectPoints(projectId, points) {
  game.set_project_points(projectId, points);
  state.gameState = game.state();
}


function startProject(projectId) {
  game.start_project(projectId);
  state.gameState = game.state();
}

function stopProject(projectId) {
  game.stop_project(projectId);
  state.gameState = game.state();
}

function banProcess(processId) {
  game.ban_process(processId);
  state.gameState = game.state();
}

function unbanProcess(processId) {
  game.unban_process(processId);
  state.gameState = game.state();
}

function rollPlanningEvents() {
  let events = game.roll_planning_events();
  state.gameState = game.state();
  return events;
}

function rollBreaksEvents() {
  let events = game.roll_breaks_events();
  state.gameState = game.state();
  return events;
}

function setTgav(tgav) {
  game.set_tgav(tgav);
  state.gameState = game.state();
}

export default {newRun, step, selectChoice,
  setProjectPoints, startProject, stopProject,
  banProcess, unbanProcess, setTgav,
  rollPlanningEvents, rollBreaksEvents};
