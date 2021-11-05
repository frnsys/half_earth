<template>
<div class="planning--page">
  <header>
    <img class="back" @click="$emit('close')" src="/assets/icons/back.svg">
    <div class="pips">
      {{availablePoints}} x <img class="pip" src="/assets/icons/pips/political_capital.png">
    </div>
  </header>
  <Cards>
    <ProjectCard v-for="p in projects" :project="p" />
  </Cards>
</div>
</template>

<script>
import ProjectMixin from './ProjectMixin';

export default {
  mixins: [ProjectMixin('Policy')],
}
</script>
