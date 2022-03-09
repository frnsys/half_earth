<template>
<div class="dropdown-menu">
  <div class="dropdown-menu-content">
    <div class="dropdown-menu-close dropdown-menu-button" @click="$emit('close')"><img src="/assets/icons/close.svg"></div>
    <header>
      <div class="dropdown-menu-inset">
        <img src="/assets/gosplant.svg" />
      </div>
    </header>
    <template v-if="showCredits">
      <Credits />
    </template>
    <template v-else>
      <div class="dropdown-menu-time">
        <img src="/assets/clock.png" />
        <div class="dropdown-menu-inset dropdown-menu-year">{{state.gameState.world.year}}</div>
      </div>
      <div class="dropdown-menu-inset dropdown-menu-stats">
        <div class="dropdown-menu-stat">
          <img :src="icons.political_capital" />
          <div class="dropdown-menu-stat-value">{{Math.max(state.gameState.political_capital, 0)}}</div>
        </div>
        <div class="dropdown-menu-stat">
          <img :src="icons.emissions" />
          <div class="dropdown-menu-stat-value">{{`${state.gameState.world.emissions.toFixed(1)}Gt`}}</div>
        </div>
        <div class="dropdown-menu-stat">
          <img :src="icons.warming" />
          <div class="dropdown-menu-stat-value">+{{state.gameState.world.temperature.toFixed(1)}}C</div>
        </div>
      </div>
      <div class="dropdown-menu-stats-labels">
        <div class="dropdown-menu-stats-label">Political Capital</div>
        <div class="dropdown-menu-stats-label">CO2 Emissions/Yr</div>
        <div class="dropdown-menu-stats-label">Temp. Anomaly</div>
      </div>
      <div class="dropdown-menu-bars">
        <div class="dropdown-menu-inset dropdown-menu-stat">
          <img :src="icons.extinction_rate" />
          <IntensityBar :intensity="extinction" :max="5" />
        </div>
        <div class="dropdown-menu-inset dropdown-menu-stat">
          <img :src="icons.contentedness" />
          <IntensityBar :intensity="contentedness" :max="5" :invert="true" />
        </div>
      </div>
      <div class="dropdown-menu-stats-labels">
        <div class="dropdown-menu-stats-label">Extinction Rate</div>
        <div class="dropdown-menu-stats-label">Contentedness</div>
      </div>
      <img class="motto" src="/assets/motto.png" />
      <div class="dropdown-menu-buttons">
        <div class="dropdown-menu-button" :class="{active: state.sound}" @click="toggleSound">Sound: {{ state.sound ? 'On' : 'Off'}}</div>
        <div class="dropdown-menu-button" :class="{active: !state.hideHelp}" @click="toggleTips">Tips: {{ !state.hideHelp ? 'On' : 'Off'}}</div>
        <div class="dropdown-menu-button" @click="restartGame">Restart Game</div>
        <div class="dropdown-menu-button" @click="showCredits = true">Credits</div>
      </div>
    </template>
  </div>
</div>
</template>

<script>
import game from '/src/game';
import state from '../state';
import {saveSettings} from '../state';
import IntensityBar from './cards/IntensityBar.vue';
import intensity from '/src/display/intensity';
import Credits from './Credits.vue';

export default {
  components: {
    Credits,
    IntensityBar,
  },
  data() {
    return {
      state,
      showCredits: false,
    };
  },
  computed: {
    contentedness() {
      return intensity.scale(state.gameState.world.contentedness, 'world_outlook');
    },
    extinction() {
      return intensity.scale(state.gameState.world.extinction_rate, 'extinction');
    },
  },
  methods: {
    toggleSound() {
      state.sound = !state.sound;
      if (state.sound && window.music.paused) {
        window.music.play();
      } else if (!state.sound && !window.music.paused) {
        window.music.pause();
      }
      saveSettings();
    },
    toggleTips() {
      state.hideHelp = !state.hideHelp;
      saveSettings();
    },
    restartGame() {
      if (confirm('Are you sure you want to start over?')) {
        game.clearSave();
        location.reload();
      }
    }
  }
}
</script>

<style>
.dropdown-menu {
  background-color: #1A1615;
  background-image: url('/assets/backgrounds/menu.jpg');
  background-size: cover;
  background-position: center;
  position: absolute;
  z-index: 20;
  left: 0;
  right: 0;
  bottom: 0;
  top: 0;
  padding: 1em 0.5em;
  overflow-y: auto;

  image-rendering: pixelated;
}

