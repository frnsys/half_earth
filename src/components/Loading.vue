<template>
<div id="loading">
  <div>
    <img src="/assets/gosplant.svg" />
    <div id="loading-text">Booting Up</div>
  </div>
</div>
</template>

<script>
import icons from 'components/icons';

const preload = [
  '/assets/stamp.svg',
  '/assets/backgrounds/menu.jpg',
  '/assets/backgrounds/dashboard.jpg',
  '/assets/backgrounds/parliament.jpg',
  '/assets/backgrounds/plan.jpg',
  '/assets/backgrounds/regions.jpg',
  '/assets/icons/close.svg',
  '/assets/gosplant.svg',
  '/assets/clock.png',
  '/assets/motto.png',
];
Object.values(icons).forEach((icon) => preload.push(icon));

export default {
  data() {
    return {
      loaded: 0,
    }
  },
  mounted() {
    preload.forEach((src) => {
      let img = new Image();
      img.onload = () => {
        this.loaded++;
        if (this.loaded >= preload.length) {
          this.$emit('loaded');
        }
      };
      img.src = src;
    });
  }
}
</script>

<style>
#loading {
  color: #fff;
  position: fixed;
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
  display: flex;
  align-items: center;
  text-align: center;
}

#loading > div {
  width: 100%;
}

#loading-text {
  font-size: 0.8em;
}

/* https://stackoverflow.com/a/28074607 */
#loading-text:after {
  overflow: hidden;
  display: inline-block;
  vertical-align: bottom;
  animation: ellipsis steps(4,end) 2000ms infinite;
  content: "\2026";
  width: 0px;
}
@keyframes ellipsis {
  to {
    width: 20px;
  }
}
</style>
