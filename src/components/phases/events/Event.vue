<template>
<div class="event" :style="{backgroundImage: `url(/assets/content/images/${event.image ? event.image.fname : 'missing_image.png'})`}">
  <div class="event--body">
    <div class="arc">{{event.arc}}</div>
    <div class="image-attribution">Image source: {{event.image ? event.image.attribution : ''}}</div>
    <div class="event--name">{{event.name}}</div>
    <ul class="effects">
      <template v-for="desc in effectDescs">
        <li v-html="desc"></li>
      </template>
    </ul>
  </div>
  <Dialogue v-if="event.dialogue" :dialogue="event.dialogue" :effects="[]" @done="done" @select="selectChoice" />
</div>
</template>

<script>
import Dialogue from 'components/Dialogue.vue';
import {describeEffect} from '/src/effects';

export default {
  props: ['event'],
  components: {
    Dialogue
  },
  activated() {
    console.log('Activated');
    console.log(this.event);
  },
  mounted() {
    console.log('Mounted');
    console.log(this.event);
  },
  computed: {
    effectDescs() {
      return this.event.effects
        .map((ev) => describeEffect(ev))
        .filter((desc) => desc !== undefined);
    }
  },
  methods: {
    done() {
      console.log('DIALOGUE-EVENT FINISHED');
      this.$emit('done');
    },
    selectChoice() {
    }
  }
}
</script>

<style>
.event {
  position: relative;
  overflow: hidden;
  margin: 8vh 2em;
  background: #222;
  height: 80vh;
  border-radius: 0.75em;
  border: 1px solid #222;
  background-size: cover;
  background-position: center center;
  z-index: 10;
  box-shadow: 2px 2px 6px rgba(0, 0, 0, 0.7);
}
.event--body {
  color: #fff;
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
.event .effects {
  margin: 2em;
  background: #222;
  padding: 0.5em;
  border-radius: 0.35em;
  font-family: 'Andada Pro';
  line-height: 1.4;
}
.event .effects img {
  width: 22px;
  vertical-align: middle;
}
.event .effects li {
  margin: 0 0 0.5em 0;
}
.event .effects li:last-child {
  margin-bottom: 0;
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
</style>
