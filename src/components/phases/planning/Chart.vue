<template>
<div class="chart" ref="stage"></div>
</template>

<script>
import Chart from 'lib/chart';

export default {
  props: ['datasets', 'markers', 'ranges'],
  mounted() {
    this.chart = new Chart(this.$refs.stage, this.ranges);
    this.render();
  },
  watch: {
    datasets() {
      this.render();
    }
  },
  methods: {
    render() {
      this.chart.reset();
      this.datasets.forEach((set) => {
        this.chart.drawLine(set.data, set.color);
      });
      this.markers.forEach((marker) => {
        if (marker.text) {
          this.chart.drawLabel(marker.text, marker.point, marker);
        } else if (marker.x) {
          this.chart.drawVLine(marker.x, marker.color);
        } else if (marker.y) {
          this.chart.drawHLine(marker.y, marker.color);
        }
      });
    }
  }
}
</script>

<style>
.chart {
  height: 200px;
}
</style>
