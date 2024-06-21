<template>
<div class="miniprocess--wrapper">
<MiniCard :class="label" class="test">
  <template v-slot:body>
    <div class="minicard-background" :style="{backgroundImage: `url(/assets/content/images/${image.fname})`}" />
    <div :style="{zIndex: 1}">
      <img class="minicard-process-icon" :src="icons[icon]" />
    </div>

  </template>
  <template v-slot:expanded>
    <ProcessCard :process="process" />
  </template>
</MiniCard>
<small class="process--label">{{t(label)}}</small>
</div>
</template>

<script>
import MiniCard from './MiniCard.vue';
import ProcessCard from '../ProcessCard.vue';
import PROCESSES from 'content/processes.json';

export default {
  props: ['process'],
  data() {
    return {
      ...PROCESSES[this.process.id],
    }
  },
  components: {
    MiniCard,
    ProcessCard,
  },
  computed: {
    icon() {
      return this.process.output.toLowerCase();
    },
    label(){
      if(this.process.output.toLowerCase() == 'electricity'){
        return 'electricity'
      }
      if(this.process.output.toLowerCase() == 'fuel'){
        return 'fuel'
      }
      if(this.process.output.toLowerCase() == 'plantcalories'){
        return 'crops'
      }
      if(this.process.output.toLowerCase() == 'animalcalories'){
        return 'livestock'
      }
    }
  }
}
</script>

<style scoped>

.miniprocess--wrapper{
  position: relative;
  width: 90px;
}

.process--label {
  color: rgba(0,0,0,0.4);
  text-transform: uppercase;
  font-size: 0.6em;
  margin-top: 0.5em;
  letter-spacing: 0.01em;
  font-weight: bold;
  font-family: 'Inter', sans-serif;
  width: 100%;
  text-align: center;
  left: 0;
}
</style>
