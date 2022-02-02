<template>
<div class="pie-chart" ref="stage"></div>
</template>

<script>
import PieChart from 'lib/pie';

export default {
  props: ['dataset', 'colors'],
  mounted() {
    this.chart = new PieChart(this.$refs.stage);
    this.render();
  },
  watch: {
    dataset() {
      this.render();
    }
  },
  methods: {
    render() {
      this.chart.reset();
      this.chart.render(this.dataset, this.colors);
    }
  }
}
</script>

<style>
.pie-chart {
  height: 200px;
}
</style>
