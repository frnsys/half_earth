<template>
<div id="globe">
  <div v-if="!ready" class="globe-loading loading-text">{{t('Loading')}}</div>
</div>
</template>

<script>
// Dynamically import (code split) the Globe module.
// It's the only one that uses threejs, which adds like ~300-400KB
// to the bundle size.
const getGlobe = () => import('../earth/globe');

let globe = null;

export default {
  props: [
    'onReady',
    'onClick',
  ],
  data() {
    return {
      ready: false
    }
  },
  mounted() {
    getGlobe().then(({default: Globe}) => {
      if (globe == null) {
        globe = new Globe(this.$el);
        globe.render();
        globe.init();
        globe.onReady(() => {
          this.ready = true;
          if (this.onReady) {
            this.onReady(globe);
          }
        });
      } else {
        globe.setEl(this.$el);
        globe.active = true;
        globe.render();
        this.ready = true;
        if (this.onReady) {
          this.onReady(globe);
        }
      }
      globe._onClick = [];
      globe.onClick((intersects) => {
        if (this.onClick) {
          this.onClick(intersects);
        }
      });

      globe.scene.resize();
      globe.resetCamera();
      this.globe = globe;
    });
  },
  beforeUnmount() {
    if (this.globe) {
      this.globe.active = false;
    }
  },
}
</script>

<style>
#globe {
  flex: 1;
  position: relative;
}

.globe-loading {
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  top: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
