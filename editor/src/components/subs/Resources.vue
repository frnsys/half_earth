<template>
<div>
  <label>
    Resources
  </label>
  <div class="kind-quantities">
    <div v-for="k in Object.keys(RESOURCES)">
      <label>{{k}} <span class="units">{{RESOURCES[k]}}</span></label>
      <input type="number" min="0"
        v-model="localData[k]"
        @change="$emit('update', localData)" />
    </div>
  </div>
</div>
</template>

<script>
export default {
  props: ['resources'],
  data() {
    return {
      localData: Object.assign({}, this.resources)
    };
  }
}
</script>
