<template>
<div class="card" @click="flipped = !flipped">
  <div class="card-top" :style="{background, color}">
    <header :style="{color}">
      <slot name="header"></slot>
    </header>
    <figure v-if="!flipped">
      <slot name="figure"></slot>
    </figure>
    <div v-else class="card-top-back">
      <slot name="top-back"></slot>
    </div>
  </div>
  <div class="card-mid card--name" :style="{background, color}">
    <div :style="{visibility: flipped ? 'hidden' : 'visible'}">
      <slot name="name"></slot>
    </div>
  </div>
  <div class="card-bot" :style="{background, color}">
    <div v-if="!flipped" class="card--body" :style="{color}">
      <slot name="body"></slot>
    </div>
    <div v-else class="card-bot-back">
      <slot name="bot-back"></slot>
    </div>
  </div>
</div>
</template>

<script>

export default {
  props: ['background', 'color'],
  data() {
    return {
      flipped: false
    }
  },
  computed: {
    hasExtras() {
      return !!this.$slots.extras;
    }
  },
  methods: {
    flip(ev) {
      this.flipped = !this.flipped;
      ev.stopImmediatePropagation();
    }
  }
}
</script>

<style>
.card {
  position: relative;
  width: 280px;
  height: 420px;
  max-height: 70vh;
  display: flex;
  flex-direction: column;
  margin: 0 auto;
}

.card-top,
.card-mid,
.card-bot {
  background: #222;
  padding: 0.25em 0.5em;
}

.card-top {
  flex: 1;
  display: flex;
  flex-direction: column;
  border-radius: 0.75em 0.75em 0.1em 0.1em;
  box-shadow: 0 0 3px rgb(0 0 0 / 50%);
}
.card-bot {
  flex: 1;
  border-radius: 0.1em 0.1em 0.75em 0.75em;
  box-shadow: 0px 1px 1px rgb(0 0 0 / 50%);
  display: flex;
}

.card-mid {
  margin: 0 auto;
  width: calc(100% - 6px);
  box-shadow: 0 1px 2px rgba(0,0,0,0.5);
}

.card header {
  display: flex;
  justify-content: space-between;
  align-items: center;

  font-size: 0.8em;
  text-transform: uppercase;

  color: #fff;
  padding: 0 0.3em;
}


.card header {
  margin-bottom: 0.25em;
}

.card--body {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  color: #fff;
  padding: 0.5em;
  border-right: 1px solid rgba(255,255,255,0.5);
  border-bottom: 1px solid rgba(255,255,255,0.5);
  border-top: 1px solid rgba(0,0,0,0.4);
  border-left: 1px solid rgba(0,0,0,0.4);
  border-radius: 0.5em;
  margin: 0 0 0.5em 0;
  font-family: 'Inter', sans-serif;
  font-size: 0.8em;
}
.card--body p img {
  width: 18px;
  vertical-align: middle;
  margin-right: 2px;
}

.card figure {
  position: relative;
  border-right: 1px solid rgba(255,255,255,0.5);
  border-bottom: 1px solid rgba(255,255,255,0.5);
  border-top: 1px solid rgba(0,0,0,0.4);
  border-left: 1px solid rgba(0,0,0,0.4);
  border-radius: 12px;
  box-shadow: inset 2px 1px 0px rgb(0 0 0 / 60%);
  height: 171px;
  overflow: hidden;
}
.card-image {
  border-radius: 10px;
  pointer-events: none; /* prevent dragging */
  display: block;
  border-left: 1px solid #555;
}
.card-image-attribution {
  color: #787087;
  font-family: 'Inter', sans-serif;
  font-size: 0.6em;
  text-transform: uppercase;
  width: 100%;
  text-align: center;
}

.card-actions {
  display: flex;
  justify-content: space-around;
}
.card-actions button {
  font-size: 1.1em;
}

.supporters,
.opposers {
  bottom: 0.5em;
  position: absolute;
  text-align: center;
  font-size: 0.9em;
  background: rgba(0,0,0,0.4);
  line-height: 0;
  border-radius: 0.2em;
  color: #fff;
}
.opposers {
  left: 0.5em;
  border-bottom: 3px solid #D60000;
}
.opposers > div:first-child {
  background: #EF3838;
}
.supporters {
  right: 0.5em;
  border-bottom: 3px solid #30E863;
}
.supporters > div:first-child {
  background: #43CC70;
}
.supporters img,
.opposers img {
  width: 24px;
}

.card-icon {
  width: 30px;
  text-align: center;
}

.card-tack-ul {
  position: absolute;
  left: 0.5em;
  top: 0.5em;
}
.card-tack-ur {
  position: absolute;
  right: 0.5em;
  top: 0.5em;
}
.card-tack-cb {
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translate(-50%, 50%);
}

.card header {
  position: relative;
}
.card header img {
  width: 16px;
  vertical-align: middle;
  margin-top: -2px;
  margin-left: 1px;
}
.card header > div:first-child {
  font-family: 'W95FA', monospace;
}

.card--name {
  text-align: center;
  font-size: 1.5em;
  padding: 0.5em 0;
}

.card header .barcode {
  position: absolute;
  top: -2px;
  width: 130px;
  left: 50%;
  transform: translate(-50%, 0);
}

.card-desc {
  font-family: 'Inter', sans-serif;
  font-size: 0.8em;
  text-align: center;
}

.card-top-back {
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  align-items: center;
  height: 171px;
}
.card-bot-back {
  width: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  align-items: center;
}
</style>
