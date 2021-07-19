<template>
<div id="globe"></div>
</template>

<script>
import Globe from '../earth/globe';
export default {
    mounted() {
        const globe = new Globe(this.$el);
        globe.render();
        globe.init();
    }
}
</script>
