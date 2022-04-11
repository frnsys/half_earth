<template>
<div>
  <div id="start-bg"></div>
  <div id="start-screen">
    <div id="start-screen-inset">
      <div id="start-inner">
        <img src="/assets/intro.svg" />
        <div class="start-subtitle">A Planetary Crisis Planning Game</div>
        <button v-if="hasSave()" @click="continueGame">Continue</button>
        <button @click="startGame">New Game</button>
        <button @click="toggleSound">Sound: {{sound() ? 'On': 'Off'}}</button>
      </div>
    </div>
  </div>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import {saveSettings} from '../state';

export default {
  methods: {
    hasSave() {
      return game.hasSave();
    },
    startGame() {
      game.newRun(true);
      this.$emit('started');
    },
    continueGame() {
      game.newRun(false);
      this.$emit('started');
    },
    toggleSound() {
      state.sound = !state.sound;
      if (!state.sound) {
        window.audioManager.mute();
      } else {
        window.audioManager.unmute();
        window.audioManager.playOneShot('/assets/sounds/notification.wav');
      }
      saveSettings();
    },
    sound() {
      return state.sound;
    }
  },
}
</script>

<style>
#start-screen {
  position: fixed;
  left: 0;
  right: 0;
  bottom: 0;
  top: 0;
  background: rgba(0,0,0,0.7);
  text-align: center;
  color: #fff;
  overflow-y: auto;
}
#start-bg {
  position: fixed;
  left: 0;
  right: 0;
  bottom: 0;
  top: 0;
  background-image: url(/assets/backgrounds/start.png);
  background-size: cover;
  background-position: center center;
  image-rendering: pixelated;
}
#start-screen img {
  display: block;
  margin: 0em auto 1em;
  width: 100%;
  max-width: 260px;
  max-height: 55vh;
}
#start-screen button {
  display: block;
  margin: 0.5em auto;
  width: 100%;
  max-width: 280px;
  background: none;
  border: none;
  border-top: 1px solid rgba(255,255,255,0.7);
  border-radius: 0;
  color: #fff;
  font-size: 1.5em;
  font-family: 'Times Ten';
  padding-top: 1em;
}

#start-screen-inset {

  position: absolute;
  left: 1rem;
  right: 1rem;
  bottom: 1rem;
  top: 1rem;

  padding: 1em;
  /* margin: 1em; */
  border-radius: 2em;
  border-top: 1px solid #000;
  border-left: 1px solid #000;
  border-bottom: 1px solid #888;
  border-right: 1px solid #888;
  /* min-height: calc(100vh - 2em); */
  display: flex;
  flex-direction: column;
  justify-content: center;
}

#start-inner{
  margin-bottom: 3em;
}

.start-subtitle {
  font-size: 0.8em;
  margin: 0 0 1.5em 0;
}

@media only screen and (min-width: 481px) {
  #start-screen img{
    max-width:500px;
  }
  #start-screen button {
    max-width: 450px;
  }
}
</style>
