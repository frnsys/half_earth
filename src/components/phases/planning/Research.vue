<template>
<div class="planning--page">
  <Cards>
    <ProjectCard v-for="p in projects" :project="p" />
  </Cards>
  <footer v-if="p !== 'Policy'">
    <div class="pips">
      {{availablePoints}}<img class="pip" src="/assets/icons/pips/research.png">
    </div>
    <div class="pips pips--buy" @click="buyPoint">
      {{nextPointCost}}<img class="pip" src="/assets/icons/pips/political_capital.png"> ⮕ <img class="pip" src="/assets/icons/pips/research.png">
    </div>
  </footer>
</div>
</template>

<script>
import ProjectMixin from './ProjectMixin';

export default {
  created() {
    this.type = 'Research';
  },
  mixins: [ProjectMixin],
}
</script>
