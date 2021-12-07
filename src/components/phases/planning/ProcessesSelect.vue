<template>
<div class="plan-change-select">
  <header>
    <div>Production Processes</div>
    <div @click="$emit('close')">Close</div>
  </header>
  <div class="planning--page" v-if="page == null">
    <div class="planning--menu">
      <button @click="page = 'Electricity'">
        <img :src="icons.electricity" />
        <div>Electricity</div>
      </button>
      <button @click="page = 'Fuel'">
        <img :src="icons.fuel" />
        <div>Fuel</div>
      </button>
      <button @click="page = 'PlantCalories'">
        <img :src="icons.plant_calories" />
        <div>Plant Calories</div>
      </button>
      <button @click="page = 'AnimalCalories'">
        <img :src="icons.animal_calories" />
        <div>Animal Calories</div>
      </button>
    </div>
  </div>
  <Processes v-else :output="page" />
</div>
</template>

<script>
import state from '/src/state';
import Processes from './Processes.vue';

export default {
  components: {
    Processes
  },
  data() {
    return {
      state,
      page: null,
    }
  },
}
</script>
