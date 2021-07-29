<template>
<Card>
  {{project.name}}
  <div class="details">
    <b>Construction</b>
    <div>{{project.construction.years}} years</div>
    <div>
      <span v-for="(v, k) in project.construction.resources">
        <b>{{k}}</b>:{{v}}
      </span>
    </div>
  </div>
  <div class="details">
    <b>Operation</b>
    <div>
      <span v-for="(v, k) in project.operation.resources">
        <b>{{k}}</b>:{{v}}/year
      </span>
    </div>
  </div>
  <div class="actions">
    <slot name="actions"></slot>
  </div>
</Card>
</template>

<script>
import Card from './Card.vue';
export default {
  props: ['project'],
  components: {
    Card
  },
}
</script>
