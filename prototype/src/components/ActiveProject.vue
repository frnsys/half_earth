<template>
<Card :class="{'in-progress': project.yearsLeft > 0}">
  <b>{{project.base.name}}</b>
  <div class="details">
    <div v-if="project.status == PROJECT_STATE.CONSTRUCTING">construction years left:{{project.yearsLeft}}</div>
    <div v-else-if="project.status == PROJECT_STATE.DESTRUCTING">destructing years left:{{project.yearsLeft}}</div>
  </div>

  <div class="details">
    <b>Operation</b>
    <div>
      <span v-for="(v, k) in project.base.operation.resources">
        <b>{{k}}</b>:{{v}}/year
      </span>
    </div>
  </div>
  <div class="details">
    <b>Destruction</b>
    <div>{{project.base.destruction.years}} years</div>
    <span v-for="(v, k) in project.base.destruction.resources">
      <b>{{k}}</b>:{{v}}/year
    </span>
  </div>

  <div class="actions">
    <slot name="actions"></slot>
  </div>
</Card>
</template>

<script>
import Card from './Card.vue';
import {PROJECT_STATE} from '../consts';

export default {
  props: ['project'],
  data() {
    return {
      PROJECT_STATE
    };
  },
  components: {
    Card
  },
}
</script>
