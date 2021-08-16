<template>
  <div class="setting">
    <div class="background" ref="background" :style="{backgroundImage:`url('${background}')`}"></div>
  </div>
</template>

<script>
import {Howl, Howler} from 'howler';

export default {
  props: {
    background: String,
    audio: String
  },
  data: () => {
    return {
      sound: null
    };
  },
  mounted() {
    // Requires user interaction to play
    this.sound = new Howl({
      src: [this.audio],
      loop: true,
      volume: 0,
    });
  },
  methods: {
    start() {
      this.sound.play();
      this.sound.fade(0, 1, 5000);
    }
  }
}
</script>

<style scoped>
.setting {
  position: fixed;
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
  z-index: -1;
  background: #000;
}
.background {
  width: 100%;
  height: 100%;
  background-size: cover;
  background-position: center center;
  background-repeat: no-repeat;
  opacity: 1;
  transition: 5s all;
  image-rendering: pixelated;
}
</style>
