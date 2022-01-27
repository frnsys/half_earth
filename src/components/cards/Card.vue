<template>
<div class="card" @click="flipped = !flipped">
  <div class="card-top" :style="{background: background}">
    <header>
      <slot name="header"></slot>
    </header>
    <figure v-if="!flipped">
      <slot name="figure"></slot>
    </figure>
  </div>
  <div class="card-mid card--name" :style="{background: background}">
    <slot name="name"></slot>
  </div>
  <div class="card-bot" :style="{background: background}">
    <div class="card--body">
      <slot v-if="!flipped" name="body"></slot>
      <slot v-else name="back"></slot>
    </div>
  </div>
</div>
</template>

<script>

export default {
  props: ['background'],
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
  width: 320px;
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
  border-radius: 0.75em 0.75em 0.1em 0.1em;
}
.card-bot {
  border-radius: 0.1em 0.1em 0.75em 0.75em;
}

.card-mid {
  margin: 0 auto;
  width: calc(100% - 6px);
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
  border-radius: 0.5em;
}
.card-image {
  border-radius: 0.5em;
  pointer-events: none; /* prevent dragging */
}
.card-image-attribution {
  font-size: 0.8em;
  color: #787087;
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
  bottom: 0;
  position: absolute;
  text-align: center;
  font-size: 0.9em;
  padding: 0 0.25em;
  background: rgba(0,0,0,0.2);
  margin: 0.25em;
  line-height: 1;
  border-radius: 0.2em;
  color: #fff;
}
.opposers {
  left: 0;
}
.opposers > div:first-child {
  background: #EF3838;
}
.supporters {
  right: 0;
}
.supporters > div:first-child {
  background: #43CC70;
}
.supporters img,
.opposers img {
  width: 24px;
  margin: 0.25em 0;
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

.card header img {
  width: 16px;
  vertical-align: middle;
  margin-top: -2px;
  margin-left: 1px;
}
.card header > div:first-child {
  font-family: 'Apple ][', monospace;
  font-size: 1.4em;
}

.card--name {
  text-align: center;
  font-size: 1.5em;
  padding: 0.5em 0;
}
</style>
