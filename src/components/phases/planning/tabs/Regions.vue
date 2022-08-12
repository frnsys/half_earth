<template>
<div class="planning--page planning--page--regions">
  <Globe id="regions-globe" class="cell" :onReady="onGlobeReady" :onClick="onGlobeClick" />
  <div class="regions-browse" v-if="selectedRegion !== null">
    <div class="region-change btn" @click="prevRegion"><img :src="icons.arrow_left"></div>
    <div class="region-name cell" ref="regionName">{{t(regions[selectedRegion].name)}}</div>
    <div class="region-change btn" @click="nextRegion"><img :src="icons.arrow_right"></div>
  </div>
  <div class="regions-region" v-for="region in regions">
    <RegionItem v-if="region.id == selectedRegion" :region="region" />
  </div>

</div>
</template>

<script>
import state from '/src/state';
import Globe from 'components/Globe.vue'
import RegionItem from '../RegionItem.vue';
import regionsToTiles from '/assets/surface/regions_to_tiles.json';
import tilesToRegions from '/assets/surface/tiles_to_regions.json';
import {scaleText} from 'lib/util';

export default {
  components: {
    Globe,
    RegionItem
  },
  data() {
    return {
      selectedRegion: 0,
      regions: state.gameState.world.regions
    }
  },
  mounted() {
    this.fitRegionName();
  },
  beforeUnmount() {
    if (this.globe) {
      Object.keys(tilesToRegions).forEach((idx) => {
        this.globe.hexsphere.unhighlightIdx(idx);
      });
    }
  },
  activated() {
    this.fitRegionName();
  },
  watch: {
    selectedRegion() {
      this.$nextTick(() => {
        this.fitRegionName();
      });
    }
  },
  methods: {
    fitRegionName() {
      scaleText(this.$refs.regionName, 18);
    },
    onGlobeReady(globe) {
      globe.clear();
      globe.rotate = false;
      globe.scene.camera.zoom = 0.15;
      globe.scene.camera.updateProjectionMatrix();
      globe.clouds.visible = false;
      this.globe = globe;
      this.centerOnRegion(this.selectedRegion);
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
      if (!this.globe) return;

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
  padding: 0 !important;
  max-width: 480px;
  margin: 0 auto;
}
#regions-globe canvas {
  max-height: 100%;
}
.region-name {
  font-size: 1.3em;
  text-align: center;
  flex: 1;
  margin: 0 0.5em;
  height: 54px;
  display: flex;
  align-items: center;
  justify-content: space-around;
}
.regions-browse {
  display: flex;
  justify-content: space-between;
  margin: 0.5em auto;
  width: 100%;
  max-width: 360px;
}
.region-change {
  background: #B3D2BC;
  /* border-right: 1px solid #1a1a1a;
  border-bottom: 1px solid #1a1a1a;
  border-top: 1px solid #FDF7E2;
  border-left: 1px solid #FDF7E2; */
  border-radius: 0.6em;
  padding: 1em;
  display: flex;
}

.planning--page--regions {
  background: url('/assets/backgrounds/regions.png');
  background-size: cover;
  background-repeat: no-repeat;
  background-position: center center;
  image-rendering: pixelated;
}

.planning--page--regions .cell {
  padding: 0.5em;
  border-radius: 0.25em;
  background: #304436;
  border-left: 1px solid #1a1a1a;
  border-top: 1px solid #1a1a1a;
  border-right: 1px solid #FDF7E2;
  border-bottom: 1px solid #FDF7E2;
  color: #fff;
  position: relative;
}

.regions-region {
  width: 100%;
  max-width: 360px;
  margin: 0 auto;
}
</style>
