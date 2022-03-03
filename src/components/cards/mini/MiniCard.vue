<template>
<div class="minicard" :style="style" :class="class" @click="expand">
  <slot name="body"></slot>
</div>
<transition name="opacityfade">
<div class="minicard--expanded" v-if="expanded" @click="collapse" ref="overlay">
  <transition-group appear name="appear-bounceup">
    <slot name="expanded"></slot>
  </transition-group>
</div>
</transition>
</template>

<script>
export default {
  props: ['style','class'],
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
  border-radius: 0.7em;
  padding: 0.5em;
  width: 90px;
  height: 130px;
  position: relative;
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  text-align: center;
  position: relative;
  overflow: hidden;
  transition: all 200ms ease-out;

  image-rendering: auto;
}
.minicard:hover{
  cursor: pointer;
}

.minicard.project:hover{
  transform: scale(1.05);
  box-shadow: 0px 2px 6px rgba(0,0,0,0.3);
}

.minicard--expanded {
  position: fixed;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;

  background-color: rgba(0,0,0,0.2);
  background-image: url('/assets/backgrounds/screen-door.png');
  background-repeat: repeat;

  display: flex;
  justify-content: space-evenly;
  align-items: center;
  flex-direction: column;
  z-index: 100;
}
.minicard--expanded header {
  color: #fff;
}
.minicard--expanded footer {
  display: flex;
  justify-content: space-evenly;
  width: 100%;
  pointer-events: none;
}
.minicard--expanded .pips--buy {
  background: rgb(42 42 42);
  border: 1px solid #fff;
}
.minicard-background {
  background-size: cover;
  background-position: center center;
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  z-index: 0;
}

.minicard > header img,
.minicard > footer img {
  width: 14px;
}
</style>
