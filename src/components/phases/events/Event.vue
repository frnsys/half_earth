<template>
<div class="event">
  <div class="event--body" :style="{backgroundImage: effectImageUrl }">
    <HelpTip :text="factorTip" x="55%" y="-18px" />
    <div class="arc">{{event.arc}}</div>
    <div class="event--factors">
      <img class="event--factor" v-for="factor in event.factors" :src="icons[factorIcon(factor)]" v-tip="{icon: factorIcon(factor), text: describeFactor(factor)}"/>
    </div>
    <div class="image-attribution">Image source: {{event.image ? event.image.attribution : ''}}</div>
    <div class="event--name">{{event.name}}</div>
    <div class="event--effects" v-if="hasVisibleEffects">
      <Effects :effects="event.effects" />
    </div>
  </div>
  <Dialogue v-bind="event" :effects="[]" @done="done" />
</div>
</template>

<script>
import state from '/src/state';
import HelpTip from 'components/Help.vue';
import Effects from 'components/Effects.vue';
import Dialogue from 'components/Dialogue.vue';

const FACTOR_DESCS = {
  'warming': 'This event is influenced by the global temperature anomaly.',
  'contentedness': 'This event is influenced by how happy people are.',
  'extinction_rate': 'This event is influenced by biodiversity pressures.',
  'sea_level_rise': 'This event is influenced by the amount of sea level rise.',
  'habitability': 'This event is influenced by the habitability of regions.',
  'IsCCS': 'This event is influenced by how much production involves carbon capture and storage.',
  'CanMeltdown': 'This event is influenced by how much energy production can meltdown.',
  'MakesNuclearWaste': 'This event is influenced by how much energy production produces nuclear waste.',
  'IsLaborIntensive': 'This event is influenced by how production is especially labor-intensive.',
  'IsFossil': 'This event is influenced by how much energy production uses fossil fuels.',
  'UsesPesticides': 'This event is influenced by how much food production uses pesticides.',
  'UsesLivestock': 'This event is influenced by how much food production uses livestock.',
  'IsIntermittent': 'This event is influenced by how much energy production is intermittent.',
  'animal_calories': 'This event is influenced by the demand for animal calories.',
  'plant_calories': 'This event is influenced by the demand for plant calories.',
  'electricity': 'This event is influenced by the demand for electricity.',
  'fuel': 'This event is influenced by the demand for fuel.',
};

const factorTip = 'The factors behind this event.↓';

export default {
  props: ['event'],
  components: {
    Effects,
    Dialogue,
    HelpTip,
  },
  beforeMount() {
    this.factorTip = factorTip;
  },
  beforeUnmount() {
    // Hide tip
    state.help[factorTip] = true;
  },
  methods: {
    done() {
      this.$emit('done');
    },
    factorIcon(factor) {
      if (factor.startsWith('Project')) {
        return 'project';
      } else if (factor.startsWith('Process')) {
        return 'process';
      } else if (factor.startsWith('NPCRelationship')) {
        let id = parseInt(factor.split(':')[2]);
        return state.gameState.npcs[id].name;
      } else {
        return factor;
      }
    },
    describeFactor(factor) {
      if (factor.startsWith('Project')) {
        let id = parseInt(factor.split(':')[1]);
        let name = state.gameState.projects[id].name;
        let label = '';
        if (factor.startsWith('ProjectFinished')) {
          label = 'active';
        } else if (factor.startsWith('ProjectInactive')) {
          label = 'inactive';
        } else if (factor.startsWith('ProjectBuilding')) {
          label = 'being built';
        }
        return `This event can occur if "${name}" is ${label}.`;
      } else if (factor.startsWith('Process')) {
        let id = parseInt(factor.split(':')[1]);
        let name = state.gameState.processes[id].name;
        if (factor.startsWith('ProcessOutput')) {
          return `This event is influenced by the output of ${name}.`;
        } else if (factor.startsWith('ProcessMixShare')) {
          return `This event is influenced by the mix share of ${name}.`;
        }
      } else if (factor.startsWith('NPCRelationship')) {
        let parts = factor.split(':');
        let relType = parts[1]
        let id = parseInt(parts[2]);
        let name = state.gameState.npcs[id].name;
        return `This event can occur if ${name} is ${relType == 'Ally' ? 'an' : 'a'} ${relType}.`
      } else {
        return FACTOR_DESCS[factor] || factor;
      }
    }
  },
  computed:{
    effectImageUrl(){
      if (this.event.image){
        if(this.event.image.fname){
          return 'url(/assets/content/images/' + this.event.image.fname + ')'
        } else {
          return 'url(/assets/missing_content.png)'
        }
      } else {
        return 'url(/assets/missing_content.png)'
      }
    },
    hasVisibleEffects(){
      if(!this.event.effects) return false;

      for (let i = 0; i < this.event.effects.length; i++) {
        const effect = this.event.effects[i];
        if (effect.type != "AddEvent" && effect.type != "TriggerEvent") return true;
      }

      return false;
    }
  }
}
</script>

<style>
.event {
  position: absolute;
  z-index: 10;
  bottom: 0;
  top: 0;
  /* background: rgba(0,0,0,0.75); */

  background-image: url('/assets/backgrounds/screen-door.png');
  background-repeat: repeat;

  padding: 8vh 1em;
  width: 100%;

  image-rendering: pixelated;
}
.event--body {
  color: #fff;
  height: 50vh;
  border-radius: 0.75em;
  border: 1px solid #222;
  background-size: cover !important;
  background-position: center !important;
  background-repeat: no-repeat;
  box-shadow: 2px 2px 6px rgba(0, 0, 0, 0.7);
  position: relative;
  background: #222;

  image-rendering: pixelated;
  font-family: 'Inter', sans-serif;
}

@media only screen and (min-width: 480px) {
  .event--body{
    margin: 0 auto;
    width:481px;
  }
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
}

.event .dialogue {
  position: relative;
  background: none;
}

.event--factors {
  position: absolute;
  right: 0.5em;
  top: 0.85em;
}
.event--factors img {
  image-rendering: auto;
  height: 22px;
  background: #222;
  border-radius: 1.2em;
  padding: 0.2em;
  margin-left: 1px;
}

.event--effects {
  padding: 1em;
  background: #222;
  margin: 1em;
  image-rendering: auto;
}
</style>
