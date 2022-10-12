<template>
<div>
  <div id="start-bg"></div>
  <div id="start-screen">
    <div id="lang-select">
      <select @change="selectLang">
        <option value="en" :selected="lang == 'en'">EN</option>
        <option value="pt-br" :selected="lang == 'pt-br'">PT-BR</option>
      </select>
    </div>
    <template v-if="showCredits">
      <Credits @click="showCredits = false" />
    </template>
    <div id="start-screen-inset">
      <div id="start-inner">
        <img src="/assets/intro.svg" />
        <div class="start-subtitle">{{t('A Planetary Crisis Planning Game')}}</div>
        <button class="start-button" v-if="hasSave()" @click="continueGame">{{t('Continue')}}</button>
        <button class="start-button" @click="startGame">{{t('New Game')}}</button>
        <div class="two-buttons">
          <button class="start-button" @click="toggleSound">{{t('Sound')}}: {{sound() ? t('On'): t('Off')}}</button>
          <hr />
          <button class="start-button" @click="showCredits = true">{{t('Credits')}}</button>
        </div>
        <a class="book-line" v-if="PLATFORM == 'STEAM'">
          <span>{{t('Based on the book')}}:&nbsp;<em>Half-Earth Socialism</em>.</span>
        </a>
        <a class="book-line " target="_blank" href="https://www.versobooks.com/books/3818-half-earth-socialism" v-else>
          <span>{{t('Read the book')}}:&nbsp;<em>Half-Earth Socialism</em>.</span>
        </a>
      </div>
    </div>
  </div>
</div>
</template>

<script>
import {lang} from '/src/i18n';
import game from '/src/game';
import state from '/src/state';
import {saveSettings} from '../state';
import Credits from './Credits.vue';

export default {
  components: {
    Credits,
  },
  data() {
    return {
      lang,
      showCredits: false
    }
  },
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
    },
    selectLang(ev) {
      let lang = ev.target.value;
      localStorage.setItem('lang', lang);
      window.location.reload();
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
  font-size: 1.2em;
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
  /* margin-bottom: 3em; */
}

.start-subtitle {
  font-size: 0.8em;
  margin: 0 0 1.5em 0;
}

.two-buttons {
  display: flex;
  max-width: 280px;
  margin: 0 auto;
}
.two-buttons hr {
  border: none;
  border-right: 1px solid rgba(255,255,255,0.7);;
  margin: 0 2px;
  margin-top: 12px;
}
.two-buttons button {
  margin-top: 0 !important;
}

#start-screen .credits {
  position: fixed;
  left: 0;
  right: 0;
  bottom: 0;
  top: 0;
  background: #111;
  z-index: 10;
  overflow-y: auto;
}

.book-line {
  color: #fff;
  text-decoration: none;
  padding: 0.4rem 0.5rem;
  text-transform: none;
  font-size: 0.9rem;
  margin-top: 1rem;
  background: rgba(0,0,0,0.2);
  border-radius: 0.25em;
  box-shadow: 2px 0px 5px rgba(0,0,0,0.1);
  transition: all 50ms ease-out;
  letter-spacing: 0.01em;
  display: flex;
  justify-content: center;
  width: 280px;
  margin:1rem auto 0;
}

.book-line span{
  border-bottom: 1px solid white;
}

.book-line:hover span{
  border-bottom: 1px solid #111;
}

.book-line:hover{
  background-color: rgba(0,0,0,0.9);
  box-shadow: 0px 0px 10px #B9F80D;
  background: #B9F80D;
  color:#111;
}



#start-inner .start-button:hover{
  color:rgba(255,255,255,0.7);
}

#lang-select {
  position: fixed;
  right: 1em;
  top: 1em;
  z-index: 2;
}
#lang-select select {
  background: #ecc1dc;
  color: #000;
  border: 1px solid #d4a5d8;
  padding: 0.2em;
  border-radius: 2px;
}

#lang-note {
  position: fixed;
  padding: 0.5em;
  top: 0.5em;
  width: 420px;
  left: 50%;
  transform: translate(-50%, 0);
  background: rgba(0,0,0,0.8);
  z-index: 10;
}

@media only screen and (min-width: 481px) {
  #start-screen img{
    max-width:500px;
  }
  #start-screen button {
    max-width: 450px;
    font-size: 1.5em;
  }

  .two-buttons {
    max-width: 450px;
  }

  .book-line{
    bottom: 1rem;
    font-size: 1rem;
    padding: 0.75rem;
    width: 450px;
  }
}

</style>
