<template>
<li class="card" :style="{backgroundImage: `url('${image}')`}">
  <div>
    <header>
      <slot name="header"></slot>
    </header>
  </div>
  <template v-if="!flipped">
    <div class="card--body">
      <div class="card--flag" v-if="flag">{{flag}}</div>
      <div class="card--title">{{title}}</div>
      <ul v-if="effects" class="effects">
        <template v-for="desc in effects">
          <li v-html="desc"></li>
        </template>
      </ul>
    </div>
    <slot name="front"></slot>
  </template>
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
  props: ['image', 'title', 'effects', 'flag'],
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
  background: #FFE0D7;
  position: relative;
  width: 310px;
  height: 400px;
  flex-direction: column;
  justify-content: space-between;

  background-size: cover;
  background-position: center;
}

.card--title {
  text-align: center;
  font-size: 1.2em;
  margin: 0.5em 0;
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
.card footer {
  font-size: 0.7em;
}

.card--toggle img {
	height: 18px;
	background: #222;
	padding: 0.25em 0.55em;
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
.card--back--body {
  flex: 1;
  justify-content: space-around;
  display: flex;
  flex-direction: column;
}
.card .effects {
  border-radius: 0.3em;
  padding: 0.5em;
  font-size: 0.8em;
}

.card--body {
  background: #222;
  color: #fff;
  padding: 0.25em;
  border-radius: 0.3em;
  position: relative;
}

.card .image-attribution {
  font-size: 0.8em;
  font-style: italic;
}

.card--actions {
  text-align: center;
}

.card--flag {
  position: absolute;
  top: 0;
  left: 50%;
  transform: translate(-50%, -50%);
  background: #222;
  color: #fff;
  border-radius: 0.3em;
  font-size: 0.65em;
  padding: 0 0.25em;
  border: 1px solid #ffecc7;
  text-transform: uppercase;
}
</style>
