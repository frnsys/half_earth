<template>
<div class="card" @click="flip" :class="{flipped : flipped, process : isProcess}">
  <div class="card-top" :style="{background, color}" v-if="hasFigure && hasHeader">
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
  <div v-if="hasName" class="card-mid card--name" :style="{background, color}">
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

  <div v-if="isProcess" class="process-mix-bar">
    <slot name="process-mix"></slot>
  </div>
</div>
</template>

<script>
export default {
  props: ['background', 'color', 'noBack', 'isProcess'],
  data() {
    return {
      flipped: false
    }
  },
  computed: {
    hasName() {
      return !!this.$slots.name;
    },
    hasHeader() {
      return !!this.$slots.header;
    },
    hasFigure() {
      return !!this.$slots.figure;
    }
  },
  methods: {
    flip(ev) {
      if (!this.noBack) {
        this.flipped = !this.flipped;
        ev.stopImmediatePropagation();
      }
    }
  },
}
</script>

<style>
.card {
  position: relative;
  width: 300px;
  height: 430px;
  max-height: 70vh;
  display: flex;
  flex-direction: column;
  margin: 0 auto;
  border-radius: 0.75em;
  -webkit-filter: drop-shadow(0px 7px 9px rgba(0,0,0,0));
  transition: all 250ms ease-out;

  transform-style: preserve-3d;

  cursor: pointer;
}

.card.process{
  width: 280px;
  margin:0 0.75em !important;
}

.card:hover{
  -webkit-filter: drop-shadow(0px 7px 9px rgba(0,0,0,0.4));
}


.card.flipped{
  transform: rotateY(180deg);
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
  /* box-shadow: 0 0 3px rgb(0 0 0 / 50%); */
  font-family: 'Inter', sans-serif;
  position: relative;
}

.card-top:after {
    content: '';
    position: absolute;
    top: -2px;
    left: -2px;
    bottom: -2px;
    right: -2px;
    border: 2px rgba(0,0,0,0.1) solid;
    border-radius: 0.85em 0.85em 0.1em 0.1em;
    pointer-events: none;
}

.card-bot {
  flex: 1;
  border-radius: 0.1em 0.1em 0.75em 0.75em;
  /* box-shadow: 0px 1px 1px rgb(0 0 0 / 50%); */
  display: flex;
  position: relative;
}

.card-bot::after {
    content: '';
    position: absolute;
    top: -2px;
    left: -2px;
    bottom: -2px;
    right: -2px;
    border: 2px rgba(0,0,0,0.1) solid;
    border-radius: 0.1em 0.1em 0.85em 0.85em;
    pointer-events: none;
}

.card-mid {
  margin: 0 auto;
  width: calc(100% - 6px);
  /* box-shadow: 0 2px 2px rgba(0,0,0,0.5); */
  box-shadow: 2px 0px 0px rgba(0,0,0,0.1), -2px 0px 0px rgba(0,0,0,0.1);
  z-index: 1;
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
  padding: 1.2em 0.5em;

  border-right: 1px solid rgba(255,255,255,0.5);
  border-bottom: 1px solid rgba(255,255,255,0.5);
  border-top: 1px solid rgba(0,0,0,0.4);
  border-left: 1px solid rgba(0,0,0,0.4);

  
  border-radius: 0.5em;
  margin: 0 0 0.5em 0;
  font-family: 'Inter', sans-serif;
  font-size: 0.8em;

  background-color: rgba(0,0,0,0.05);

  image-rendering: auto;
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
  border-radius: 5px;
  box-shadow: inset 2px 1px 0px rgb(0 0 0 / 60%);
  height: 190px;
  overflow: hidden;
}
.card-image {
  /* border-radius: 10px; */
  pointer-events: none; /* prevent dragging */
  display: block;
  border-left: 1px solid #555;
  object-fit: cover;
  width: 100%;
  height: 100%;
  image-rendering: pixelated;
}
.card-image-attribution {
  color: rgba(255,255,255,0.8);
  /* mix-blend-mode: difference; */
  font-family: 'Inter', sans-serif;
  font-size: 0.5em;
  letter-spacing: 0.02em;
  font-weight: 600;
  text-transform: uppercase;
  width: 100%;
  text-align: center;
  opacity: 0.8;
  background-color: rgba(0,0,0,0.1);
  border-radius: 2px;
  padding:2px;
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
  padding: 0.5em 0.5em;
  line-height: 110%;
}

.card header .barcode {
  position: absolute;
  top: -2px;
  width: 130px;
  left: 50%;
  transform: translate(-50%, 0);
}

.card-desc {
  font-family: 'Times Ten', serif;
  font-size: 1em;
  text-align: center;
  margin: 2em 1em 1em 1em;
  font-style: italic;
}

.card-top-back {
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  align-items: center;
  height: 190px;
  transform: rotateY(180deg);
}
.card-bot-back {
  width: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  align-items: center;
  transform: rotateY(180deg);
}

.card.flipped header{
  transform: rotateY(180deg);
}

.process-mix-bar{
  width: 20px;
  height: 100%;
  /* background-color: red; */
  background-color: rgba(0,0,0,0.4);
  position: absolute;
  right: -10px;
  z-index: -50;
  border-radius: 0 0.75em 0.75em 0;
}
</style>
