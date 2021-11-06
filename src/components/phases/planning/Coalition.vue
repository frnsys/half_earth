<template>
<div class="planning--page">
  <header>
    <img class="back" @click="$emit('close')" src="/assets/icons/back.svg">
  </header>
  <Cards>
    <NPCCard v-for="npc in npcs" :npc="npc" />
  </Cards>
</div>
</template>

<script>
import state from '/src/state';
import Cards from './Cards.vue';
import NPCCard from 'components/cards/NPCCard.vue';

export default {
  components: {
    Cards,
    NPCCard
  },
  data() {
    return {
      npcs: state.gameState.npcs,
    }
  },
}
</script>

<style>
</style>
