<template>
<div class="card">
  <header>
    <slot name="header"></slot>
  </header>
  <figure v-if="!flipped">
    <slot name="figure"></slot>
  </figure>
  <div class="card--body">
    <slot v-if="!flipped" name="body"></slot>
    <slot v-else name="back"></slot>
  </div>
  <footer>
    <slot name="footer"></slot>
    <img :src="flipped ? icons.flip : icons.info" @click="flipped = !flipped">
  </footer>
</div>
</template>

<script>

export default {
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
  border-radius: 0.6em;
  box-shadow: 0 2px 2px rgba(0,0,0,0.8);
  padding: 0.25em 0.5em;
  background: #222222;
  position: relative;
  width: 320px;
  height: 420px;
  max-height: 70vh;
  display: flex;
  flex-direction: column;
  margin: 0 auto;
}

.card header,
.card footer {
  display: flex;
  justify-content: space-between;
  align-items: center;

  font-size: 0.8em;
  text-transform: uppercase;

  background: #222;
  color: #fff;
  padding: 0 0.3em;
  border-radius: 0.3em;
}
.card header {
  margin-bottom: 0.25em;
}
.card footer img {
  width: 28px;
}

.card--body {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  color: #fff;
}
.card--body p img {
  width: 18px;
  vertical-align: middle;
  margin-right: 2px;
}

.card figure {
  position: relative;
}
.card-image {
  border-radius: 0.3em;
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

.card header img {
  width: 12px;
  vertical-align: middle;
  margin-top: -2px;
}
</style>
