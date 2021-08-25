<template>
<div id="globe"></div>
</template>

<script>
// Dynamically import (code split) the Globe module.
// It's the only one that uses threejs, which adds like ~300-400KB
// to the bundle size.
const getGlobe = () => import('../earth/globe');

import emissionsData from '../../assets/hector/rcp45.to_2050.json';

let startYear = emissionsData.startYear;
let gameStartYear = 2025;
let i = gameStartYear - startYear;

export default {
  mounted() {
    getGlobe().then(({default: Globe}) => {
      this.globe = new Globe(this.$el);
      this.globe.render();

      // For testing biome updates
      // const incrementYear = () => {
      //   let curYear = startYear + i;
      //   if (curYear < 2050) {
      //     console.log(`Year: ${curYear}`);
      //     let update = {};
      //     Object.keys(emissionsData.data).forEach((k) => {
      //       update[k] = emissionsData.data[k][i];
      //     });
      //     i++;
      //     this.globe.addEmissionsThenUpdate(update).then(() => {
      //       setTimeout(() => {
      //         incrementYear();
      //       }, 2000);
      //     });
      //   }
      // };
      // this.globe.init().then(() => {
      //   setTimeout(() => {
      //     incrementYear();
      //   }, 2000);
      // });
    });
  }
}
</script>

<style>
#globe {
  flex: 1;
}
</style>
