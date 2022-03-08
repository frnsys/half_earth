<template>
<div id="loading">
  <div>
    <img src="/assets/gosplant.svg" />
    <div class="loading-text">{{state.loadingSave ? 'Loading saved data' : 'Booting Up'}}</div>
    <div class="loading-bar">
      <div class="loading-bar-fill" :style="{width: barWidth}"></div>
    </div>
    <img class="motto" src="/assets/motto.png" />

    <div class='fonts'>
      <span style='font-family: "W95FA"'>a</span>
      <span style='font-family: "Inter"'>a</span>
      <span style='font-family: "Times Ten"'>a</span>
    </div>
  </div>
</div>
</template>

<script>
import state from '/src/state';
import icons from 'components/icons';

import PRELOAD_ASSETS from '/assets/content/preload_assets.json';

var preload = [
  '/assets/stamp.svg',
  '/assets/backgrounds/menu.jpg',
  '/assets/backgrounds/dashboard.png',
  '/assets/backgrounds/parliament.png',
  '/assets/backgrounds/plan.png',
  '/assets/backgrounds/regions.png',
  '/assets/backgrounds/report.png',
  '/assets/icons/close.svg',
  '/assets/gosplant.svg',
  '/assets/clock.png',
  '/assets/motto.png',
];

PRELOAD_ASSETS.forEach(asset => {
  preload.push(asset);
});

Object.values(icons).forEach((icon) => preload.push(icon));

export default {
  data() {
    return {
      state,
      loaded: 0,
    }
  },
  mounted() {
    preload.forEach((src) => {
      let img = new Image();
      img.onload = () => {
        this.loaded++;
        if (this.loaded >= preload.length) {
          this.$emit('loaded');
        }
      };
      img.src = src;
    });
  },
  computed: {
    barWidth() {
      return `${this.loaded/preload.length * 100}%`;
    }
  }
}
</script>

<style>
#loading {
  color: #fff;
  position: fixed;
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
  display: flex;
  align-items: center;
  text-align: center;
}

#loading > div {
  width: 100%;
}

.loading-text {
  font-size: 0.8em;
}

.fonts{
  position: absolute;
  left: -1000px;
}

/* https://stackoverflow.com/a/28074607 */
.loading-text:after {
  overflow: hidden;
  display: inline-block;
  vertical-align: bottom;
  animation: ellipsis steps(4,end) 2000ms infinite;
  content: "\2026";
  width: 0px;
}
@keyframes ellipsis {
  to {
    width: 20px;
  }
}

.loading-bar {
  height: 7px;
  width: 220px;
  margin: 0.5em auto;
  border: 1px solid #fff;
  border-radius: 0.5em;
  overflow: hidden;
}
.loading-bar-fill {
  height: 100%;
  background: #fff;
}
</style>
