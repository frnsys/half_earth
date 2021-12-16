<template>
<div class="planning--page">
  <Cards>
    <ProjectCard v-for="i in projectOrder"
      :project="projects[i]"
      @change="$emit('change')" />
  </Cards>
  <footer>
    <div class="pips">
      {{availablePoints}}<img class="pip" :src="icons.research">
    </div>
    <div class="pips pips--buy" @click="buyPoint">
      {{nextPointCost}}<img class="pip" :src="icons.political_capital"> ⮕ <img class="pip" :src="icons.research">
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
