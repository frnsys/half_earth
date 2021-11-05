<template>
<div class="planning--page">
  <header>
    <img class="back" @click="$emit('close')" src="/assets/icons/back.svg">
    <div class="pips">
      {{availablePoints}} x <img class="pip" src="/assets/icons/pips/research.png">
    </div>
    <div class="pips pips--buy" @click="buyPoint">
      {{nextPointCost}} x <img class="pip" src="/assets/icons/pips/political_capital.png"> ⮕ <img class="pip" src="/assets/icons/pips/research.png">
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
  mixins: [ProjectMixin('Research')],
}
</script>
