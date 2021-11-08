<template>
<div class="planning--page">
  <Cards>
    <ProjectCard v-for="p in projects" :project="p" />
  </Cards>
  <footer>
    <div class="pips">
      {{availablePoints}}<img class="pip" src="/assets/icons/pips/political_capital.png">
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
