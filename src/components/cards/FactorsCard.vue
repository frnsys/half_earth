<template>
<Card class="factors" :noBack="true">
  <template v-slot:body>
    <FactorsList :factors="factors" />
  </template>
  <template v-slot:back>
  </template>
</Card>

<div class="factors-note">
Only direct impacts are shown.
</div>
</template>

<script>
import Card from './Card.vue';
import FactorsList from './FactorsList.vue';

export default {
  props: ['factors'],
  components: {
    Card,
    FactorsList,
  },
}
</script>

<style>
.card.factors .card--body {
  height: 410px;
  overflow-y: scroll;
  scrollbar-width: thin;
}
.card.factors footer img {
  display: none;
}
.card.factors header {
  position: sticky;
  top: 0;
  background: #333;
}
.card.factors .card--body {
  justify-content: space-between;
}

.factors-note {
  position: absolute;
  bottom: 2em;
  left: 50%;
  transform: translate(-50%, 0);
  background: #222;
  color: #fff;
  padding: 0.5em 1em;
  text-align: center;
  border-radius: 0.2em;
  font-size: 0.8em;
}
</style>
