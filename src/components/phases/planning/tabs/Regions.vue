<template>
<div class="planning--page">
  <Globe id="regions-globe" :onReady="onGlobeReady" :onClick="onGlobeClick" />
  <div v-for="region in regions">
    <RegionItem v-if="region.id == selectedRegion" :region="region" />
  </div>
  <div class="regions-browse" v-if="selectedRegion !== null">
    <div @click="prevRegion">Prev</div>
    <div @click="nextRegion">Next</div>
  </div>
</div>
</template>

<script>
import state from '/src/state';
import Globe from 'components/Globe.vue'
import RegionItem from '../RegionItem.vue';
import regionsToTiles from '/assets/surface/regions_to_tiles.json';
import tilesToRegions from '/assets/surface/tiles_to_regions.json';

export default {
  components: {
    Globe,
    RegionItem
  },
  data() {
    return {
      selectedRegion: null,
      regions: state.gameState.world.regions
    }
  },
  methods: {
    onGlobeReady(globe) {
      globe.clear();
      globe.rotate = false;
      globe.scene.camera.zoom = 0.15;
      globe.scene.camera.updateProjectionMatrix();
      globe.clouds.visible = false;
      this.globe = globe;
    },
    nextRegion() {
      this.selectedRegion++;
      if (this.selectedRegion >= state.gameState.world.regions.length) {
        this.selectedRegion = 0;
      }
      this.centerOnRegion(this.selectedRegion);
    },
    prevRegion() {
      this.selectedRegion--
      if (this.selectedRegion < 0) {
        this.selectedRegion = state.gameState.world.regions.length - 1;
      }
      this.centerOnRegion(this.selectedRegion);
    },
    centerOnRegion(regionId) {
      // Reset highlights
      Object.keys(tilesToRegions).forEach((idx) => {
        this.globe.hexsphere.unhighlightIdx(idx);
      });

      let tiles = this.regionTiles(regionId);
      this.globe.hexsphere.centerOnIndex(tiles[0]);

      // Highlight region
      tiles.forEach((idx) => {
        this.globe.hexsphere.highlightIdx(idx);
      });
    },
    regionTiles(regionId) {
      let name = state.gameState.world.regions[regionId].name;
      let tiles = regionsToTiles[name];
      return tiles['inland'].concat(tiles['coasts']);
    },
    onGlobeClick(intersects) {
      if (intersects.length > 0) {
        let obj = intersects[0].object;
        let regionId = tilesToRegions[obj.userData.idx];
        if (regionId !== undefined) {
          this.selectedRegion = regionId;
          this.centerOnRegion(regionId);
        }
      }
    }
  }
}
</script>

<style>
#regions-globe {
  height: 40vh;
  width: 100%;
}
#regions-globe canvas {
  max-height: 100%;
}
.regions-browse {
  display: flex;
  justify-content: space-between;
}
</style>
