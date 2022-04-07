<template>
<div class="break">
  <Dialogue v-if="hasDialogue" v-bind="event" @done="show = true" />
  <div class="break--actions" v-if="show">
    <h2>Well Played!</h2>
    <button @click="startRun">Try Again?</button>
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
import {randChoice} from 'lib/util';
import EventsMixin from 'components/EventsMixin';

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
      show: false,
      events: game.roll.end('Start')
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
}
</script>
