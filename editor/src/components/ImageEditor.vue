<template>
<div class="overlay" @click="tryClose" ref="overlay">
  <div class="image-editor">
    <div class="image-previews">
      <div>
        <div class="image-preview-label">Original</div>
        <img class="image-preview" :src="`/image/${localData.original}`" ref="input" />
        <button class="image-preview-upload" @click="triggerUpload">Upload</button>
      </div>
      <div>
        <div class="image-preview-label">Modified</div>
        <img class="image-preview" :src="`/image/${localData.image}`" ref="output" />
        <button class="image-preview-save" v-if="newModified" @click="saveModified">Save Modified</button>
      </div>
    </div>
    <input type="text" v-model="localData.attribution" placeholder="Attribution credit" @blur="update" />
    <div class="palettes">
      <div class="saved-palette" v-for="palette in palettes">
        <div @click="() => loadPalette(palette)" class="swatch" v-for="color in palette.colors" :style="{background: color}"></div>
      </div>
    </div>
    <div class="palette">
      <div class="color-select" v-for="color, i in colors">
        <input type="color" v-model=colors[i] />
        <div class="color-delete" @click="() => deleteColor(i)">X</div>
      </div>
      <button @click="addColor">+ Color</button>
      <button @click="savePalette">Save Palette</button>
    </div>
    <button class="dither-button" @click="applyDither">Apply Dither</button>
    <canvas id="palette" ref="palette" />
    <input class="image-upload-input" type="file" ref="uploadInput" @change="uploadImage" />
  </div>
</div>
</template>

<script>
import api from '../api';
import uuid from '../uuid';
import state from '../state';
import * as Magick from "wasm-imagemagick";

async function getImage(src) {
  const img = await fetch(src)
  return new Uint8Array(await img.arrayBuffer())
}

function uploadImage(img) {
  let formData = new FormData();
  formData.append('image', img, img.name);

  return fetch('/image', {
    headers: {
      'Accept': 'application/json',
    },
    method: 'POST',
    body: formData
  })
    .then((res) => {
      if (!res.ok) {
        throw new Error(`Response ${res.status}`);
      }
      return res.json();
    });

}

export default {
  props: ['image', 'dimensions'],
  data() {
    return {
      state,
      newModified: false,
      newModifiedBlob: null,
      colors: ['#426499'],
      localData: Object.assign({}, this.image)
    };
  },
  computed: {
    palettes() {
      return Object.values(this.state.items)
        .filter((i) => i._type == 'Palette' && !i.deleted);
    }
  },
  methods: {
    update() {
      this.$emit('update', this.localData);
    },
    addColor() {
      this.colors.push('#426499');
    },
    deleteColor(i) {
      this.colors.splice(i, 1);
    },
    savePalette() {
      api.update({
        id: uuid(),
        _created: Date.now(),
        _type: 'Palette',
        colors: this.colors
      });
    },
    loadPalette(palette) {
      this.colors = [...palette.colors];
    },
    async genPalette(colors) {
      const ctx = this.$refs.palette.getContext('2d');
      colors.forEach((color, i) => {
        ctx.fillStyle = color;
        ctx.fillRect(i, 0, 1, 1);
      });
      return new Promise((resolve, reject) => {
        this.$refs.palette.toBlob((blob) => {
          blob.arrayBuffer().then((buff) => {
            resolve(new Uint8Array(buff));
          });
        });
      });
    },
    async applyDither() {
      const inputFiles = [{
        name: 'input.jpg',
        content: await getImage(`/image/${this.localData.original}`),
      }, {
        name: 'palette.png',
        content: await this.genPalette(this.colors),
      }];

      const command = [
        'convert', 'input.jpg',

        // Scale down to exaggerate dithering effect
        '-resize', `${this.dimensions}^`,

        // Crop for consistent sizing
        '-gravity', 'Center',
        '-extent', `${this.dimensions}`,

        // Apply dither
        '-dither', 'Riemersma',
        '-remap', 'palette.png',

        // Scale back up with nearest neighbor interpolation
        '-interpolate', 'Nearest',
        '-filter', 'point',
        '-resize', '150%',
        'output.png'];

      const processedFiles = await Magick.Call(inputFiles, command);
      const blob = processedFiles[0]['blob'];
      this.$refs.output.src = URL.createObjectURL(blob);

      this.newModified = true;
      this.newModifiedBlob = blob.slice(0, blob.size, 'image/png');
      this.newModifiedBlob.name = 'modified.png';
    },
    tryClose(ev) {
      if (ev.target == this.$refs.overlay) {
        this.$emit('close');
      }
    },
    triggerUpload() {
      this.$refs.uploadInput.click();
    },
    saveModified() {
      uploadImage(this.newModifiedBlob).then(({filename}) => {
        this.localData.image = filename;
        this.update();
      });
    },
    uploadImage() {
      let img = this.$refs.uploadInput.files[0];
      if (!img) return;
      uploadImage(img).then(({filename}) => {
        this.localData.image = filename;
        this.localData.original = filename;
        this.update();
      });
    },
  }
};
</script>

<style>
.overlay {
  position: fixed;
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
  z-index: 100;
  background: rgba(0,0,0,0.9);
  display: flex;
  align-items: center;
}
.image-editor {
  background: #18181A;
  border-radius: 0.2em;
  max-width: 800px;
  margin: 0 auto;
  border: 1px solid #fff;
}
#palette {
  display: none;
}
.palette {
  padding: 1em;
  display: flex;
  flex-wrap: wrap;
}
.palette input {
  width: 48px;
	padding: 0;
  border: 1px solid #D0D0D7;
  vertical-align: middle;
  border-radius: 2px 0 0 2px;
}
.palette button {
  margin-right: 0.2em;
}
.color-select {
  display: flex;
  margin-right: 0.2em;
}
.color-delete {
	background: #D0D0D7;
	border-radius: 0 0.2em 0.2em 0;
	padding: 0.1em;
	font-size: 0.8em;
	line-height: 1.5;
}
.image-previews {
  display: flex;
}
.image-previews > div {
  width: 50%;
  min-height: 240px;
  position: relative;
}
.image-previews .image-preview {
  height: 100%;
}
.dither-button {
	width: 100%;
	border-radius: 0 0 0.2em 0.2em;
	border: 1px solid #18181A;
  padding: 0.5em;
}
.color-delete {
  cursor: pointer;
}
.saved-palette {
  cursor: pointer;
  padding: 0 2px;
  display: inline-block;
  margin-right: 0.25em;
}
.saved-palette:hover {
  background: #E5CF2B;
}
.swatch {
  width: 12px;
  height: 12px;
  display: inline-block;
}
.image-preview-label {
  position: absolute;
  top: 0.5em;
  left: 50%;
  transform: translate(-50%, 0);
  background: rgba(0,0,0,0.7);
  color: #fff;
  padding: 0 0.2em;
  border-radius: 0.2em;
  font-size: 0.7em;
}
.image-preview-save,
.image-preview-upload {
  font-size: 0.7em;
  position: absolute;
  bottom: 0.5em;
  right: 0.5em;
  padding: 0 0.1em;
}
.image-preview-upload {
  right: auto;
  left: 0.5em;
}
.image-upload-input {
  display: none;
}
</style>
