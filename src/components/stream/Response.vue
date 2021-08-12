<template>
<div class="response">
  {{response.name}}
  <div>
    <template v-for="(v, k) in response.costs">
      <Resource :name="k" :value="v" />
    </template>
    <template v-for="(v, k) in response.impacts">
      <Resource :name="k" :value="v" />
    </template>
  </div>
</div>
</template>

<script>
import Resource from '../Resource.vue';

export default {
  props: {
    response: Object,
  },
  components: {
    Resource
  }
}
</script>

<style>
.response {
  text-align: center;
  display: flex;
  align-items: center;
  justify-content: space-around;
  flex-direction: column;
  height: 100%;
}
</style>
