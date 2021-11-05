<template>
<div class="card">
  <header>
    <slot name="header"></slot>
  </header>
  <div class="card--body">
    <slot v-if="!flipped" name="front"></slot>
    <slot v-else name="back"></slot>
  </div>
  <footer>
    <slot name="footer"></slot>
    <img :src="`/assets/placeholders/${flipped ? 'flip.svg' : 'info.svg'}`" @click="flipped = !flipped">
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
  padding: 0.5em 0.5em 0.25em;
  background: #222222;
  position: relative;
  width: 320px;
  height: 400px;
  display: flex;
  flex-direction: column;

  /* TEMPORARY */
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
}

.card-actions {
  display: flex;
  justify-content: space-around;
}
.card-actions button {
  font-size: 1.1em;
}
</style>
