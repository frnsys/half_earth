import state from './state';
import {initState} from './state';
import factors from '/src/display/factors';
import {GameInterface, Phase, Difficulty} from 'half-earth-engine';

// Version timestamp must be >= this value
const EXPIRED_TIMESTAMP = 1646115265;

// Would let player choose difficulty;
// except at this point implementing different difficulty levels
// will be way too complicated. Maybe revisit at some point.
// also; this needs to be re-created for each run.
let game;

// Get the updated game state,
// and compute some additional variables
function updateState() {
  state.gameState = game.state();
  state.gameState.industries.forEach((ind) => {
    ind.demand = game.industry_demand(ind.id);
  });
  state.points.research += game.collect_research_points();
}

function updateFactors() {
  state.factors = factors.rank();
}

// Start a new run
function newRun(reset) {
  game = GameInterface.new(Difficulty.Normal);
  if (reset) {
    clearSave();
  }

  let save = loadGame();
  if (reset || save === null) {
    let init = initState();
    Object.keys(init).forEach((k) => {
      state[k] = init[k];
    });
  } else {
    state.loadingSave = true;
    Object.keys(save.state).forEach((k) => {
      state[k] = save.state[k];
    });
    game.load_state(save.game);
  }
  let year = game.state().world.year;
  state.startYear = year;
  state.endYear = year + 100;
  updateState();
  updateFactors();
  loadMeta();
  return game
}

function saveGame() {
  // Don't need to copy the entire gameState
  let {gameState, ...substate} = state;
  let s = {
    state: substate,
    version: VERSION,
    version_timestamp: TIMESTAMP,
    game: game.get_save_state(),
  };
  localStorage.setItem('gameData', JSON.stringify(s));
}

function hasSave() {
  let data = loadGame();
  return data !== null;
}

function loadGame() {
  let data = localStorage.getItem('gameData');
  if (data !== null) {
    let parsed = JSON.parse(data);
    let invalid = parsed.version_timestamp < EXPIRED_TIMESTAMP;
    if (invalid) {
      return null;
    } else {
      return parsed;
    }
  } else {
    return null;
  }
}

function clearSave() {
  localStorage.removeItem('gameData');
}

// Step the game by one year
function step() {
  let [completedProjects, regionChanges] = game.step();
  updateState();
  // updateFactors();
  return {
    completedProjects,
    regionChanges
  };
}

function stepCycle() {
  game.step_cycle();
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

function applyEvents(events) {
  events.forEach(({eventId, regionId}) => {
    game.apply_event(eventId, regionId);
  });
  updateState();
  updateFactors();
}

function applyIconEvents(events) {
  events.forEach(({eventId, regionId}) => {
    game.apply_event(eventId, regionId);
  });
  updateState();
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

function downgradeProject(id) {
  game.downgrade_project(id);
  updateState();
  updateFactors();
}

function rollNewPolicyOutcomes() {
  let ids = game.roll_new_policy_outcomes();
  updateState();
  updateFactors();
  return ids;
}

function setTgav(tgav) {
  game.set_tgav(tgav);
  updateState();
}

function simulate(years) {
  return game.simulate(years);
}

function checkRequests() {
  return game.check_requests();
}

function isAlly(name) {
  return state.gameState.npcs.find((npc) => npc.name == name).is_ally;
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
    return _roll('World', subphase, 1);
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

function playerSeats() {
  return state.gameState.npcs.filter((npc) => npc.relationship >= 5).reduce((acc, npc) => {
    return acc + npc.seats;
  }, 0);
}

// Save/load game metadata
function saveMeta() {
  let data = {
    runsPlayed: state.gameState.runs,
  };
  localStorage.setItem('gameMeta', JSON.stringify(data));
}

function loadMeta() {
  let data = localStorage.getItem('gameMeta');
  if (data !== null) {
    let parsed = JSON.parse(data);
    game.set_runs_played(parsed.runsPlayed || 0);
    return parsed;
  }
}

export default {
  newRun, saveMeta, hasSave,
  saveGame, loadGame, clearSave,
  step, stepCycle,
  updateState, setTgav,
  changePoliticalCapital,
  changeLocalOutlook,
  changeHabitability,
  checkRequests,
  changeProcessMixShare,
  rollNewPolicyOutcomes,
  setProjectPoints, startProject, stopProject,
  upgradeProject, downgradeProject,
  applyEvent, applyEvents, applyIconEvents, roll, simulate,
  applyBranchEffects, evalBranchConditions,
  playerSeats, isAlly,
  updateFactors};
