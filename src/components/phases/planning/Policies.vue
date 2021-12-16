<template>
<div class="planning--page">
  <Cards>
    <ProjectCard v-for="i in projectOrder"
      :project="projects[i]"
      @change="$emit('change')" />
  </Cards>
  <footer>
    <div class="pips">
      {{availablePoints}}<img class="pip" :src="icons.political_capital">
    </div>
  </footer>
</div>
</template>

<script>
import ProjectMixin from './ProjectMixin';

export default {
  created() {
    this.type = 'Policy';
  },
  mixins: [ProjectMixin],
}
</script>
