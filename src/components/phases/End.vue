<template>
<div class="break">
  <Dialogue v-if="hasDialogue" v-bind="event" @done="showStart = true" />
  <div class="badges-section" v-if="badges && showStart">
    <div class="badges">
      <img v-for="badge in badges"
        :src="`/assets/badges/${badge.name}.png`"
        v-tip="{text: t(badge.desc)}" />
    </div>
  </div>
  <div class="break--actions" v-if="showStart">
    <h2>{{t(message)}}</h2>
    <button class='try-again-button' @click="startRun">{{t('Try Again?')}}</button>
  </div>
  <div v-if="shareImgUrl && showStart">
    <img class="share-image" crossorigin="anonymous" :src="shareImgUrl" />
    <a class="twitter-share-button" :href="`https://twitter.com/intent/tweet?text=${shareUrl}`" target="_blank">Tweet</a>
  </div>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import share from 'lib/share';
import debug from '/src/debug';
import {randChoice} from 'lib/util';
import EventsMixin from 'components/EventsMixin';

const MESSAGES = [
  'The world can still be salvaged...',
  'This is not the end...',
];

export default {
  props: ['lose'],
  mixins: [EventsMixin],
  mounted() {
    this.init();
  },
  activated() {
    this.init();
  },
  data() {
    let events = this.lose ? game.roll.break('Start') : game.roll.end('Start');
    return {
      badges: [],
      showStart: debug.hideEvents || events.length == 0 ? true : false,
      shareImgUrl: null,
      events,
    }
  },
  computed: {
    message() {
      if (this.lose) {
        return randChoice(MESSAGES);
      } else {
        return 'Well Played!';
      }
    }
  },
  methods: {
    init() {
      this.showEvent();
      this.getShareImage();
      game.clearSave();
    },
    startRun() {
      window.location.reload();
      /* game.newRun(true); */
      /* state.phase = 'PLANNING'; */
    },
    getShareImage() {
      share(!this.lose, (data) => {
        if (data.success) {
          let {badges, url, image} = data;
          this.shareImgUrl = image;
          this.shareUrl = url;
          this.badges = badges;
        }
      });
    },
  },
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
  margin: 0 0 0.3em 0;
}
.badges-section {
  text-align: center;
  margin: 0;
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
  width: 80px;
  font-size: 0.9em;
  margin: 0em auto 1em auto;
  padding: 0.35em 0;
  border-radius: 0.3em;
  border-left: 2px solid #638ba4;
  border-top: 2px solid #638ba4;
  border-right: 2px solid #1b587e;
  border-bottom: 2px solid #1b587e;
  text-decoration: none;
}
.twitter-share-button:hover {
  background: #177dbd;
}

.share-image {
  display: block;
  max-width: 500px;
  margin: 0 auto 1em;
  border-radius: 0.4em;
}

.try-again-button{
  font-family: 'Times Ten', serif;
}
</style>
