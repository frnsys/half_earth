<template>
<li class="card">
  <div>
    <header>
      <slot name="header"></slot>
    </header>
    <figure v-if="!flipped">
      <img :src="image" />
    </figure>
  </div>
  <div v-if="!flipped" class="card--title">{{title}}</div>
  <slot v-else name="back"></slot>
  <footer>
    <div class="card--flavor">Gosplant Analysis Report</div>
    <div class="card--toggle" @click="flip">
      <img v-if="!flipped" src="/assets/icons/info.svg" />
      <img v-else src="/assets/icons/back.svg" />
    </div>
  </footer>
  <div class="card--extras" v-if="hasExtras">
    <slot name="extras"></slot>
  </div>
</li>
</template>

<script>

export default {
  props: ['image', 'title'],
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
  padding: 0.75em 1em 0.25em;
  background: #FFE0D7;
  position: relative;
  width: 310px;
  height: 400px;
  flex-direction: column;
  justify-content: space-between;
}

.card--title {
  text-align: center;
  font-size: 1.8em;
  margin: 0.5em 0;
}

.card figure {
  margin: 0.5em 0;
}
.card figure img {
  border-radius: 0.5em;
  pointer-events: none;
}

.card header,
.card footer {
  display: flex;
  justify-content: space-between;
  align-items: center;

  font-size: 0.8em;
  text-transform: uppercase;
}

.card--toggle img {
	height: 24px;
	background: #BABABA;
	padding: 0.25em 0.66em;
	border-radius: 20em;
  cursor: pointer;
}

.card--extras {
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translate(-50%, 50%);
}
.card--action--cost img {
  width: 12px;
}
.card--tag {
	padding: 0.2em 0.5em;
	border-radius: 0.2em;
	font-size: 0.8em;
	color: #fff;
  background: #222;
	box-shadow: 0 2px 5px rgba(0,0,0,0.8);
  text-transform: uppercase;
}
</style>
