<template>
<Card>
  <template v-slot:header>
    <div>{{name}}</div>
    <div v-tip="{text: 'This process produces electricity.', icon: 'energy'}">{{consts.icons[output]}}</div>
  </template>
  <template v-slot:front>
    <figure>
      <img class="card-image" :src="`/assets/content/images/${image.fname}`" />
      <img
        v-tip="{text: 'This process is expected to expand.', icon: 'improve'}"
        class="process-trend" src="/assets/placeholders/improve.svg">
      <img
        v-tip="{text: 'This process uses coal.', icon: 'feedstock'}"
        class="process-feedstock" :src="assets.icons.feedstock">
    </figure>
    <div class="card-actions">
      <slot name="actions"></slot>
    </div>
    <div class="process-intensity">
      <IntensityIcon
        v-tip="{text: 'Energy: It flows through everything.', icon: 'energy'}"
        resource="energy" :intensity="2" />
      <IntensityIcon
        v-tip="{text: 'Labor: Together with nature, the source of all things.', icon: 'labor'}"
        resource="labor" :intensity="2" />
      <IntensityIcon
        v-tip="{text: 'Water: The giver of life.', icon: 'water'}"
        resource="water" :intensity="2" />
      <IntensityIcon
        v-tip="{text: 'Biodiversity: The co-inhabitants of the planet.', icon: 'biodiversity'}"
        resource="biodiversity" :intensity="2" />
      <IntensityIcon
        v-tip="{text: 'Land: The foo bar.', icon: 'land'}"
        resource="land" :intensity="1" />
      <IntensityIcon
        v-tip="{text: 'Emissions: The foo bar', icon: 'emissions'}"
        resource="emissions" :intensity="3" />
    </div>
    <div class="process-opposers">
      <div>Nay</div>
      <img src="/assets/characters/The Authoritarian.png">
      <img src="/assets/characters/The Economist.png">
      <img src="/assets/characters/The Technocrat.png">
    </div>
    <div class="process-supporters">
      <div>Yea</div>
      <img src="/assets/characters/The Scientist.png">
      <img src="/assets/characters/The Populist.png">
      <img src="/assets/characters/The Ecologist.png">
    </div>
  </template>
  <template v-slot:back>
    <p>{{description}}</p>
    <p>This process currently produces {{produced.amount}}{{consts.icons[output]}} and {{produced.emissions}}{{consts.icons['emissions']}} per year.</p>
    <div class="card-image-attribution">
      Image: {{image.attribution}}
    </div>
  </template>
  <template v-slot:footer>
    <div>GOSPLANT</div>
  </template>
</Card>
</template>

<script>
import Card from './Card.vue';
import state from '/src/state';
import consts from '/src/consts';
import display from 'lib/display';
import IntensityIcon from './IntensityIcon.vue';

export default {
  props: ['process'],
  components: {
    Card,
    IntensityIcon,
  },
  data() {
    return {
      ...this.process,
      ...state.processes[this.process.id],
      output: consts.outputs.keys[this.process.output],
    };
  },
  computed: {
    produced() {
      let baseAmount = state.gameState.produced_by_process[this.id];
      let amount = display.output(baseAmount, this.output);
      amount = amount > 0 ? Math.max(amount, 1) : amount;

      let emissions = display.gtco2eq(this.byproducts, baseAmount);
      emissions = emissions > 0 ? Math.max(emissions, 1) : emissions;
      return {
        emissions,
        amount
      };
    }
  }
}
</script>

<style>
.process-intensity {
  display: flex;
  justify-content: space-evenly;
  margin: 0.5em 0;
}
.process-supporters,
.process-opposers {
  position: absolute;
  top: 50%;
  display: flex;
  flex-direction: column;
  text-decoration: underline;
}
.process-opposers {
  left: 0;
  transform: translate(-50%, -50%);
  color: #EF3838;
}
.process-supporters {
  right: 0;
  transform: translate(50%, -50%);
  color: #43CC70;
}
.process-supporters img,
.process-opposers img {
  width: 32px;
  margin: 0.25em 0;
}

.process-trend,
.process-feedstock {
  width: 24px;
  background: #222;
  border-radius: 10em;
  position: absolute;
  top: 0.5em;
  padding: 0.35em 0.2em;
  border: 1px solid #888;
}
.process-trend {
  right: 0.5em;
}
.process-feedstock {
  left: 0.5em;
  padding: 0.2em 0.2em;
}
</style>
