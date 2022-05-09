<template>
<div class="overlay image-selector" @click="tryClose" ref="overlay">
  <template v-for="img in images">
    <div>
      <img :src="`/image/${img.image}`" @click="select(img)" />
      <div style="color:#fff;">{{img.name}}</div>
    </div>
  </template>
</div>
</template>

<script>
import state from '/src/state';
import allImages from '../../all_images.json';

export default {
  props: ['image', 'dimensions'],
  data() {
    return {
      state,
    };
  },
  computed: {
    images() {
      return Object.values(this.state.items)
        /* .filter((i) => !i.deleted && i.image) */
        .filter((i) => i.image)
        .map((i) => {
          return {
            name: i.name,
            ...i.image
          }
        }).concat(allImages);
    }
  },
  methods: {
    tryClose(ev) {
      if (ev.target == this.$refs.overlay) {
        this.$emit('close');
      }
    },
    select(img) {
      this.$emit('selected', img);
      this.$emit('close');
    }
  }
}
</script>

<style>
.image-selector {
  flex-wrap: wrap;
  justify-content: space-around;
  overflow-y: scroll;
}
.image-selector img {
  width: 180px;
  cursor: pointer;
}
</style>
