<template>
<div class="break">
  <Dialogue v-if="hasDialogue" v-bind="event" @done="showStart = true" />
  <div class="break--actions" v-if="showStart">
    <h2>{{message}}</h2>
    <button @click="startRun">Try Again</button>
  </div>
  <div v-if="shareImgUrl && showStart">
    <img :src="shareImgUrl" />
    <div class="badges-section">
      <h3>Badges</h3>
      <div class="badges">
        <img v-for="badge in badges"
          :src="`/assets/badges/${badge.name}.png`"
          v-tip="{text: badge.desc}" />
      </div>
    </div>
    <a class="twitter-share-button" :href="`https://twitter.com/intent/tweet?text=${shareImgUrl}`" target="_blank">Tweet</a>
  </div>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import share from 'lib/share';
import debug from '/src/debug';
import EventsMixin from 'components/EventsMixin';
import {randChoice} from 'lib/util';

const MESSAGES = [
  'The world can still be salvaged...',
  'This is not the end...',
];

export default {
  mixins: [EventsMixin],
  mounted() {
    this.init();
  },
  activated() {
    this.init();
  },
  data() {
    return {
      badges: [],
      showStart: debug.hideEvents ? true : false,
      shareImgUrl: null,
      events: game.roll.break('Start')
    }
  },
  methods: {
    init() {
      this.showEvent();
      this.getShareImage();
      game.clearSave();
    },
    startRun() {
      game.newRun(true);
      state.phase = 'PLANNING';
    },
    getShareImage() {
      share(false, ({badges, url}) => {
        this.shareImgUrl = url;
        this.badges = badges;
      });
    },
  },
  computed: {
    message() {
      return randChoice(MESSAGES);
    }
  }
}
</script>

<style>
.break {
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  height: 100vh;
}
.break h2 {
  margin-top: 0;
  text-transform: uppercase;
  font-style: italic;
  font-weight: normal;
}

.break--actions {
  color: #fff;
  text-align: center;
  margin: 2em;
}

.break .dialogue {
  background: none;
}

.badges-section h3 {
  font-family: 'Inter', sans-serif;
  text-transform: uppercase;
  font-size: 0.7em;
  color: #fff;
  text-align: center;
}
.badges-section {
  text-align: center;
  margin: 0.5em 0;
}
.badges img {
  width: 32px;
  margin: 0 0.1em;
}

.twitter-share-button {
  display: block;
  text-align: center;
  color: #fff;
  background: #1EA1F2;
  width: 120px;
  margin: 0em auto 1em auto;
  padding: 0.35em 0;
  border-radius: 0.3em;
  border-left: 1px solid #b0d9f3;
  border-top: 1px solid #b0d9f3;
  border-right: 1px solid #1b587e;
  border-bottom: 1px solid #1b587e;
  text-decoration: none;
}
.twitter-share-button:hover {
  background: #177dbd;
}
</style>
