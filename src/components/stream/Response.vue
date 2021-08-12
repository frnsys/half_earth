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

  position: relative;
  border-radius: 0.5em;
  width: 200px;
  height: 250px;
  background: #F17F5A;
  color: #fff;
  z-index: 1;
  user-select: none;
  margin: 0 auto;
}
</style>
