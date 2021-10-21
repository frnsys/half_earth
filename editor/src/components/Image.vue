<template>
<div class="image-form">
  <img class="image-preview" v-if="localData.image" :src="`/image/${localData.image}`"/>
  <div class="image-attribution-preview">{{localData.attribution}}</div>
  <button @click="editing = true">✎</button>
  <ImageEditor v-if="editing" :image="localData" :dimensions="dimensions" @update="update($event)" @close="editing = false" />
</div>
</template>

<script>
import ImageEditor from './ImageEditor.vue';

export default {
  props: ['image', 'dimensions'],
  components: {
    ImageEditor
  },
  data() {
    return {
      editing: false,
      localData: Object.assign({}, this.image)
    }
  },
  methods: {
    update(data) {
      this.localData = data;
      this.$emit('update', data);
    }
  }
};
</script>

<style>
.image-form {
  margin: 0.5em 0 0 0;
  border-radius: 0.2em;
  min-height: 160px;
  position: relative;
  background: #eee;
}
.image-form > button {
  position: absolute;
  bottom: 0.5em;
  right: 0.5em;
}
.image-preview {
  max-width: 100%;
  max-height: 80vh;
  display: block;
  margin: 0 auto;
  border: 1px solid #222;
  border-radius: 0.2em;
}
.image-attribution-preview {
  position: absolute;
  left: 0.5em;
  bottom: 0.5em;
  font-size: 0.7em;
  background: rgba(0,0,0,0.7);
  color: #fff;
  padding: 0 0.2em;
  border-radius: 0.2em;
}
</style>
