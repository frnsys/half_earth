<template>
<div>
  <label>
    Byproducts
  </label>
  <div class="kind-quantities">
    <div v-for="k in Object.keys(BYPRODUCTS)">
      <label>{{k}} <span class="units">{{BYPRODUCTS[k]}}</span></label>
      <input type="number" min="0"
        v-model="localData[k]"
        @change="$emit('update', localData)" />
    </div>
  </div>
</div>
</template>

<script>
export default {
  props: ['byproducts'],
  data() {
    return {
      localData: Object.assign({}, this.byproducts)
    };
  }
}
</script>
