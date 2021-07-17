<template>
<div id="globe"></div>
</template>

<script>
import Globe from '../globe/globe';
export default {
    mounted() {
        const globe = new Globe(this.$el);
        globe.render();
    }
}
</script>
