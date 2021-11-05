<template>
<div class="minicard" @click="expand">
  <slot name="body"></slot>
</div>
<div class="minicard--expanded" v-if="expanded" @click="collapse" ref="overlay">
  <slot name="expanded"></slot>
</div>
</template>

<script>
export default {
  data() {
    return {
      expanded: false,
    }
  },
  methods: {
    expand() {
      this.expanded = true;
    },
    collapse(ev) {
      if (ev.target == this.$refs.overlay) {
        this.expanded = false;
      }
    }
  }
}
</script>

<style>
.minicard {
  border-radius: 0.3em;
  background: #222;
  padding: 0.5em;
  width: 80px;
  height: 120px;
  position: relative;
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  text-align: center;
}
.minicard--expanded {
  position: fixed;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  background: rgba(0,0,0,0.8);
  display: flex;
  justify-content: space-around;
  align-items: center;
  z-index: 100;
}
</style>
