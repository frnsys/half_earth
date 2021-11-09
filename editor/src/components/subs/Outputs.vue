<template>
<div class="kind-quantities">
  <div v-for="k in Object.keys(OUTPUTS)">
    <label>{{k}} <span class="units">{{OUTPUTS[k]}}</span></label>
    <input type="number" min="0"
      v-model="localData[k]"
      @change="$emit('update', localData)" />
  </div>
</div>
</template>

<script>
export default {
  props: ['outputs'],
  data() {
    return {
      localData: Object.assign({}, this.outputs)
    };
  }
}
</script>
