<template>
<div class="event">
  <div class="event--body" :style="{backgroundImage: `url(/assets/content/images/${event.image ? event.image.fname : 'missing_image.png'})`}">
    <div class="arc">{{event.arc}}</div>
    <div class="factors">
      <img class="factor" v-for="factor in event.factors" :src="icons[factor]" v-tip="{icon: factor, text: describeFactor(factor)}"/>
    </div>
    <div class="image-attribution">Image source: {{event.image ? event.image.attribution : ''}}</div>
    <div class="event--name">{{event.name}}</div>
    <div class="event--effects">
      <Effects :effects="event.effects" />
    </div>
  </div>
  <Dialogue v-bind="event" :effects="[]" @done="done" />
</div>
</template>

<script>
import display from 'lib/display';
import Effects from 'components/Effects.vue';
import Dialogue from 'components/Dialogue.vue';

export default {
  props: ['event'],
  components: {
    Effects,
    Dialogue
  },
  methods: {
    done() {
      this.$emit('done');
    },
    describeFactor(factor) {
      switch (factor) {
        case 'warming':
          return 'This event is influenced by the global temperature anomaly.';
        case 'contentedness':
          return 'This event is influenced by how happy people are.';
        case 'extinction_rate':
          return 'This event is influenced by biodiversity pressures.';
        case 'sea_level_rise':
          return 'This event is influenced by the amount of sea level rise.';
        case 'habitability':
          return 'This event is influenced by the habitability of regions.';
        default:
          return factor;
      }
    }
  },
}
</script>

<style>
.event {
  position: absolute;
  z-index: 10;
  bottom: 0;
  top: 0;
  background: rgba(0,0,0,0.75);
  padding: 8vh 1em;
  width: 100%;
}
.event--body {
  color: #fff;
  height: 40vh;
  border-radius: 0.75em;
  border: 1px solid #222;
  background-size: cover;
  background-position: center center;
  box-shadow: 2px 2px 6px rgba(0, 0, 0, 0.7);
  position: relative;
  background: #222;
}
.event .arc {
  text-transform: uppercase;
  text-align: center;
  font-size: 0.75em;
  padding: 0.25em 0;
  background: #222;
  display: block;
  margin: 1em auto;
  border-radius: 0.35em;
  width: 180px;
  font-family: 'Andada Pro';
}
.event .image-attribution {
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translate(-50%, 0);
  font-size: 0.6em;
  font-style: italic;
  width: 100%;
  text-align: center;
  text-shadow: 1px 1px 2px black;
  margin: 0.5em 0;
}

.event--name {
  text-align: center;
  background: #222222;
  margin: 0 1em;
  border-radius: 0.3em;
  padding: 0.2em 0;
  font-family: 'Andada Pro';
}

.event .dialogue {
  position: relative;
  background: none;
}

.factors {
  position: absolute;
  right: 0.5em;
  top: 0.85em;
}
.factors img {
  height: 22px;
  background: #222;
  border-radius: 1.2em;
  padding: 0.2em;
}

.event--effects {
  padding: 1em;
  background: #222;
  margin: 1em;
}
</style>
