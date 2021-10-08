<template>
<div class="image-form">
  <img class="image-preview" v-if="image" :src="`/image/${image}`"/>
  <div>
    <label>Upload Image</label>
    <input type="file" ref="input" @change="uploadImage" />
  </div>
</div>
</template>

<script>
export default {
  props: ['image'],
  methods: {
    uploadImage() {
      let img = this.$refs.input.files[0];
      if (!img) return;

      let formData = new FormData();
      formData.append('image', img);

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
        })
        .then(({filename}) => {
          this.$emit('update', filename);
        });
    },
  },
};
</script>

<style>
.image-form {
	background: #222;
	color: #fff;
  margin: 0.5em 0 0 0;
  border-radius: 0.2em;
}
.image-preview {
  max-width: 100%;
  max-height: 80vh;
  display: block;
  margin: 0 auto;
  border: 1px solid #222;
}
.image-form > div {
	padding: 0 0.5em 0.5em 0.5em;
}
</style>