.dropdown-menu header {
  width: 100%;
  text-align: center;
}

header .dropdown-menu-inset {
  height: 48px;
  padding: 0.5em;
}

.dropdown-menu-inset {
  background: #423B3B;
  border-radius: 0.4em;
  border-top: 1px solid #1a1a1a;
  border-left: 1px solid #1a1a1a;
  border-bottom: 1px solid #7a7777;
  border-right: 1px solid #7a7777;
  display: inline-block;
}

.dropdown-menu-close {
  display: inline-block;
  width: 48px;
  height: 48px;
  position: absolute;
  top: 1em;
  right: 0.5em;
}
.dropdown-menu-button {
  background: #C4BAB4;
  border-right: 1px solid #1a1a1a;
  border-bottom: 1px solid #1a1a1a;
  border-top: 1px solid #FDF7E2;
  border-left: 1px solid #FDF7E2;
  border-radius: 0.6em;
  text-align: center;
  padding: 0.5em 0;
  cursor: pointer;
}
.dropdown-menu-button.active {
  background: #2FE863;
}
.dropdown-menu-buttons {
  text-transform: uppercase;
  font-size: 0.55em;
  font-family: 'Inter', sans-serif;
  font-weight: bold;
  display: flex;
}
.dropdown-menu-buttons > .dropdown-menu-button {
  flex: 1;
  margin: 0 0.1em;
  padding: 1.5em 0;
}
.dropdown-menu-buttons > .dropdown-menu-button:first-child {
  margin-left: 0;
}
.dropdown-menu-buttons > .dropdown-menu-button:last-child {
  margin-right: 0;
}

.dropdown-menu-stats {
  width: 100%;
  display: flex;
  margin: 2em 0 0.5em 0;
}

.dropdown-menu-stat {
  flex: 1;
  text-align: center;
  border-right: 1px solid #eee;
  border-left: 1px solid #000;
  margin: 0.5em 0;
  padding-top: 0.75em;
  position: relative;
}
.dropdown-menu-stat:first-child {
  border-left: none;
}
.dropdown-menu-stat:last-child {
  border-right: none;
}
.dropdown-menu-stat img {
  width: 28px;
  position: absolute;
  top: 0em;
  left: 50%;
  transform: translate(-50%, -80%);
  box-shadow: 0px 1px 2px rgb(0 0 0 / 50%);
  border-radius: 2em;
}

.dropdown-menu-stat-value {
  color: #fff;
  font-family: 'W95FA', monospace;
  font-size: 1.1em;
}

.dropdown-menu-stats-labels {
  display: flex;
}
.dropdown-menu-stats-label {
  flex: 1;
  text-align: center;
  color: rgba(255,255,255,0.6);
  text-transform: uppercase;
  font-size: 0.55em;
  font-family: 'Inter', sans-serif;
  font-weight: bold;
}

.dropdown-menu-bars {
  display: flex;
  margin: 2em 0 0;
}
.dropdown-menu-bars .dropdown-menu-stat {
  flex: 1;
}
.dropdown-menu-bars .dropdown-menu-stat:first-child {
  border-right: 1px solid #7a7777;
  margin-right: 0.5em;
}
.dropdown-menu-bars .dropdown-menu-stat:last-child {
  border-right: 1px solid #7a7777;
}
.dropdown-menu-bars .intensity-pips {
  width: 120px;
  margin: 0.5em auto 1em auto;
}
.dropdown-menu-bars .intensity-pip {
  height: 10px;
  width: 24px;
  background: #564C4C;
}

.motto {
  display: block;
  margin: 1.5em auto;
  width: 270px;
}

.dropdown-menu-time {
  display: flex;
  margin-top: 1em;
}
.dropdown-menu-time img {
  height: 100px;
}
.dropdown-menu-year {
  color: #fff;
  width: 10em;
  text-align: center;
  height: 100px;
  font-size: 2em;
  display: flex;
  margin-left: 0.25em;
  align-items: center;
  justify-content: space-around;
  border-radius: 10em;
}

.dropdown-menu-content {
  max-width: 420px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  min-height: 100%;
}

@media only screen and (min-width: 480px) {
  .dropdown-menu{
    display: flex;
  }
  .dropdown-menu-content {
    min-height: auto;
    align-self: center;
  }
}
</style>
